use self::feed::FeedOperation;
use self::thread::ThreadingOperation;
use crate::hal::{Controls, Display, QuadEncoder};
use crate::settings;
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
    /// Reload stepper settings from EEPROM. Sets acceleration, reverse flag and speed. Speed
    /// is set to the default traversal speed.
    fn reload_stepper_settings(&mut self) {
        let reversed = settings::IS_REVERSED.read(self.flash) != 0;
        let acceleration = (u32::from(settings::ACCELERATION.read(self.flash))
            * u32::from(settings::MICROSTEPS.read(self.flash)))
            << 8;
        let traversal = u32::from(settings::TRAVERSAL.read(self.flash));
        let steps_per_inch = settings::steps_per_inch(self.flash);
        let speed = ((traversal * steps_per_inch) << 8) / 60;

        self.shared.stepper.lock(|s| {
            s.set_reversed(reversed);
            s.set_speed(speed).unwrap();
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

/// Trait for a generic menu item
pub trait MenuItem {
    fn run(&mut self, r: &mut MenuResources);
}

pub struct SettingsMenuTemplate<const N: usize> {
    settings: [settings::Setting; N],
}

pub type SettingsMenu = SettingsMenuTemplate<7>;

impl<const N: usize> MenuItem for SettingsMenuTemplate<N> {
    fn run(&mut self, r: &mut MenuResources) {
        while let Some(setting) =
            crate::menu::util::run_selection(r, "-- Settings --", &self.settings, 0)
        {
            crate::menu::util::run_setting(r, setting);
        }
    }
}

impl SettingsMenu {
    pub fn new() -> SettingsMenu {
        SettingsMenu {
            settings: [
                settings::IS_LATHE,
                settings::IS_REVERSED,
                settings::MICROSTEPS,
                settings::PITCH,
                settings::MAX_IPM,
                settings::ACCELERATION,
                settings::TRAVERSAL,
            ],
        }
    }
}

pub struct LatheMenu {
    feed: FeedOperation,
    thread: ThreadingOperation,
    settings: SettingsMenu,
}

impl LatheMenu {
    pub fn new() -> LatheMenu {
        LatheMenu {
            feed: FeedOperation::new(true),
            thread: ThreadingOperation::new(),
            settings: SettingsMenu::new(),
        }
    }
}

impl MenuItem for LatheMenu {
    fn run(&mut self, r: &mut MenuResources) {
        const LABELS: [&str; 3] = ["> Power Feed", "> Threading", "> Settings"];

        // Default menu item
        self.feed.run(r);
        while let Some(pos) = crate::menu::util::run_selection_idx(r, "-- Select --", &LABELS, 0) {
            match pos {
                0 => self.feed.run(r),
                1 => self.thread.run(r),
                2 => self.settings.run(r),
                _ => unreachable!(),
            }
        }
    }
}

pub struct MillMenu {
    feed: FeedOperation,
    settings: SettingsMenu,
}

impl MillMenu {
    pub fn new() -> MillMenu {
        MillMenu {
            feed: FeedOperation::new(false),
            settings: SettingsMenu::new(),
        }
    }
}

impl MenuItem for MillMenu {
    fn run(&mut self, r: &mut MenuResources) {
        const LABELS: [&str; 2] = ["> Power Feed", "> Settings"];

        // Default menu item
        self.feed.run(r);
        while let Some(pos) = crate::menu::util::run_selection_idx(r, "-- Select --", &LABELS, 0) {
            match pos {
                0 => self.feed.run(r),
                1 => self.settings.run(r),
                _ => unreachable!(),
            }
        }
    }
}
