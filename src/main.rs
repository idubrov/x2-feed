#![feature(const_fn)]
#![feature(used)]
#![feature(struct_field_attributes)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate stm32f103xx;
extern crate stepgen;
extern crate hd44780;

#[macro_use]
extern crate cortex_m_rtfm as rtfm;

use rtfm::{P0, P4, T0, C4, T4, TMax, Resource};
use stm32f103xx::interrupt::Tim1UpTim10;
use stm32f103xx::{Syst};
use hw::{delay, clock, lcd, led, encoder, driver, stepper, ESTOP};
use core::cell::RefCell;

mod timer;
mod hw;

type Tim4Timer = timer::Timer<stm32f103xx::Tim4>;

static TIMER: Tim4Timer = Tim4Timer::new();
static LCD: lcd::Lcd = lcd::Lcd::new();
static LED: led::Led = led::Led::new();
static ENC: encoder::Encoder = encoder::Encoder::new();
static DRIVER: driver::DriverRef = driver::DriverRef::new();

static STEPPER: Resource<RefCell<stepper::Stepper>, C4> =
    Resource::new(RefCell::new(stepper::Stepper::new()));

tasks!(stm32f103xx, {
step_completed: Task {
        interrupt: Tim1UpTim10,
        priority: P4,
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
    TIM3: Peripheral {
        register_block: Tim3,
        ceiling: C0,
    },
    TIM4: Peripheral {
        register_block: Tim4,
        ceiling: C1,
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
    let tim3 = TIM3.access(priority, threshold);
    let tim4 = TIM4.access(priority, threshold);
    let driver = DRIVER.materialize(&tim1, &gpioa);

    clock::setup(&rcc, &syst, &flash);

    // FIXME: ...
    LED.init(&gpioa, &rcc);

    TIMER.init(&tim4, &rcc);
    LCD.init(&gpiob, &rcc);
    ENC.init(&tim3, &gpioa, &rcc);
    ENC.set_limit(&tim3, 20);
    driver.init(&rcc);
    driver.release();

    let stepper = STEPPER.access(priority, threshold);
    stepper.borrow_mut().set_acceleration((ACCELERATION * MICROSTEPS) << 8);
}

fn estop(syst: &Syst, lcd: &hd44780::HD44780<lcd::LcdHw>) -> ! {
    ::delay::ms(syst, 1); // Wait till power is back to normal

    // Immediately disable driver outputs
    ::hw::ENABLE.set(unsafe { &(*stm32f103xx::GPIOA.get()) }, 0);
    lcd.position(0, 0);
    lcd.print("*E-STOP*");
    lcd.position(0, 1);
    lcd.print("        ");
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
    fast_ipm: u16
}

fn idle(priority: P0, threshold: T0) -> ! {
    let syst = SYST.access(&priority, &threshold);
    // FIXME: ..
    let gpioa = unsafe { &(*stm32f103xx::GPIOA.get()) }; //GPIOA.access(&priority, &threshold);
    let gpiob = GPIOB.access(&priority, &threshold);
    let tim3 = TIM3.access(&priority, &threshold);
    let lcd = LCD.materialize(&syst, &gpiob);
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
    };
    ENC.set_current(&tim3, state.slow_ipm);
    ENC.set_limit(&tim3, MAX_IPM);
    loop {
        if ESTOP.get(&gpiob) == 0 {
            estop(&syst, &lcd);
        }

        lcd.position(0, 0);

        let left = ::hw::LEFT.get(&gpioa) == 1;
        let right = ::hw::RIGHT.get(&gpioa) == 1;
        let fast = ::hw::FAST.get(&gpioa) == 1;

        let mut ipm = ENC.current(&tim3);
        match (state.fast, fast) {
            (false, true) => {
                // Switch to fast IPM
                state.slow_ipm = ipm;
                ipm = state.fast_ipm;
                ENC.set_current(&tim3, ipm);
                state.fast = true;
            }

            (true, false) => {
                // Switch to slow IPM
                state.fast_ipm = ipm;
                ipm = state.slow_ipm;
                ENC.set_current(&tim3, ipm);
                state.fast = false;
            }

            _ => {}
        }

        // Update stepper speed based on current setting
        // FIXME: divide after shift?
        let speed = ((((ipm + 1) << 8) as u32) * PITCH * STEPS_PER_ROTATION * MICROSTEPS) / 60;
        if state.speed != speed {
            stepper_command(&priority, &threshold, |mut s, _| { s.set_speed(speed); });
            state.speed = speed;
        }

        match (state.run_state, left, right) {
            (RunState::Stopped, true, false) => {
                stepper_command(&priority, &threshold, |mut s, d| { s.move_to(&syst, d, -1000000000); });
                state.run_state = RunState::Running;
            }

            (RunState::Stopped, false, true) => {
                stepper_command(&priority, &threshold, |mut s, d| { s.move_to(&syst, d, 1000000000); });
                state.run_state = RunState::Running;
            }

            (RunState::Running, false, false) => {
                stepper_command(&priority, &threshold, |mut s, _| s.stop());
                state.run_state = RunState::Stopping;
            }

            (RunState::Stopping, _, _) => {
                if stepper_command(&priority, &threshold, |mut s, d| s.is_stopped(d)) {
                    state.run_state = RunState::Stopped;
                }
            }

            _ => {}
        }

        lcd.position(0, 1);
        let ipm0 = (ipm + 1) % 10;
        let ipm1 = ((ipm + 1) / 10) % 10;
        lcd.write((ipm1 as u8) + ('0' as u8));
        lcd.write((ipm0 as u8) + ('0' as u8));
        if state.fast {
            lcd.print(" FIPM");
        } else {
            lcd.print(" IPM ");
        }
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