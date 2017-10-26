use idle::Resources;
use rtfm::{Threshold, Resource};
use config::{Display, QuadEncoderResource, STEPS_PER_ROTATION};
use hal::{Event, Button};
use core::fmt::Write;
use menu::steputil;
use settings;
use core::fmt;
use menu::util::{Navigation, NavStatus};

// Any reasonably big number to make sure you cannot crank half of it on the encoder between 'ticks'
const LIMIT: u16 = 20_000;

struct EncoderDelta<'a> {
    last: u16,
    old_limit: u16,
    old_position: u16,
    encoder: &'a mut QuadEncoderResource,
}

impl <'a> EncoderDelta<'a> {
    fn new(encoder: &'a mut QuadEncoderResource) -> Self {
        let old_limit = encoder.get_limit();
        let old_position = encoder.current();
        encoder.set_current(LIMIT / 2);
        encoder.set_limit(LIMIT);
        Self {
            last: 0,
            old_limit,
            old_position,
            encoder
        }
    }

    pub fn delta(&mut self) -> i16 {
        let current = self.encoder.current();
        // Substract unsigned wrapping around LIMIT
        let delta = if current < self.last { current + LIMIT - self.last } else { current - self.last };
        self.last = current;
        // Convert delta to signed -LIMIT/2 to LIMIT/2
        if delta < LIMIT / 2 { delta as i16 } else { (delta as i16) - LIMIT as i16 }
    }
}

impl <'a> Drop for EncoderDelta<'a> {
    fn drop(&mut self) {
        self.encoder.set_limit(self.old_limit);
        self.encoder.set_current(self.old_position);
    }
}

fn print_limit(lcd: &mut Display, position: Option<i32>, steps_per_inch: u32) -> fmt::Result {
    match position {
        None => {
            write!(lcd, "Not set")
        },
        Some(position) => {
            let thousands = 1000 * position / (steps_per_inch as i32);
            let inches = thousands / 1000;
            let thousands = thousands % 1000;
            write!(lcd, "{}.{:0>3}", inches, thousands.abs())
        }
    }
}


pub fn capture_limit(t: &mut Threshold, r: &mut Resources, label: &'static str) -> (Option<i32>, NavStatus) {
    let mut lcd = Display::new(r.SCREEN);
    let mut deltaenc = EncoderDelta::new(r.ENCODER);
    let mut limit: Option<i32> = None;
    let mut nav = Navigation::new();

    // Pre-compute steps-per-inch
    let steps_per_inch = u32::from(settings::PITCH.read(r.FLASH)) *
        u32::from(settings::MICROSTEPS.read(r.FLASH)) *
        STEPS_PER_ROTATION;

    loop {
        let pos = r.STEPPER.claim(t, |s, _t| s.position());
        let delta = i32::from(deltaenc.delta());
        let event = r.CONTROLS.read_event();

        // Update screen
        lcd.position(0, 0);
        write!(lcd, "{}: ", label).unwrap();
        print_limit(&mut lcd, limit, steps_per_inch).unwrap();
        lcd.position(0, 1);
        print_limit(&mut lcd, Some(pos), steps_per_inch).unwrap();

        // Update stepper position
        if delta != 0 {
            steputil::move_delta(t, delta, &mut r.DRIVER, &mut r.STEPPER);
            // FIXME: print "MOVING..."
            steputil::wait_stopped(t, &mut r.STEPPER);
        }

        // Update limit
        if let Event::Pressed(Button::Fast) = event {
            limit = match limit {
                Some(limit) if limit == pos => None,
                _ => Some(pos),
            };
        }

        if let Some(status) = nav.check(event) {
            return (limit, status);
        }
    }
}