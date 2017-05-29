#![allow(dead_code)]

pub mod gpio;
pub mod clock;
pub mod delay;
pub mod lcd;
pub mod led;
pub mod encoder;
pub mod driver;
pub mod stepper;
pub mod controls;

extern crate stepgen;

use stm32f103xx::{Gpioa, Gpiob};

// LCD
pub static RS: gpio::PinRange<Gpiob> = gpio::PinRange::new(1, 1);
pub static RW: gpio::PinRange<Gpiob> = gpio::PinRange::new(10, 1);
pub static E: gpio::PinRange<Gpiob> = gpio::PinRange::new(11, 1);
pub static DATA: gpio::PinRange<Gpiob> = gpio::PinRange::new(12, 4);

// LED
pub static LED: gpio::PinRange<Gpioa> = gpio::PinRange::new(4, 1);

// Encoder
pub static BTN: gpio::PinRange<Gpioa> = gpio::PinRange::new(5, 1);
pub static DT: gpio::PinRange<Gpioa> = gpio::PinRange::new(6, 1);
pub static CLK: gpio::PinRange<Gpioa> = gpio::PinRange::new(7, 1);

// Emergency stop
pub static ESTOP: gpio::PinRange<Gpiob> = gpio::PinRange::new(0, 1);

// Stepper driver
pub static STEP: gpio::PinRange<Gpioa> = gpio::PinRange::new(8, 1);
pub static DIR: gpio::PinRange<Gpioa> = gpio::PinRange::new(9, 1);
pub static ENABLE: gpio::PinRange<Gpioa> = gpio::PinRange::new(10, 1);
pub static RESET: gpio::PinRange<Gpioa> = gpio::PinRange::new(11, 1);

pub const FREQUENCY: u32 = 72_000_000;
pub const TICK_FREQUENCY: u32 = 1_000_000; // 1us timer resolution

const fn ns2ticks(ns: u32) -> u16 {
    const NANOS_IN_TICK: u32 = 1000000000 / TICK_FREQUENCY;
    return ((ns + NANOS_IN_TICK - 1) / NANOS_IN_TICK) as u16;
}

pub const STEP_PULSE_WIDTH_TICKS: u16 = ns2ticks(75);

// Controls
pub static LEFT: gpio::PinRange<Gpioa> = gpio::PinRange::new(1, 1);
pub static RIGHT: gpio::PinRange<Gpioa> = gpio::PinRange::new(2, 1);
pub static FAST: gpio::PinRange<Gpioa> = gpio::PinRange::new(3, 1);
