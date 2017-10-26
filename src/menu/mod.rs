use idle;
use rtfm::Threshold;
use core::fmt;
use self::feed::FeedMenu;
use settings;

#[macro_use]
mod util;
mod feed;
mod limits;
mod steputil;

pub trait MenuItem: fmt::Display {
    fn run(&mut self, t: &mut Threshold, r: &mut idle::Resources);

    fn is_active_by_default(&self, _t: &mut Threshold, _r: &mut idle::Resources) -> bool {
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
