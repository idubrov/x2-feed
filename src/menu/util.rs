use idle;
use rtfm::Threshold;
use core::fmt::Write;
use config::Display;
use menu::MenuItem;
use hal::{Event, Button, delay};
use settings;
use estop;

pub fn run_menu(items: &mut [&mut MenuItem], t: &mut Threshold, r: &mut idle::Resources) {
    if let Some(def) = items.iter().position(|item| item.is_active_by_default(t, r)) {
        items[def].run(t, r);
    }

    let mut selected = 0usize;
    let mut nav: Navigation = Navigation::new();

    loop {
        r.ENCODER.set_current(selected as u16);
        r.ENCODER.set_limit(items.len() as u16);

        Display::new(r.SCREEN).clear();
        loop {
            selected = r.ENCODER.current() as usize;
            let current: &mut MenuItem = items[selected];

            let event = r.CONTROLS.read_event();
            match nav.check(event) {
                Some(NavStatus::Exit) => return,
                Some(NavStatus::Select) => {
                    current.run(t, r);
                    break;
                },
                _ => {},
            }

            let mut lcd = Display::new(r.SCREEN);
            lcd.position(0, 0);
            write!(&mut lcd, "{}", current).unwrap();
        }
    }
}

pub fn run_setting(setting: &settings::Setting, label: &'static str, r: &mut idle::Resources) {
    let mut lcd = Display::new(r.SCREEN);
    lcd.clear();

    let (min, max) = setting.range();
    let orig = setting.read(r.FLASH);
    r.ENCODER.set_current(orig - min);
    r.ENCODER.set_limit(max - min + 1);
    loop {
        if let Event::Pressed(Button::Encoder) = r.CONTROLS.read_event() {
            break;
        }

        lcd.position(0, 0);
        write!(lcd, "{: <16}", label).unwrap();
        lcd.position(0, 1);
        write!(lcd, "{}", r.ENCODER.current() + min).unwrap();
    }

    let current = r.ENCODER.current() + min;
    if current != orig {
        setting.write(r.FLASH, current).unwrap();
    }
}

macro_rules! menu {
    ($name:ident, $label: expr, { $( $item:ident ( $($params:expr),* ) ),* }) => {
        #[allow(non_snake_case)]
        pub struct $name {
            $($item: $item),*
        }

        impl $name {
            pub fn new() -> Self {
                Self {
                    $( $item: $item::new( $($params),* ) ),*
                }
            }
        }

        impl MenuItem for $name {
            fn run(&mut self, t: &mut Threshold, r: &mut idle::Resources) {
                ::menu::util::run_menu(&mut [ $( &mut self.$item ),* ], t, r)
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{: <16}", $label)
            }
        }
    }
}

macro_rules! menu_setting {
    ($name:ident, $label:expr, $setting:path) => {
        #[allow(non_snake_case)]
        pub struct $name;

        impl $name {
            pub fn new() -> Self {
                Self { }
            }
        }

        impl MenuItem for $name {
            fn run(&mut self, _t: &mut Threshold, r: &mut idle::Resources) {
                ::menu::util::run_setting(&$setting, $label, r)
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{: <16}", $label)
            }
        }
    }
}

const EXIT_DURATION_US: u32 = 1_000_000;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NavStatus {
    Exit,
    Select
}

pub struct Navigation {
    pressed_at: Option<u32>
}

impl Navigation {
    pub fn new() -> Self {
        Self {
            pressed_at: None
        }
    }
    pub fn check(&mut self, event: Event) -> Option<NavStatus> {
        estop::check();

        if let Some(pressed_at) = self.pressed_at {
            if delay::duration_us(pressed_at) >= EXIT_DURATION_US {
                return Some(NavStatus::Exit);
            }
        }
        match event {
            Event::Pressed(Button::Encoder) => self.pressed_at = Some(delay::current()),
            Event::Unpressed(Button::Encoder) => {
                self.pressed_at = None;
                return Some(NavStatus::Select)
            },
            _ => {}
        }
        None
    }
}