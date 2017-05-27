pub mod gpio;
pub mod clock;
pub mod delay;
pub mod lcd;
pub mod led;

use stm32f103xx::{Gpioa, Gpiob, Syst, Rcc};

// LCD
static RS: gpio::PinRange<Gpiob> = gpio::PinRange::new(1, 1);
static RW: gpio::PinRange<Gpiob> = gpio::PinRange::new(10, 1);
static E: gpio::PinRange<Gpiob> = gpio::PinRange::new(11, 1);
static DATA: gpio::PinRange<Gpiob> = gpio::PinRange::new(12, 4);

// LED
static LED: gpio::PinRange<Gpioa> = gpio::PinRange::new(4, 1);

