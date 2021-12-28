use crate::font;
use crate::hal::{Button, Event, STEPS_PER_ROTATION};
use crate::menu::util::{NavStatus, Navigation};
use crate::menu::{limits, steputil, MenuItem, MenuResources};
use crate::settings;
use crate::stepper::Error as StepperError;
use crate::stepper::State as StepperState;
use core::fmt;
use core::fmt::Write;
use rtic::Mutex;
use stepgen::Error as StepgenError;

#[derive(Copy, Clone)]
pub enum FeedRate {
    IPM(u16),
    IPR(u16),
}

impl FeedRate {
    fn with_rate(&self, rate: u16) -> Self {
        match *self {
            FeedRate::IPM(_ipm) => FeedRate::IPM(rate),
            FeedRate::IPR(_ipr) => FeedRate::IPR(rate),
        }
    }

    fn rate(&self) -> u16 {
        match *self {
            FeedRate::IPM(rate) | FeedRate::IPR(rate) => rate,
        }
    }

    fn to_speed(&self, steps_per_inch: u32, rpm: u32) -> u32 {
        // Update stepper speed based on current setting
        // Shift by 8 to convert to 24.8 format
        match *self {
            FeedRate::IPM(ipm) => ((u32::from(ipm) * steps_per_inch) << 8) / 60,
            FeedRate::IPR(ipr) => {
                // IPR are in thou, so additionally divide by 1_000
                // Also, RPM is already in 24.8 format, so no need to shift
                // FIXME: wrapping?
                (u64::from(ipr) * u64::from(rpm) * u64::from(steps_per_inch) / 60_000) as u32
            }
        }
    }
}

impl fmt::Display for FeedRate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FeedRate::IPM(ipm) => write!(f, "{: >3} IPM", ipm),
            FeedRate::IPR(ipr) => write!(f, "0.{:0>3} IPR", ipr),
        }
    }
}

pub struct FeedMenu {
    speed: u32,
    error: Option<StepperError>,
    slow_feed: FeedRate,
    fast_feed: FeedRate,
    fast: bool,
    rpm: u32,
    limits: (Option<i32>, Option<i32>),
}

impl FeedMenu {
    pub fn new(ipr: bool) -> FeedMenu {
        FeedMenu {
            speed: 0,
            error: None,
            slow_feed: if ipr {
                FeedRate::IPR(4)
            } else {
                FeedRate::IPM(10)
            },
            fast_feed: FeedRate::IPM(30),
            fast: false,
            rpm: 0,
            limits: (None, None),
        }
    }

    fn update_screen(&self, r: &mut MenuResources, feed: FeedRate) {
        let run_state = r.shared.stepper.lock(|s| s.state());
        let is_fast = r.controls.state().fast;

        r.display.position(0, 0);
        let rrpm = (self.rpm + 128) >> 8;

        let llim = if self.limits.0.is_some() { " L" } else { "  " };
        let rlim = if self.limits.1.is_some() { " R" } else { "  " };
        write!(r.display, "{: >4} RPM{}{}", rrpm, llim, rlim).unwrap();

        r.display.position(0, 1);
        let c = match (run_state, is_fast) {
            (StepperState::Running(false), false) => font::LEFT,
            (StepperState::Running(true), false) => font::RIGHT,
            (StepperState::Running(false), true) => font::FAST_LEFT,
            (StepperState::Running(true), true) => font::FAST_RIGHT,
            _ => ' ',
        };
        write!(r.display, "{}{}", c, feed).unwrap();
        match self.error {
            Some(StepperError::StepgenError(StepgenError::TooSlow)) => {
                write!(r.display, " Slow!").unwrap()
            }
            Some(StepperError::StepgenError(StepgenError::TooFast)) => {
                write!(r.display, " Fast!").unwrap()
            }
            _ => write!(r.display, "      ").unwrap(),
        };
    }

    fn handle_feed_rate(&mut self, event: Event, r: &mut MenuResources) -> FeedRate {
        // Encoder is off by one (as it starts from 0)
        let proto = if self.fast {
            self.fast_feed
        } else {
            self.slow_feed
        };
        let mut feed = proto.with_rate(r.encoder.current() + 1);
        match event {
            Event::Pressed(Button::Fast) => {
                // Switch to fast IPM
                self.fast = true;
                self.slow_feed = feed;
                feed = self.fast_feed;
                r.encoder.set_current(feed.rate() - 1);
            }
            Event::Unpressed(Button::Fast) => {
                // Switch to slow IPM
                self.fast = false;
                self.fast_feed = feed;
                feed = self.slow_feed;
                r.encoder.set_current(feed.rate() - 1);
            }
            _ => {}
        }

        feed
    }

