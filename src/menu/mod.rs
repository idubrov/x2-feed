use self::feed::FeedMenuItem;
use self::thread::ThreadMenuItem;
use crate::hal::{Controls, Display, QuadEncoder};
use crate::settings;
use core::fmt;
use rtic::Mutex;
use stm32_hal::gpio::Pin;
use stm32f1::stm32f103::FLASH;

pub struct MenuResources<'a> {
    pub encoder: &'a mut QuadEncoder,
    pub display: &'a mut Display,
    pub controls: &'a mut Controls,
    pub flash: &'a mut FLASH,
    pub estop: &'a Pin,
    pub shared: crate::app::idle::SharedResources<'a>,
    /// Stepper driver frequency (timer ticks per second), used for calculating acceleration time for threads
    pub driver_freq: u32,
}

impl MenuResources<'_> {
    // Reload stepper settings from EEPROM
    fn reload_stepper_settings(&mut self) {
        let reversed = settings::IS_REVERSED.read(self.flash) != 0;
        let acceleration = (u32::from(settings::ACCELERATION.read(self.flash))
            * u32::from(settings::MICROSTEPS.read(self.flash)))
            << 8;

        self.shared.stepper.lock(|s| {
            s.set_reversed(reversed);
            s.set_acceleration(acceleration).unwrap();
        });
    }
}

#[macro_use]
mod util;
mod feed;
mod limits;
mod steputil;
mod thread;

pub trait MenuItem: fmt::Display {
    fn run(&mut self, r: &mut MenuResources);

    fn is_active_by_default(&self, _r: &mut MenuResources) -> bool {
        false
    }

    fn is_enabled(&self, _r: &mut MenuResources) -> bool {
        true
    }
}

menu_setting!(IsLathe, "Is Lathe?", settings::IS_LATHE);
menu_setting!(Reversed, "Is Reversed?", settings::IS_REVERSED);
menu_setting!(Microsteps, "Microsteps", settings::MICROSTEPS);
menu_setting!(Pitch, "Pitch", settings::PITCH);
menu_setting!(MaxIPM, "MaxIPM", settings::MAX_IPM);
menu_setting!(Acceleration, "Acceleration", settings::ACCELERATION);

menu!(SettingsMenu, "Settings", {
    IsLathe(),
    Reversed(),
    Microsteps(),
    Pitch(),
    MaxIPM(),
    Acceleration(),
});

menu!(MainMenu, "Main", {
    FeedMenuItem(),
    ThreadMenuItem(),
    SettingsMenu(),
});
