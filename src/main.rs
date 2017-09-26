#![feature(const_fn)]
#![feature(used)]
#![feature(proc_macro)]
#![no_std]
#![cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
#![cfg_attr(feature = "cargo-clippy", allow(eq_op))]
#![deny(warnings)]

//! Stepper-motor based power feed for X2 mill.
//!
//! The following features are supported:
//!
//! 1. Power feeding in both directions.
//! 1. Two feed modes: "slow" and "fast".
//! 1. Setting feed speed via rotary encoder (both "slow" and "fast").
//! 1. Spindle tachometer via hall sensor.
//! 1. Emergency stop mode: halts stepper motor driver when emergency stop is pressed.
//! 1. Screen screen displays current spindle speed and feed speed.
//!
//! # PCB
//! See PCB (Eagle CAD) in the [pcb/](pcb/) directory.

extern crate cortex_m;
extern crate stm32f103xx;
extern crate stepgen;
extern crate lcd;
extern crate stm32_extras;
extern crate cortex_m_rtfm as rtfm;

use stm32f103xx::{GPIOA, GPIOB};
use hal::{StepperDriver, RpmSensor, QuadEncoder};
use hw::config::DRIVER_TICK_FREQUENCY;
use core::fmt::Write;
use rtfm::{app, Threshold, Resource};
use hw::{clock, delay, ControlsState};

type Display = lcd::Display<hw::Screen>;

mod hal;
mod hw;
mod font;
mod stepper;

app! {
    device: stm32f103xx,

    resources: {
        static STEPPER: stepper::Stepper = stepper::Stepper::new(DRIVER_TICK_FREQUENCY);
        static HALL: hw::Hall = hw::Hall::new();
        static DRIVER: hw::Driver = hw::Driver;
        static ENCODER: hw::Encoder = hw::Encoder;
        static LED: hw::Led = hw::Led;
        static CONTROLS: hw::Controls = hw::Controls;
        static SCREEN: hw::Screen = hw::Screen;
    },

    idle: {
        resources: [DRIVER, STEPPER, ENCODER, SYST, GPIOA, GPIOB, HALL, LED, CONTROLS, SCREEN],
    },

    tasks: {
        TIM1_UP_TIM10: {
            path: step_completed,
            priority: 16,
            resources: [DRIVER, STEPPER]
        },

        TIM2: {
            path: hall_interrupt,
            priority: 1,
            resources: [HALL]
        }
    },
}

fn passivate(gpioa: &GPIOA, gpiob: &GPIOB) {
    // Pull down remaining inputs on GPIOA and GPIOB
    // PA12
    gpioa.brr.write(|w| w
        .br12().set_bit());
    gpioa.crh.modify(|_, w| w
        .mode12().input().cnf12().bits(0b10)
    );

    // PB5, PB6, PB7, PB8, PB9
    gpiob.brr.write(|w| w
        .br5().set_bit()
        .br6().set_bit()
        .br7().set_bit()
        .br8().set_bit()
        .br9().set_bit());
    gpiob.crl.modify(|_, w| w
        .mode5().input().cnf5().bits(0b10)
        .mode6().input().cnf6().bits(0b10)
        .mode7().input().cnf7().bits(0b10)
    );

    gpiob.crh.modify(|_, w| w
        .mode8().input().cnf8().bits(0b10)
        .mode9().input().cnf9().bits(0b10)
    );
}

fn init(p: init::Peripherals, r: init::Resources) {
    clock::setup(p.RCC, p.SYST, p.FLASH);

    // Initialize peripherals
    r.DRIVER.init(p.RCC, p.GPIOA);
    r.LED.init(p.GPIOA, p.RCC);
    r.SCREEN.init(p.RCC);
    r.ENCODER.init(p.GPIOA, p.RCC);
    r.ENCODER.set_current(0); // Start with 1 IPM
    r.ENCODER.set_limit(MAX_IPM);

    r.CONTROLS.init(p.RCC);
    r.HALL.init(p.TIM2, p.GPIOA, p.RCC);

    // Disable unused inputs
    passivate(p.GPIOA, p.GPIOB);


    r.STEPPER.set_acceleration((ACCELERATION * MICROSTEPS) << 8).unwrap();

    // LCD device init
    // Need to wait at least 40ms after Vcc rises to 2.7V
    // STM32 could start much earlier than that
    delay::ms(50);

    init_screen();
}

fn init_screen() {
    let mut lcd = Display::new(hw::Screen);

    lcd.init(lcd::FunctionLine::Line2, lcd::FunctionDots::Dots5x8);
    lcd.display(lcd::DisplayMode::DisplayOn, lcd::DisplayCursor::CursorOff, lcd::DisplayBlink::BlinkOff);
    font::upload_characters(&mut lcd);
    lcd.entry_mode(lcd::EntryModeDirection::EntryRight, lcd::EntryModeShift::NoShift);
}

fn estop() -> ! {
    delay::ms(1); // Wait till power is back to normal

    // Immediately disable driver outputs
    ::hw::config::ENABLE.set(unsafe { &(*stm32f103xx::GPIOA.get()) }, 0);

    let mut lcd = Display::new(hw::Screen);
    lcd.position(0, 0);
    write!(lcd, "*E-STOP*").unwrap();
    lcd.position(0, 1);
    write!(lcd, "        ").unwrap();
    loop {
        cortex_m::asm::nop();
    }
}

