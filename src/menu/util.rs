use idle;
use rtfm::Threshold;
use core::fmt::Write;
use config::Display;
use ::menu::MenuItem;
use hal::{Event, Button, delay};
use settings;

pub fn run_menu(items: &mut [&mut MenuItem], t: &mut Threshold, r: &mut idle::Resources) {
    if let Some(def) = items.iter().position(|item| item.is_active_by_default(t, r)) {
        items[def].run(t, r);
    }

    let mut selected = 0usize;
    let mut exit: Exit = Exit::new();

    loop {
        r.ENCODER.set_current(selected as u16);
        r.ENCODER.set_limit(items.len() as u16);

        Display::new(r.SCREEN).clear();
        loop {
            selected = r.ENCODER.current() as usize;
            let current: &mut MenuItem = items[selected];

            let event = r.CONTROLS.read_event();
            if let Event::Unpressed(Button::Encoder) = event {
                current.run(t, r);
                break;
            }

            let mut lcd = Display::new(r.SCREEN);
            lcd.position(0, 0);
            write!(&mut lcd, "{}", current).unwrap();

            if exit.should_exit(event) {
                return;
            }
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

pub struct Exit {
    pressed_at: Option<u32>
}

impl Exit {
    pub fn new() -> Self {
        Self {
            pressed_at: None
        }
    }
    pub fn should_exit(&mut self, event: Event) -> bool {
        if let Some(pressed_at) = self.pressed_at {
            if delay::duration_us(pressed_at) >= EXIT_DURATION_US {
                return true;
            }
        }
        match event {
            Event::Pressed(Button::Encoder) => self.pressed_at = Some(delay::current()),
            Event::Unpressed(Button::Encoder) => self.pressed_at = None,
            _ => {}
        }
        false
    }
}