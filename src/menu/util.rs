use crate::hal::{delay, Button, Event};
use crate::menu::{MenuItem, MenuResources};
use crate::settings;
use core::fmt::Write;
use stm32_hal::gpio::Pin;

/// Run composite menu (menu consisting of menu items)
pub fn run_menu(items: &mut [&mut dyn MenuItem], r: &mut MenuResources) {
    if let Some(def) = items.iter().position(|item| item.is_active_by_default(r)) {
        // If we have default menu item, we enter it immediately
        items[def].run(r);
    }

    let mut selected = 0usize;
    let mut nav: Navigation = Navigation::new();

    loop {
        r.encoder.set_current(selected as u16);
        r.encoder.set_limit(items.len() as u16);

        r.display.clear();
        'inner: loop {
            selected = usize::from(r.encoder.current());
            let current: &mut dyn MenuItem = items[selected];

            let event = r.controls.read_event();
            match nav.check(&r.estop, event) {
                Some(NavStatus::Exit) => return,
                Some(NavStatus::Select) => {
                    current.run(r);
                    break 'inner;
                }
                _ => {}
            }

            r.display.position(0, 0);
            write!(&mut r.display, "{}", current).unwrap();
        }
    }
}

pub fn run_setting(setting: &settings::Setting, label: &'static str, r: &mut MenuResources) {
    r.display.clear();

    let (min, max) = setting.range();
    let orig = setting.read(r.flash);
    r.encoder.set_current(orig - min);
    r.encoder.set_limit(max - min + 1);
    loop {
        if let Event::Unpressed(Button::Encoder) = r.controls.read_event() {
            break;
        }

        r.display.position(0, 0);
        write!(r.display, "{: <16}", label).unwrap();
        r.display.position(0, 1);
        write!(r.display, "{: <16}", r.encoder.current() + min).unwrap();
    }

    let current = r.encoder.current() + min;
    if current != orig {
        setting.write(r.flash, current).unwrap();
    }
}

macro_rules! menu {
    ($name:ident, $label: expr, { $( $item:ident ( $($params:expr),* $(,)? ) ),* $(,)? }) => {
        #[allow(non_snake_case)]
        pub struct $name {
            $($item: $item),*
        }

        impl $name {
            pub fn new(r: &mut crate::menu::MenuResources) -> Self {
                Self {
                    $( $item: $item::new( r, $($params),* ) ),*
                }
            }
        }

        impl MenuItem for $name {
            fn run(&mut self, r: &mut crate::menu::MenuResources) {
                crate::menu::util::run_menu(&mut [ $( &mut self.$item ),* ], r)
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{: <16}", $label)
            }
        }
    };
}

macro_rules! menu_setting {
    ($name:ident, $label:expr, $setting:path) => {
        #[allow(non_snake_case)]
        pub struct $name;

        impl $name {
            pub fn new(_r: &mut crate::menu::MenuResources) -> Self {
                Self {}
            }
        }

        impl MenuItem for $name {
            fn run(&mut self, r: &mut crate::menu::MenuResources) {
                crate::menu::util::run_setting(&$setting, $label, r)
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{: <16}", $label)
            }
        }
    };
}

const EXIT_DURATION_US: u32 = 1_500_000;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NavStatus {
    Exit,
    Select,
}

pub struct Navigation {
    pressed_at: Option<u32>,
}

impl Navigation {
    pub fn new() -> Self {
        Self { pressed_at: None }
    }
    pub fn check(&mut self, estop: &Pin, event: Event) -> Option<NavStatus> {
        if !estop.read() {
            panic!("*E-STOP*");
        }

        if let Some(pressed_at) = self.pressed_at {
            if delay::duration_us(pressed_at) >= EXIT_DURATION_US {
                return Some(NavStatus::Exit);
            }
        }
        match event {
            Event::Pressed(Button::Encoder) => self.pressed_at = Some(delay::current()),
            Event::Unpressed(Button::Encoder) => {
                let was_pressed = self.pressed_at.is_some();
                self.pressed_at = None;
                if was_pressed {
                    return Some(NavStatus::Select);
                }
            }
            _ => {}
        }
        None
    }
}
