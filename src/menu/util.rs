use crate::hal::{delay, Button, Event, EStop, Controls};
use crate::menu::MenuResources;
use crate::settings;
use core::fmt::Write;

/// Run a "selection menu", a menu where one of the several items is selected. Items could be
/// selected both by pressing "Fast" button or by pressing "Select" button for a short period.
/// Pressing "Select" for longer acts as an "Exit" action (no selection is returned).
#[inline(never)]
pub fn run_selection<'a, T: core::fmt::Display>(
    r: &mut MenuResources,
    header: &str,
    elements: &'a [T],
    initial: usize,
) -> Option<&'a T> {
    run_selection_internal(r, header, &|pos| &elements[pos], initial, elements.len())
        .map(|pos| &elements[pos])
}

/// Run a "selection menu", a menu where one of the several items is selected. Items could be
/// selected both by pressing "Fast" button or by pressing "Select" button for a short period.
/// Pressing "Select" for longer acts as an "Exit" action (no selection is returned).
pub fn run_selection_idx<T: core::fmt::Display>(
    r: &mut MenuResources,
    header: &str,
    elements: &[T],
    initial: usize,
) -> Option<usize> {
    run_selection_internal(r, header, &|pos| &elements[pos], initial, elements.len())
}

/// Run a "selection menu", a menu where one of the several items is selected. Items could be
/// selected both by pressing "Fast" button or by pressing "Select" button for a short period.
/// Pressing "Select" for longer acts as an "Exit" action (no selection is returned).
fn run_selection_internal<'a>(
    r: &mut MenuResources,
    header: &str,
    labels: &'a dyn Fn(usize) -> &'a dyn core::fmt::Display,
    initial: usize,
    total: usize,
) -> Option<usize> {
    let encoder = r.encoder.set_current_limit(initial as u16, total as u16);
    r.display.clear();
    wait_loop(r.controls, r.estop, || {
        let selected = usize::from(encoder.current());
        let label = labels(selected);
        r.display.position(0, 0);
        write!(r.display, "{: <16}", header).unwrap();
        r.display.position(0, 1);
        write!(r.display, "{: <16}", label).unwrap();
        selected
    })
}


pub fn run_setting(r: &mut MenuResources, setting: &settings::Setting) {
    r.display.clear();

    let (min, max) = setting.range();
    let orig = setting.read(r.eeprom);
    let encoder = r.encoder.set_current_limit(orig - min, max - min + 1);
    loop {
        if let Event::Unpressed(Button::Encoder) = r.controls.read_event() {
            break;
        }

        r.display.position(0, 0);
        write!(r.display, "{: <16}", setting.label()).unwrap();
        r.display.position(0, 1);
        write!(r.display, "{: <16}", encoder.current() + min).unwrap();
    }

    let current = encoder.current() + min;
    if current != orig {
        setting.write(r.eeprom, current).unwrap();
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
    /// Duration for which `Select` button was pressed.
    pressed_duration: Option<delay::Duration>,
}

impl Navigation {
    pub fn new() -> Self {
        Self { pressed_duration: None }
    }
    pub fn check(&mut self, estop: &EStop, event: Event) -> Option<NavStatus> {
        if estop.is_emergency_stop() {
            panic!("*E-STOP*");
        }

        if let Some(ref mut pressed_duration) = self.pressed_duration {
            if pressed_duration.duration() > EXIT_DURATION_US {
                return Some(NavStatus::Exit);
            }
        }
        match event {
            Event::Pressed(Button::Encoder) => self.pressed_duration = Some(delay::Duration::new()),
            Event::Unpressed(Button::Encoder) => {
                if core::mem::take(&mut self.pressed_duration).is_some() {
                    return Some(NavStatus::Select);
                }
            }
            _ => {}
        }
        None
    }
}

pub struct PrintablePosition {
    position: i32,
    steps_per_inch: i32,
}

pub fn printable_position(position: i32, steps_per_inch: i32) -> PrintablePosition {
    PrintablePosition {
        position,
        steps_per_inch,
    }
}

impl core::fmt::Display for PrintablePosition {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        // Divide with rounding
        let bias = if self.position < 0 {
            -self.steps_per_inch / 2
        } else {
            self.steps_per_inch / 2
        };
        let thousands = (1000 * self.position + bias) / (self.steps_per_inch as i32);
        let inches = thousands / 1000;
        let thousands = thousands % 1000;
        let sign = if thousands < 0 { "-" } else { "" };
        write!(f, "{}{}.{:0>3}", sign, inches.abs(), thousands.abs())
    }
}

/// Run a "wait" loop: execute given callback in a loop until operator presses `Select` button
/// or `Fast` button. If `Select` is pressed for a long period, the function returns `None`
/// (indicating "exit"). Otherwise, the return value is the value returned from the callback.
pub fn wait_loop<R>(controls: &mut Controls, estop: &mut EStop, mut cb: impl FnMut() -> R) -> Option<R> {
    let mut nav = Navigation::new();
    loop {
        let result = cb();

        // We use `Fast` button for continuing the operation instead of typical `Encoder` button.
        let event = controls.read_event();
        match nav.check(estop, event) {
            Some(NavStatus::Exit) => return None,
            Some(NavStatus::Select) => return Some(result),
            None if matches!(event, Event::Pressed(Button::Fast)) => return Some(result),
            _ => {}
        }
    }
}
