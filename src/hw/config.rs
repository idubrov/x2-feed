#![allow(dead_code)]
pub const FREQUENCY: u32 = 72_000_000;

use stm32f103xx::{GPIOA, GPIOB};

use hw::gpio;
use hal::{Controls, Led};
use hal;

pub type Screen = hal::Screen<GPIOB>;
pub const fn screen() -> Screen {
    // PB1 is RS, PB10 is RW, PB11 is E, PB12-PB15 are DB4-DB7
    Screen::new(GPIOB, 1, 10, 11, 12)
}

// Encoder
pub const BTN: gpio::PinRange<GPIOA> = gpio::PinRange::new(5, 1);
pub const DT: gpio::PinRange<GPIOA> = gpio::PinRange::new(6, 1);
pub const CLK: gpio::PinRange<GPIOA> = gpio::PinRange::new(7, 1);

// Emergency stop
pub const ESTOP: gpio::PinRange<GPIOB> = gpio::PinRange::new(0, 1);

// Stepper driver
pub const STEP: gpio::PinRange<GPIOA> = gpio::PinRange::new(8, 1);
pub const DIR: gpio::PinRange<GPIOA> = gpio::PinRange::new(9, 1);
pub const ENABLE: gpio::PinRange<GPIOA> = gpio::PinRange::new(10, 1);
pub const RESET: gpio::PinRange<GPIOA> = gpio::PinRange::new(11, 1);

pub const DRIVER_TICK_FREQUENCY: u32 = 1_000_000; // 1us timer resolution

pub const STEP_PULSE_WIDTH_NS: u16 = 75;

pub const fn led() -> Led<GPIOA> {
    Led::new(GPIOA, 4) // PA4 is LED
}

pub const fn controls() -> Controls<GPIOA> {
    Controls::new(GPIOA, 1, 2, 3)
}

pub mod hall {
    pub type PORT = super::GPIOA;
    pub fn port() -> &'static PORT { unsafe { &*super::GPIOA.get() } }

    pub const HALL_TICK_FREQUENCY: u32 = 100_000; // 0.01 ms
    pub const HALL_MAX_RPM: u32 = 6000;
    pub const HALL_MIN_RPM: u32 = 50;

    pub const PIN: usize = 0;
}