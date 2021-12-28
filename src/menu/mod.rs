use self::feed::FeedMenu;
use crate::settings;
use core::fmt;
use stm32f1::stm32f103::FLASH;
use stm32_hal::gpio::Pin;
use crate::hal::{Controls, Display, QuadEncoder};

pub struct MenuResources<'a> {
    pub encoder: &'a mut QuadEncoder,
    pub display: &'a mut Display,
    pub controls: &'a mut Controls,
    pub flash: &'a mut FLASH,
    pub estop: &'a Pin,
    pub shared: crate::app::idle::SharedResources<'a>,
}

#[macro_use]
mod util;
mod feed;
mod limits;
mod steputil;

pub trait MenuItem: fmt::Display {
    fn run(&mut self, r: &mut MenuResources);

    fn is_active_by_default(&self, _r: &mut MenuResources) -> bool {
        false
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
    Acceleration()
});

menu!(MainMenu, "Main", {
    FeedMenu(true),
    SettingsMenu()
});
