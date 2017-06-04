#![feature(const_fn)]
#![feature(used)]
#![feature(struct_field_attributes)]
#![no_std]

//! Stepper-motor based power feed for X2 mill.
//!
//! The following features are supported:
//!
//! 1. Power feeding in both directions.
//! 1. Two feed modes: "slow" and "fast".
//! 1. Setting feed speed via rotary encoder (both "slow" and "fast").
//! 1. Spindle tachometer via hall sensor.
//! 1. Emergency stop mode: halts stepper motor driver when emergency stop is pressed.
//! 1. LCD screen displays current spindle speed and feed speed.
//!
//! # PCB
//! See PCB (Eagle CAD) in the [pcb/](pcb/) directory.

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate stm32f103xx;
extern crate stepgen;
extern crate hd44780;

#[macro_use]
extern crate cortex_m_rtfm as rtfm;

use rtfm::{P0, P2, P4, C2, C4, T0, T2, T4, TMax, Resource};
use stm32f103xx::interrupt::{Tim2, Tim1UpTim10};
use stm32f103xx::{Syst};
use hw::{delay, clock, lcd, led, encoder, driver, stepper, controls, hall, ESTOP};
use core::cell::RefCell;
use core::fmt::Write;

mod hw;

static LCD: lcd::Lcd = lcd::Lcd::new();
static LED: led::Led = led::Led::new();
static ENC: encoder::Encoder = encoder::Encoder::new();
static DRIVER: driver::DriverRef = driver::DriverRef::new();
static CONTROLS: controls::Controls = controls::Controls::new();

static STEPPER: Resource<RefCell<stepper::Stepper>, C4> =
    Resource::new(RefCell::new(stepper::Stepper::new()));

static HALL: Resource<RefCell<hall::Hall>, C2> =
    Resource::new(RefCell::new(hall::Hall::new()));

tasks!(stm32f103xx, {
    step_completed: Task {
        interrupt: Tim1UpTim10,
        priority: P4,
        enabled: true,
    },
    hall_interrupt: Task {
        interrupt: Tim2,
        priority: P2,
        enabled: true,
    },
});

peripherals!(stm32f103xx, {
    RCC: Peripheral {
        register_block: Rcc,
        ceiling: C0,
    },
    SYST: Peripheral {
        register_block: Syst,
        ceiling: C0,
    },
    FLASH: Peripheral {
        register_block: Flash,
        ceiling: C0,
    },
    TIM1: Peripheral {
        register_block: Tim1,
        ceiling: C4,
    },
    TIM2: Peripheral {
        register_block: Tim2,
        ceiling: C2,
    },
    TIM3: Peripheral {
        register_block: Tim3,
        ceiling: C0,
    },
    GPIOA: Peripheral {
        register_block: Gpioa,
        ceiling: C4,
    },
    GPIOB: Peripheral {
        register_block: Gpiob,
        ceiling: C0,
    },
});

fn init(ref priority: P0, threshold: &TMax) {
    let rcc = RCC.access(priority, threshold);
    let syst = SYST.access(priority, threshold);
    let flash = FLASH.access(priority, threshold);
    let gpioa = GPIOA.access(priority, threshold);
    let gpiob = GPIOB.access(priority, threshold);
    let tim1 = TIM1.access(priority, threshold);
    let tim2 = TIM2.access(priority, threshold);
    let tim3 = TIM3.access(priority, threshold);
    let driver = DRIVER.materialize(&tim1, &gpioa);
    let hall = HALL.access(priority, threshold);

    clock::setup(&rcc, &syst, &flash);

    // Initialize hardware
    LED.init(&gpioa, &rcc);
    LCD.init(&gpiob, &rcc);
    ENC.init(&tim3, &gpioa, &rcc);
    ENC.set_current(&tim3, 0); // Start with 1 IPM
    ENC.set_limit(&tim3, MAX_IPM);
    CONTROLS.init(&gpioa, &rcc);
    hall.borrow_mut().init(&tim2, &gpioa, &rcc);

    driver.init(&rcc);
    driver.release();
    let stepper = STEPPER.access(priority, threshold);
    stepper.borrow_mut().set_acceleration((ACCELERATION * MICROSTEPS) << 8).unwrap();
}

fn estop(syst: &Syst, lcd: &mut hd44780::HD44780<lcd::LcdHw>) -> ! {
    ::delay::ms(syst, 1); // Wait till power is back to normal

    // Immediately disable driver outputs
    ::hw::ENABLE.set(unsafe { &(*stm32f103xx::GPIOA.get()) }, 0);

    lcd.position(0, 0);
    write!(lcd, "*E-STOP*").unwrap();
    lcd.position(0, 1);
    write!(lcd, "        ").unwrap();
    loop {
        cortex_m::asm::nop();
    }
}

fn stepper_command<T, CB>(priority: &P0, threshold: &T0, cb: CB) -> T
    where
        CB: for<'a> FnOnce(core::cell::RefMut<'a, stepper::Stepper>, &driver::Driver) -> T {
    threshold.raise(
        &STEPPER, |threshold| {
            let gpioa = GPIOA.access(priority, threshold);
            let tim1 = TIM1.access(priority, threshold);
            let driver = DRIVER.materialize(&tim1, &gpioa);
            let stepper = STEPPER.access(priority, threshold);
            let result = cb(stepper.borrow_mut(), &driver);
            result
        }
    )
}

#[derive(Clone, Copy)]
enum RunState {
    Stopped,
    Stopping,
    Running
}


