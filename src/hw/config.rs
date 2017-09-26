#![allow(dead_code)]
pub const FREQUENCY: u32 = 72_000_000;

use stm32f103xx::{GPIOA, GPIOB};

use hw::gpio;

pub mod lcd {
    pub type PORT = super::GPIOB;
    pub const RS: usize = 1; // PB1 is RS
    pub const RW: usize = 10; // PB10 is RW
    pub const E: usize = 11; // PB11 is E
    pub const DATA: usize = 12; // PB12-PB15 are DB4-DB7
}

// LED
pub const LED: gpio::PinRange<GPIOA> = gpio::PinRange::new(4, 1);

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

// Controls
pub const LEFT: gpio::PinRange<GPIOA> = gpio::PinRange::new(1, 1);
pub const RIGHT: gpio::PinRange<GPIOA> = gpio::PinRange::new(2, 1);
pub const FAST: gpio::PinRange<GPIOA> = gpio::PinRange::new(3, 1);

// Hall
pub const HALL_TICK_FREQUENCY: u32 = 100_000; // 0.01 ms
pub const HALL_MAX_RPM: u32 = 6000;
pub const HALL_MIN_RPM: u32 = 50;

pub const HALL: gpio::PinRange<GPIOA> = gpio::PinRange::new(0, 1);