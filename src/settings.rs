use eeprom::EEPROM;
use stm32_hal::flash::FlashResult;
use stm32f1::stm32f103::FLASH;

#[derive(Clone, Copy)]
pub struct Setting {
    tag: u16,
    default: u16,
    min: u16,
    max: u16,
    label: &'static str,
}

impl Setting {
    pub const fn new(label: &'static str, tag: u16, default: u16, min: u16, max: u16) -> Setting {
        Setting {
            label,
            tag,
            default,
            min,
            max,
        }
    }

    pub fn range(&self) -> (u16, u16) {
        (self.min, self.max)
    }

    pub fn read(&self, flash: &FLASH) -> u16 {
        flash
            .eeprom()
            .read(self.tag)
            .map(|v| v.max(self.min).min(self.max))
            .unwrap_or(self.default)
    }

    pub fn write(&self, flash: &FLASH, value: u16) -> FlashResult {
        flash
            .eeprom()
            .write(self.tag, value.max(self.min).min(self.max))
    }

    pub fn label(&self) -> &'static str {
        self.label
    }
}

// Currently not configurable
pub const STEPS_PER_ROTATION: u32 = 200;
pub const IS_LATHE: Setting = Setting::new("Is Lathe?", 0x01, 0, 0, 1);
pub const IS_REVERSED: Setting = Setting::new("Reverse Dir?", 0x02, 0, 0, 1);
pub const MICROSTEPS: Setting = Setting::new("Microsteps", 0x03, 16, 1, 125);
pub const PITCH: Setting = Setting::new("Pitch", 0x04, 16, 1, 32);
pub const MAX_IPM: Setting = Setting::new("Max IPM", 0x05, 30, 1, 30);
// Steps per second per second
pub const ACCELERATION: Setting = Setting::new("Acceleration", 0x06, 1200, 200, 2400);

/// Read settings and calculate how many steps do we make per inch
pub fn steps_per_inch(flash: &FLASH) -> u32 {
    u32::from(PITCH.read(flash)) * u32::from(MICROSTEPS.read(flash)) * STEPS_PER_ROTATION
}