const PITCH: u32 = 16;
const MICROSTEPS: u32 = 16;
const MAX_IPM: u16 = 30;
const ACCELERATION: u32 = 1200; // Steps per second per second
const STEPS_PER_ROTATION: u32 = 200;

struct State {
    run_state: RunState,
    fast: bool,
    speed: u32,
    slow_ipm: u16,
    fast_ipm: u16,
    rpm: u32,
}

fn idle(priority: P0, threshold: T0) -> ! {
    let syst = SYST.access(&priority, &threshold);
    let gpiob = GPIOB.access(&priority, &threshold);
    let tim3 = TIM3.access(&priority, &threshold);
    let mut lcd = LCD.materialize(&syst, &gpiob);
    lcd.init();
    lcd.display(hd44780::DisplayMode::DisplayOn, hd44780::DisplayCursor::CursorOff, hd44780::DisplayBlink::BlinkOff);
    lcd.entry_mode(hd44780::EntryModeDirection::EntryRight, hd44780::EntryModeShift::NoShift);

    // Need to wait at least 40ms after Vcc rises to 2.7V
    // STM32 could start much earlier than that
    ::delay::ms(&syst, 50);

    let mut state = State {
        run_state: RunState::Stopped,
        fast: false,
        speed: 0,
        // Offset by 1, as IPM of 0 is not allowed.
        slow_ipm: 10 - 1,
        fast_ipm: 30 - 1,
        rpm: 0,
    };
    ENC.set_current(&tim3, state.slow_ipm - 1);
    loop {
        if ESTOP.get(&gpiob) == 0 {
            estop(&syst, &mut lcd);
        }

        let input = CONTROLS.get();
        let ipm = handle_ipm(&mut state, input, &priority, &threshold);
        handle_feed(&mut state, input, &priority, &threshold);
        handle_rpm(&mut state, &priority, &threshold);

        lcd.position(0, 0);
        let rrpm = (state.rpm + 128) >> 8;
        write!(&mut lcd, " {: >4} RPM", rrpm).unwrap();

        lcd.position(0, 1);
        write!(&mut lcd, "{: >4} {}IPM", (ipm + 1) as u32, if state.fast { 'F' } else { ' ' }).unwrap();
    }
}

fn handle_ipm(state: &mut State, input: controls::State, priority: &P0, threshold: &T0) -> u16 {
    let tim3 = TIM3.access(&priority, &threshold);

    let mut ipm = ENC.current(&tim3) + 1; // Encoder is off by one (as it starts from 0)
    match (state.fast, input.fast) {
        (false, true) => {
            // Switch to fast IPM
            state.slow_ipm = ipm;
            ipm = state.fast_ipm;
            ENC.set_current(&tim3, ipm - 1);
            state.fast = true;
        }

        (true, false) => {
            // Switch to slow IPM
            state.fast_ipm = ipm;
            ipm = state.slow_ipm;
            ENC.set_current(&tim3, ipm - 1);
            state.fast = false;
        }

        _ => {}
    }
    // Update stepper speed based on current setting
    // FIXME: divide after shift?
    let speed = (((ipm << 8) as u32) * PITCH * STEPS_PER_ROTATION * MICROSTEPS) / 60;
    if state.speed != speed {
        stepper_command(&priority, &threshold, |mut s, _| { s.set_speed(speed) }).unwrap();
        state.speed = speed;
    }
    ipm
}

fn handle_feed(state: &mut State, input: controls::State, priority: &P0, threshold: &T0) {
    match (state.run_state, input.left, input.right) {
        (RunState::Stopped, true, false) => {
            // FIXME: ideally, we should have "move left" command instead of using "-infinity" and "+infinity"
            stepper_command(&priority, &threshold, |mut s, d| { s.move_to(d, -1000000000); });
            state.run_state = RunState::Running;
        }

        (RunState::Stopped, false, true) => {
            stepper_command(&priority, &threshold, |mut s, d| { s.move_to(d, 1000000000); });
            state.run_state = RunState::Running;
        }

        (RunState::Running, false, false) => {
            stepper_command(&priority, &threshold, |mut s, _| s.stop());
            state.run_state = RunState::Stopping;
        }

        (RunState::Stopping, _, _) => {
            if stepper_command(&priority, &threshold, |s, d| s.is_stopped(d)) {
                state.run_state = RunState::Stopped;
            }
        }

        _ => {}
    }
}

fn handle_rpm(state: &mut State, priority: &P0, threshold: &T0) {
    let rpm = threshold.raise(
        &HALL, |threshold| {
            let hall = HALL.access(&priority, threshold);
            let rpm = hall.borrow().rpm();
            rpm
        }
    );
    // Only capture if difference is big enough (more than .5%)
    if state.rpm == 0 || rpm * 200 > state.rpm * 201 || rpm * 200 < state.rpm * 199 {
        state.rpm = rpm;
    }
}

fn step_completed(mut _task: Tim1UpTim10, ref priority: P4, ref threshold: T4) {
    let gpioa = GPIOA.access(priority, threshold);
    let tim1 = TIM1.access(priority, threshold);
    let driver = DRIVER.materialize(&tim1, &gpioa);
    let stepper = STEPPER.access(priority, threshold);

    if tim1.sr.read().uif().is_pending() {
        tim1.sr.modify(|_, w| w.uif().clear());
        stepper.borrow_mut().step_completed(&driver);
    }
}

fn hall_interrupt(mut _task: Tim2, ref priority: P2, ref threshold: T2) {
    let tim2 = TIM2.access(priority, threshold);
    let hall = HALL.access(priority, threshold);
    hall.borrow_mut().interrupt(&tim2);
}
