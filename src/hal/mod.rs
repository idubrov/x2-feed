mod controls;
pub mod delay;
mod driver;
mod encoder;
mod estop;
mod led;
mod rpm;
mod screen;

pub const FREQUENCY: u32 = 72_000_000;

use eeprom::Params;
use stm32f1xx_hal::flash::{FlashSize, SectorSize};
pub use self::controls::{Button, Controls, ControlsState, Event};
pub use self::driver::DRIVER_TICK_FREQUENCY;
pub use self::driver::{StepperDriver, StepperDriverImpl};
pub use self::encoder::QuadEncoder;
pub use self::estop::EStop;
pub use self::led::Led;
pub use self::rpm::RpmSensor;
pub use self::screen::Screen;

pub type Display = lcd::Display<Screen>;

/// Important! Need to reserve that amount of pages at the end in the linker script!
const EEPROM_PAGES: u32 = 10;

pub const EEPROM_PARAMS: Params = Params {
  first_page: (FlashSize::Sz64K as u32) - EEPROM_PAGES,
  flash_size: FlashSize::Sz64K,
  page_size: SectorSize::Sz1K,
  page_count: EEPROM_PAGES,
};