    fn update_speed(&mut self, r: &mut MenuResources, speed: u32) {
        if self.speed != speed {
            self.speed = speed;
            self.error = r.shared.stepper.lock(|s| s.set_speed(speed)).err();
        }
    }

    fn update_movement(&mut self, event: Event, r: &mut MenuResources) {
        let run_state = r.shared.stepper.lock(|s| s.state());
        match (run_state, event) {
            (StepperState::Stopped, Event::Pressed(Button::Left)) => {
                // Use very low number for moving left
                // FIXME: explicit support for -+INF?
                move_to(r, self.limits.0.unwrap_or(-1_000_000_000));
            }

            (StepperState::Stopped, Event::Pressed(Button::Right)) => {
                // Use very high number for moving right
                move_to(r, self.limits.1.unwrap_or(1_000_000_000));
            }

            (StepperState::Running(false), Event::Unpressed(Button::Left))
            | (StepperState::Running(true), Event::Unpressed(Button::Right)) => {
                r.shared.stepper.lock(|s| s.stop())
            }

            _ => {}
        }
    }

    fn update_rpm(&mut self, rpm: u32) {
        // Only capture if difference is big enough (more than .5%)
        if self.rpm == 0 || rpm * 200 > self.rpm * 201 || rpm * 200 < self.rpm * 199 {
            self.rpm = rpm;
        }
    }

    fn run_feed(&mut self, r: &mut MenuResources) -> NavStatus {
        reload_stepper_settings(r);

        // Pre-compute steps-per-inch
        let steps_per_inch = u32::from(settings::PITCH.read(r.flash))
            * u32::from(settings::MICROSTEPS.read(r.flash))
            * STEPS_PER_ROTATION;

        r.encoder.set_current(self.slow_feed.rate() - 1);
        r.encoder.set_limit(settings::MAX_IPM.read(r.flash));

        r.display.clear();

        let mut nav = Navigation::new();
        loop {
            let event = r.controls.read_event();
            let rpm = r.shared.hall.lock(|hall| hall.rpm());

            let feed = self.handle_feed_rate(event, r);
            self.update_speed(r, feed.to_speed(steps_per_inch, rpm));
            self.update_movement(event, r);
            self.update_rpm(rpm);
            self.update_screen(r, feed);

            if let Some(status) = nav.check(&r.estop, event) {
                self.stop_and_wait(r);
                return status;
            }
        }
    }

    fn stop_and_wait(&self, r: &mut MenuResources) {
        r.shared.stepper.lock(|s| s.stop());
        {
            r.display.clear();
            r.display.position(0, 0);
            write!(r.display, "Stopping").unwrap();
            r.display.position(0, 1);
            write!(r.display, "  ...").unwrap();
        }

        steputil::wait_stopped(&mut r.shared);
    }
}

impl MenuItem for FeedMenu {
    fn run(&mut self, r: &mut MenuResources) {
        loop {
            // FIXME: make submenus?
            if let NavStatus::Exit = self.run_feed(r) {
                break;
            }

            let (left, status) = limits::capture_limit(r, "Left");
            if let NavStatus::Exit = status {
                break;
            }

            let (right, status) = limits::capture_limit(r, "Right");
            if let NavStatus::Exit = status {
                break;
            }

            self.limits = (left, right);
        }
    }

    fn is_active_by_default(&self, r: &mut MenuResources) -> bool {
        let lathe = settings::IS_LATHE.read(r.flash) != 0;
        match self.slow_feed {
            FeedRate::IPR(_) => lathe,
            FeedRate::IPM(_) => !lathe,
        }
    }
}

impl fmt::Display for FeedMenu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Feed (IPM)")
    }
}

// Helper function to run stepper command. Claims both driver and stepper.
fn move_to(r: &mut MenuResources, target: i32) {
    r.shared.stepper.lock(|s| s.move_to(target)).unwrap()
}

// Reload stepper settings from EEPROM
fn reload_stepper_settings(r: &mut MenuResources) {
    let reversed = settings::IS_REVERSED.read(r.flash) != 0;
    let acceleration = (u32::from(settings::ACCELERATION.read(r.flash))
        * u32::from(settings::MICROSTEPS.read(r.flash)))
        << 8;

    r.shared.stepper.lock(|s| {
        s.set_reversed(reversed);
        s.set_acceleration(acceleration).unwrap();
    });
}
