use crate::hal::{Button, Event, Display, QuadEncoder, STEPS_PER_ROTATION};
use crate::menu::util::{NavStatus, Navigation};
use crate::menu::{steputil, MenuResources};
use crate::settings;
use core::fmt;
use core::fmt::Write;
use rtic::Mutex;

// Any reasonably big number to make sure you cannot crank half of it on the encoder between 'ticks'
const LIMIT: u16 = 20_000;

struct EncoderDelta<'a> {
    last: u16,
    old_limit: u16,
    old_position: u16,
    encoder: &'a mut QuadEncoder,
}

impl<'a> EncoderDelta<'a> {
    fn new(encoder: &'a mut QuadEncoder) -> Self {
        let old_limit = encoder.get_limit();
        let old_position = encoder.current();
        encoder.set_current(LIMIT / 2);
        encoder.set_limit(LIMIT);
        Self {
            last: LIMIT / 2,
            old_limit,
            old_position,
            encoder,
        }
    }

    pub fn delta(&mut self) -> i16 {
        let current = self.encoder.current();
        // Substract unsigned wrapping around LIMIT
        let delta = if current < self.last {
            current + LIMIT - self.last
        } else {
            current - self.last
        };
        self.last = current;
        // Convert delta to signed -LIMIT/2 to LIMIT/2
        if delta < LIMIT / 2 {
            delta as i16
        } else {
            (delta as i16) - LIMIT as i16
        }
    }
}

impl<'a> Drop for EncoderDelta<'a> {
    fn drop(&mut self) {
        self.encoder.set_limit(self.old_limit);
        self.encoder.set_current(self.old_position);
    }
}

fn print_limit(lcd: &mut Display, position: Option<i32>, steps_per_inch: u32) -> fmt::Result {
    match position {
        None => {
            write!(lcd, "Not set")
        }
        Some(position) => {
            let thousands =
                (1000 * position + (steps_per_inch as i32) / 2) / (steps_per_inch as i32);
            let inches = thousands / 1000;
            let thousands = thousands % 1000;
            write!(lcd, "{}.{:0>3}", inches, thousands.abs())
        }
    }
}

pub fn capture_limit(r: &mut MenuResources, label: &'static str) -> (Option<i32>, NavStatus) {
    let mut deltaenc = EncoderDelta::new(r.encoder);
    let mut limit: Option<i32> = None;
    let mut nav = Navigation::new();

    // Pre-compute steps-per-inch
    let steps_per_inch = u32::from(settings::PITCH.read(r.flash))
        * u32::from(settings::MICROSTEPS.read(r.flash))
        * STEPS_PER_ROTATION;

    loop {
        let pos = r.shared.stepper.lock(|s| s.position());
        let delta = i32::from(deltaenc.delta());
        let event = r.controls.read_event();

        // Update screen
        r.display.position(0, 0);
        write!(r.display, "{}: ", label).unwrap();
        print_limit(&mut r.display, limit, steps_per_inch).unwrap();
        r.display.position(0, 1);
        print_limit(&mut r.display, Some(pos), steps_per_inch).unwrap();

        // Update stepper position
        if delta != 0 {
            let speed = ((10 * steps_per_inch) << 8) / 60;
            // FIXME: Traversal speed?
            r.shared.stepper.lock(|s| s.set_speed(speed)).unwrap();
            steputil::move_delta(delta * (steps_per_inch as i32) / 1000, &mut r.shared);
            // FIXME: print "MOVING..."
            steputil::wait_stopped(&mut r.shared);
        }

        // Update limit
        if let Event::Pressed(Button::Fast) = event {
            limit = match limit {
                Some(limit) if limit == pos => None,
                _ => Some(pos),
            };
        }

        if let Some(status) = nav.check(r.estop, event) {
            return (limit, status);
        }
    }
}
