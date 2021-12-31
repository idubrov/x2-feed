use crate::hal::{Button, Event};
use crate::menu::util::{printable_position, NavStatus, Navigation};
use crate::menu::{steputil, MenuResources};
use crate::settings;
use core::fmt::Write;
use rtic::Mutex;

pub fn capture_limit(r: &mut MenuResources, label: &'static str) -> (Option<i32>, NavStatus) {
    let mut deltaenc = r.encoder.delta_encoder();
    let mut limit: Option<i32> = None;
    let mut nav = Navigation::new();

    // Pre-compute steps-per-inch
    let steps_per_inch = settings::steps_per_inch(r.flash) as i32;

    loop {
        let pos = r.shared.stepper.lock(|s| s.position());
        let delta = i32::from(deltaenc.delta());
        let event = r.controls.read_event();

        // Update screen
        r.display.position(0, 0);
        write!(r.display, "{}: ", label).unwrap();
        match limit {
            None => write!(r.display, "Not Set").unwrap(),
            Some(limit) => {
                write!(r.display, "{}", printable_position(limit, steps_per_inch)).unwrap()
            }
        }
        r.display.position(0, 1);
        write!(r.display, "{}", printable_position(pos, steps_per_inch)).unwrap();

        // Update stepper position; unit is one thou
        if delta != 0 {
            // FIXME: hard-coded speed?...
            let speed = ((10 * steps_per_inch) << 8) / 60;
            // FIXME: Traversal speed?
            r.shared
                .stepper
                .lock(|s| s.set_speed(speed as u32))
                .unwrap();
            steputil::move_delta(delta * steps_per_inch / 1000, &mut r.shared);
            // FIXME: print "MOVING..."
            steputil::wait_stopped(&mut r.shared);
        }

        match event {
            Event::Pressed(Button::Fast) if limit == Some(pos) => {
                limit = None;
            }
            Event::Pressed(Button::Fast) => {
                limit = Some(pos);
            }
            Event::Pressed(Button::Left) => {
                // FIXME: feed...
            }
            _ => {}
        }

        if let Some(status) = nav.check(r.estop, event) {
            return (limit, status);
        }
    }
}
