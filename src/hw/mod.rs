pub mod gpio;
pub mod clock;
pub mod delay;
pub mod lcd;
pub mod led;
pub mod encoder;
pub mod driver;

use stm32f103xx::{Gpioa, Gpiob, Syst, Rcc};

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