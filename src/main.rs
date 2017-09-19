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

extern crate cortex_m_rtfm as rtfm;

use stm32f103xx::{SYST, GPIOA, GPIOB};
use hw::*;
use core::fmt::Write;
use rtfm::{app, Threshold, Resource};

mod hal;
mod hw;
mod font;

app! {
    device: stm32f103xx,

    resources: {
        static STEPPER: stepper::Stepper = stepper::Stepper::new();
        static HALL: hall::Hall = hall::Hall::new();
    },

    idle: {
        resources: [TIM1, TIM3, SYST, GPIOA, GPIOB, HALL, STEPPER],
    },

    tasks: {
        TIM1_UP_TIM10: {
            path: step_completed,
            resources: [STEPPER, TIM1, GPIOA]
        },

        TIM2: {
            path: hall_interrupt,
            resources: [HALL, TIM2]
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
    let driver = SingletonDriver.materialize(p.TIM1, p.GPIOA);

    clock::setup(p.RCC, p.SYST, p.FLASH);

    // Initialize hardware
    Led.init(p.GPIOA, p.RCC);
    Screen.init(p.GPIOB, p.RCC);
    Encoder.init(p.TIM3, p.GPIOA, p.RCC);
    Encoder.set_current(p.TIM3, 0); // Start with 1 IPM
    Encoder.set_limit(p.TIM3, MAX_IPM);
    Controls.init(p.GPIOA, p.RCC);
    r.HALL.init(p.TIM2, p.GPIOA, p.RCC);

    passivate(p.GPIOA, p.GPIOB);

    driver.init(p.RCC);
    r.STEPPER.set_acceleration((ACCELERATION * MICROSTEPS) << 8).unwrap();
}

fn estop(syst: &SYST, lcd: &mut Display) -> ! {
    ::delay::ms(syst, 1); // Wait till power is back to normal

    // Immediately disable driver outputs
    ::hw::config::ENABLE.set(unsafe { &(*stm32f103xx::GPIOA.get()) }, 0);

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
        CB: for<'a> FnOnce(&mut stepper::Stepper, &mut driver::BoundDriver) -> T {

    let stepper = &mut r.STEPPER;
    let tim1 = &r.TIM1;
    let gpioa = &r.GPIOA;

    stepper.claim_mut(t, |stepper, t| {
        tim1.claim(t, |tim1, t| {
            gpioa.claim(t, |gpioa, _t| {
                let mut driver = SingletonDriver.materialize(tim1, gpioa);
                cb(stepper, &mut driver)
            })
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

fn init_screen(r: &idle::Resources) {
    let mut lcd = Screen.materialize(r.SYST, r.GPIOB);
    lcd.init(lcd::FunctionLine::Line2, lcd::FunctionDots::Dots5x8);
    lcd.display(lcd::DisplayMode::DisplayOn, lcd::DisplayCursor::CursorOff, lcd::DisplayBlink::BlinkOff);
    font::upload_characters(&mut lcd);
    lcd.entry_mode(lcd::EntryModeDirection::EntryRight, lcd::EntryModeShift::NoShift);
}

fn update_screen(state: &State, r: &idle::Resources) {
    let mut lcd = Screen.materialize(r.SYST, r.GPIOB);
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
    init_screen(&r);

    // Need to wait at least 40ms after Vcc rises to 2.7V
    // STM32 could start much earlier than that
    ::delay::ms(r.SYST, 50);

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
    Encoder.set_current(r.TIM3, state.slow_ipm - 1);
    loop {
        if ::hw::config::ESTOP.get(r.GPIOB) == 0 {
            {
                let mut lcd = Screen.materialize(r.SYST, r.GPIOB);
                estop(r.SYST, &mut lcd);
            }
        }

        let input = r.GPIOA.claim(t, |gpioa, _t| Controls.get(gpioa));
        handle_ipm(&mut state, input, t, &mut r);
        handle_feed(&mut state, input, t, &mut r);
        handle_rpm(&mut state, t, &r);

        update_screen(&state, &r);
    }
}

fn handle_ipm(state: &mut State, input: ControlsState, t: &mut Threshold, r: &mut idle::Resources) {
    let mut ipm = Encoder.current(r.TIM3) + 1; // Encoder is off by one (as it starts from 0)
    match (state.fast, input.fast) {
        (false, true) => {
            // Switch to fast IPM
            state.slow_ipm = ipm;
            ipm = state.fast_ipm;
            Encoder.set_current(r.TIM3, ipm - 1);
            state.fast = true;
        }

        (true, false) => {
            // Switch to slow IPM
            state.fast_ipm = ipm;
            ipm = state.slow_ipm;
            Encoder.set_current(r.TIM3, ipm - 1);
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
    let mut driver = SingletonDriver.materialize(r.TIM1, r.GPIOA);
    let tim1 = &r.TIM1;

    if tim1.sr.read().uif().is_pending() {
        tim1.sr.modify(|_, w| w.uif().clear());
        r.STEPPER.step_completed(&mut driver);
    }
}

fn hall_interrupt(_t: &mut Threshold, r: TIM2::Resources) {
    r.HALL.interrupt(r.TIM2);
}
