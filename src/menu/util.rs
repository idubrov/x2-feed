use crate::hal::{delay, Button, Event};
use crate::menu::MenuResources;
use crate::settings;
use core::fmt::Write;
use stm32_hal::gpio::Pin;

pub fn run_selection(r: &mut MenuResources, header: &str, labels: &[&str]) -> Option<usize> {
    let mut selected = 0usize;
    let mut nav: Navigation = Navigation::new();
    loop {
        r.encoder.set_current(selected as u16);
        r.encoder.set_limit(labels.len() as u16);
        r.display.clear();
        loop {
            selected = usize::from(r.encoder.current());
            let current = &labels[selected];

            let event = r.controls.read_event();
            match nav.check(&r.estop, event) {
                Some(NavStatus::Exit) => return None,
                Some(NavStatus::Select) => return Some(selected),
                _ if matches!(event, Event::Pressed(Button::Fast)) => return Some(selected),
                _ => {}
            }

            r.display.position(0, 0);
            write!(r.display, "{: <16}", header).unwrap();
            r.display.position(0, 1);
            write!(r.display, "{: <16}", current).unwrap();
        }
    }
}

pub fn run_setting(r: &mut MenuResources, setting: &settings::Setting, label: &str) {
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

/// How long "Select" button needs to be pressed to be interpreted as "exit current menu"
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
