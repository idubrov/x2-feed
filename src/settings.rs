use stm32f103xx::FLASH;
use eeprom::EEPROM;
use stm32_hal::flash::FlashResult;

pub struct Setting {
    tag: u16,
    default: u16,
    min: u16,
    max: u16
}

impl Setting {
    pub const fn new(tag: u16, default: u16, min: u16, max: u16) -> Setting {
        Setting { tag, default, min, max }
    }

    pub fn range(&self) -> (u16, u16) {
        (self.min, self.max)
    }

    pub fn read(&self, flash: &FLASH) -> u16 {
        flash.eeprom().read(self.tag)
            .map(|v| v.max(self.min).min(self.max))
            .unwrap_or(self.default)
    }

    pub fn write(&self, flash: &FLASH, value: u16) -> FlashResult {
        flash.eeprom().write(self.tag, value.max(self.min).min(self.max))
    }
}

pub const IS_LATHE: Setting = Setting::new(0x01, 0, 0, 1);
pub const IS_REVERSED: Setting = Setting::new(0x02, 0, 0, 1);
pub const MICROSTEPS: Setting = Setting::new(0x03, 16, 1, 125);
pub const PITCH: Setting = Setting::new(0x04, 16, 1, 32);
pub const MAX_IPM: Setting = Setting::new(0x05, 30, 1, 30);
pub const ACCELERATION: Setting = Setting::new(0x06, 1200, 200, 2400); // Steps per second per second