fn stepper_command<T, CB>(t: &mut Threshold, r: &mut idle::Resources, cb: CB) -> T
    where
        CB: for<'a> FnOnce(&mut stepper::Stepper, &mut StepperDriver) -> T {

    let stepper = &mut r.STEPPER;
    let driver = &mut r.DRIVER;
    stepper.claim_mut(t, |stepper, t| {
        driver.claim_mut(t, |driver, _t| {
            cb(stepper, driver as &mut hw::Driver)
        })
    })
}

#[derive(Clone, Copy)]
enum RunState {
    Stopped,
    Stopping,
    RunningLeft,
    RunningRight
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
    ipm: u16,
    rpm: u32,
}

fn update_screen(state: &State) {
    let mut lcd = Display::new(hw::Screen);

    lcd.position(0, 0);
    let rrpm = (state.rpm + 128) >> 8;
    write!(&mut lcd, "{: >4} RPM", rrpm).unwrap();

    lcd.position(0, 1);
    let s = (state.fast, state.run_state);
    let c = match s {
        (false, RunState::RunningLeft) => font::LEFT,
        (false, RunState::RunningRight) => font::RIGHT,
        (true, RunState::RunningLeft) => font::FAST_LEFT,
        (true, RunState::RunningRight) => font::FAST_RIGHT,
        _ => ' '
    };
    write!(&mut lcd, "{}{: >3} IPM", c, u32::from(state.ipm + 1)).unwrap();
}

fn idle(t: &mut Threshold, mut r: idle::Resources) -> ! {
    let mut state = State {
        run_state: RunState::Stopped,
        fast: false,
        speed: 0,
        // Offset by 1, as IPM of 0 is not allowed.
        slow_ipm: 10 - 1,
        fast_ipm: 30 - 1,
        rpm: 0,
        ipm: 0,
    };
    r.ENCODER.set_current(state.slow_ipm - 1);
    loop {
        if ::hw::config::ESTOP.get(r.GPIOB) == 0 {
            {
                estop();
            }
        }

        let input = r.CONTROLS.get();
        handle_ipm(&mut state, input, t, &mut r);
        handle_feed(&mut state, input, t, &mut r);
        handle_rpm(&mut state, t, &r);

        update_screen(&state);
    }
}

fn handle_ipm(state: &mut State, input: ControlsState, t: &mut Threshold, r: &mut idle::Resources) {
    let mut ipm = r.ENCODER.current() + 1; // Encoder is off by one (as it starts from 0)
    match (state.fast, input.fast) {
        (false, true) => {
            // Switch to fast IPM
            state.slow_ipm = ipm;
            ipm = state.fast_ipm;
            r.ENCODER.set_current(ipm - 1);
            state.fast = true;
        }

        (true, false) => {
            // Switch to slow IPM
            state.fast_ipm = ipm;
            ipm = state.slow_ipm;
            r.ENCODER.set_current(ipm - 1);
            state.fast = false;
        }

        _ => {}
    }
    // Update stepper speed based on current setting
    // Shift by 8 to convert to 24.8 format
    let speed = (u32::from(ipm << 8) * PITCH * STEPS_PER_ROTATION * MICROSTEPS) / 60;
    if state.speed != speed {
        stepper_command(t, r, |s, _| { s.set_speed(speed) }).unwrap();
        state.speed = speed;
    }
    state.ipm = ipm
}

fn handle_feed(state: &mut State, input: ControlsState, t: &mut Threshold, r: &mut idle::Resources) {
    match (state.run_state, input.left, input.right) {
        (RunState::Stopped, true, false) => {
            // Use very low number for moving left
            stepper_command(t, r, |s, d| { s.move_to(d, -1_000_000_000); });
            state.run_state = RunState::RunningLeft;
        }

        (RunState::Stopped, false, true) => {
            // Use very high number for moving right
            stepper_command(t, r, |s, d| { s.move_to(d, 1_000_000_000); });
            state.run_state = RunState::RunningRight;
        }

        (RunState::RunningLeft, false, false) | (RunState::RunningRight, false, false) => {
            stepper_command(t, r, |s, _| s.stop());
            state.run_state = RunState::Stopping;
        }

        (RunState::Stopping, _, _) => {
            if stepper_command(t, r, |s, _| s.is_stopped()) {
                state.run_state = RunState::Stopped;
            }
        }

        _ => {}
    }
}

fn handle_rpm(state: &mut State, t: &mut Threshold, r: &idle::Resources) {
    use rtfm::Resource;

    let rpm = r.HALL.claim(t, |hall, _t| hall.rpm());

    // Only capture if difference is big enough (more than .5%)
    if state.rpm == 0 || rpm * 200 > state.rpm * 201 || rpm * 200 < state.rpm * 199 {
        state.rpm = rpm;
    }
}

fn step_completed(_t: &mut Threshold, r: TIM1_UP_TIM10::Resources) {
    if r.DRIVER.interrupt() {
        let driver: &mut hw::Driver = r.DRIVER;
        r.STEPPER.step_completed(driver)
    }
}

fn hall_interrupt(_t: &mut Threshold, r: TIM2::Resources) {
    r.HALL.interrupt();
}
