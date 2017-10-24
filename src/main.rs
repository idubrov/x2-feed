#![feature(const_fn)]
#![feature(used)]
#![feature(proc_macro)]
#![feature(try_trait)]
#![feature(lang_items)]
#![no_std]
#![cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
#![cfg_attr(feature = "cargo-clippy", allow(eq_op))]
//#![deny(warnings)]

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
extern crate stm32_hal;
extern crate cortex_m_rtfm as rtfm;
extern crate bare_metal;
extern crate eeprom;

use stm32f103xx::{GPIOA, GPIOB};
use hal::*;
use config::*;
use rtfm::{app, Threshold};
use hal::{clock, delay};
use eeprom::EEPROM;
use menu::{MenuItem, MainMenu};
use stm32_hal::gpio::Port;

mod hal;
mod config;
mod font;
mod stepper;
mod menu;
mod estop;
mod settings;

app! {
    device: stm32f103xx,

    resources: {
        static STEPPER: stepper::Stepper = stepper::Stepper::new(DRIVER_TICK_FREQUENCY);
        static HALL: RpmSensorResource = config::hall();
        static DRIVER: StepperDriverResource = config::driver();
        static ENCODER: QuadEncoderResource = config::encoder();
        static LED: LedResource = config::led();
        static CONTROLS: ControlsResource = config::controls();
        static SCREEN: ScreenResource = config::screen();
    },

    idle: {
        resources: [DRIVER, STEPPER, ENCODER, SYST, GPIOA, GPIOB, HALL, LED, CONTROLS, SCREEN, FLASH],
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

    // Enable peripherals
    p.RCC.apb1enr.modify(|_, w| w.tim2en().enabled());
    p.RCC.apb1enr.modify(|_, w| w.tim3en().enabled());
    p.RCC.apb2enr.modify(|_, w| w.tim1en().enabled());
    p.RCC.apb2enr.modify(|_, w| w.iopaen().enabled());
    p.RCC.apb2enr.modify(|_, w| w.iopben().enabled());
    p.RCC.apb2enr.modify(|_, w| w.afioen().enabled());

    // Initialize peripherals
    r.DRIVER.init();
    r.LED.init();
    r.SCREEN.init();
    r.ENCODER.init();

    r.CONTROLS.init();
    r.HALL.init();

    // Disable unused inputs
    passivate(p.GPIOA, p.GPIOB);

    // Initialize EEPROM emulation
    p.FLASH.eeprom().init().unwrap();

    // LCD device init
    // Need to wait at least 40ms after Vcc rises to 2.7V
    // STM32 could start much earlier than that
    delay::ms(50);

    init_screen(&r);
}

fn init_screen(r: &init::Resources) {
    let mut lcd = Display::new(r.SCREEN);

    lcd.init(lcd::FunctionLine::Line2, lcd::FunctionDots::Dots5x8);
    lcd.display(lcd::DisplayMode::DisplayOn, lcd::DisplayCursor::CursorOff, lcd::DisplayBlink::BlinkOff);
    font::upload_characters(&mut lcd);
    lcd.entry_mode(lcd::EntryModeDirection::EntryRight, lcd::EntryModeShift::NoShift);
}


fn idle(t: &mut Threshold, mut r: idle::Resources) -> ! {
    let mut menu = MainMenu::new();
    loop {
        menu.run(t, &mut r);
    }
}

fn step_completed(_t: &mut Threshold, r: TIM1_UP_TIM10::Resources) {
    if r.DRIVER.interrupt() {
        let driver: &mut StepperDriverResource = r.DRIVER;
        r.STEPPER.step_completed(driver)
    }
}

fn hall_interrupt(_t: &mut Threshold, r: TIM2::Resources) {
    r.HALL.interrupt();
}

#[no_mangle]
#[lang = "panic_fmt"]
pub unsafe extern "C" fn panic_fmt(_: ::core::fmt::Arguments, _: &'static str, _: u32, _: u32) -> ! {
    // Immediately disable driver outputs, do it without claiming the driver
    let gpioa = &(*stm32f103xx::GPIOA.get());
    gpioa.write_pin(DRIVER_ENABLE_PIN, false);

    loop {
        cortex_m::asm::nop();
    }
}