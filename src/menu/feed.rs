use crate::app::idle::SharedResources;
use crate::font;
use crate::hal::{Button, Controls, Display, Event, QuadEncoder};
use crate::menu::util::{NavStatus, Navigation};
use crate::menu::{limits, steputil, MenuItem, MenuResources};
use crate::settings;
use crate::stepper::State as StepperState;
use crate::stepper::{Direction, StepperError};
use core::fmt;
use core::fmt::Write;
use rtic::Mutex;
use stepgen::Error as StepgenError;

#[derive(Copy, Clone)]
pub enum FeedRate {
    /// Inches per minute
    InchesPerMinute(u16),
    /// Thousands of inches per revolution
    InchesPerRevolution(u16),
}

impl FeedRate {
    fn with_rate(&self, rate: u16) -> Self {
        match *self {
            FeedRate::InchesPerMinute(_ipm) => FeedRate::InchesPerMinute(rate),
            FeedRate::InchesPerRevolution(_ipr) => FeedRate::InchesPerRevolution(rate),
        }
    }

    fn rate(&self) -> u16 {
        match *self {
            FeedRate::InchesPerMinute(rate) | FeedRate::InchesPerRevolution(rate) => rate,
        }
    }

    fn to_speed(self, steps_per_inch: u32, rpm: u32) -> u32 {
        // Update stepper speed based on current setting
        // Shift by 8 to convert to 24.8 format
        match self {
            FeedRate::InchesPerMinute(ipm) => ((u32::from(ipm) * steps_per_inch) << 8) / 60,
            FeedRate::InchesPerRevolution(ipr) => {
                // IPR are in thou, so additionally divide by 1_000
                // Also, RPM is already in 24.8 format, so no need to shift
                let result = u64::from(ipr)
                    .checked_mul(u64::from(rpm))
                    .unwrap()
                    .checked_mul(u64::from(steps_per_inch))
                    .unwrap()
                    .checked_div(60_000)
                    .unwrap();
                if result > u64::from(u32::MAX) {
                    panic!("speed overflow");
                }
                result as u32
            }
        }
    }
}

impl fmt::Display for FeedRate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FeedRate::InchesPerMinute(ipm) => write!(f, "{: >3} IPM", ipm),
            FeedRate::InchesPerRevolution(ipr) => write!(f, "0.{:0>3} IPR", ipr),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FeedSpeed {
    Fast,
    Slow,
}

pub struct FeedOperation {
    speed: u32,
    error: Option<StepperError>,
    slow_speed: FeedRate,
    fast_speed: FeedRate,
    feed: FeedSpeed,
    rpm: u32,
    limits: (Option<i32>, Option<i32>),
}

impl FeedOperation {
    pub fn new(is_lathe: bool) -> FeedOperation {
        FeedOperation {
            speed: 0,
            error: None,
            slow_speed: if is_lathe {
                FeedRate::InchesPerRevolution(4)
            } else {
                FeedRate::InchesPerMinute(10)
            },
            fast_speed: FeedRate::InchesPerMinute(30),
            feed: FeedSpeed::Slow,
            rpm: 0,
            limits: (None, None),
        }
    }

    fn update_screen(
        &self,
        shared: &mut SharedResources,
        display: &mut Display,
        controls: &Controls,
        feed: FeedRate,
    ) {
        let run_state = shared.stepper.lock(|s| s.state());
        let feed_speed = if controls.state().fast {
            FeedSpeed::Fast
        } else {
            FeedSpeed::Slow
        };

        display.position(0, 0);
        let rrpm = (self.rpm + 128) >> 8;

        let llim = if self.limits.0.is_some() { " L" } else { "  " };
        let rlim = if self.limits.1.is_some() { " R" } else { "  " };
        write!(display, "{: >4} RPM{}{}", rrpm, llim, rlim).unwrap();

        display.position(0, 1);
        let c = match (run_state, feed_speed) {
            (
                StepperState::Running {
                    dir: Direction::Left,
                    ..
                },
                FeedSpeed::Slow,
            ) => font::LEFT,
            (
                StepperState::Running {
                    dir: Direction::Right,
                    ..
                },
                FeedSpeed::Slow,
            ) => font::RIGHT,
            (
                StepperState::Running {
                    dir: Direction::Left,
                    ..
                },
                FeedSpeed::Fast,
            ) => font::FAST_LEFT,
            (
                StepperState::Running {
                    dir: Direction::Right,
                    ..
                },
                FeedSpeed::Fast,
            ) => font::FAST_RIGHT,
            _ => ' ',
        };
        write!(display, "{}{}", c, feed).unwrap();
        match self.error {
            Some(StepperError::StepgenError(StepgenError::TooSlow)) => {
                write!(display, " Slow!").unwrap()
            }
            Some(StepperError::StepgenError(StepgenError::TooFast)) => {
                write!(display, " Fast!").unwrap()
            }
            _ => write!(display, "      ").unwrap(),
        };
    }

    fn handle_feed_rate(&mut self, event: Event, encoder: &mut QuadEncoder) -> FeedRate {
        let proto = match self.feed {
            FeedSpeed::Fast => self.fast_speed,
            FeedSpeed::Slow => self.slow_speed,
        };
        // Encoder is off by one (as it starts from 0)
        let mut feed = proto.with_rate(encoder.current() + 1);
        match event {
            Event::Pressed(Button::Fast) => {
                // Switch to fast IPM
                self.feed = FeedSpeed::Fast;
                self.slow_speed = feed;
                feed = self.fast_speed;
                encoder.set_current(feed.rate() - 1);
            }
            Event::Unpressed(Button::Fast) => {
                // Switch to slow IPM
                self.feed = FeedSpeed::Slow;
                self.fast_speed = feed;
                feed = self.slow_speed;
                encoder.set_current(feed.rate() - 1);
            }
            _ => {}
        }

        feed
    }

    fn update_speed(&mut self, shared: &mut SharedResources, speed: u32) {
        if self.speed != speed {
            self.speed = speed;
            self.error = shared.stepper.lock(|s| s.set_speed(speed)).err();
        }
    }

    fn update_movement(&mut self, event: Event, shared: &mut SharedResources) {
        let run_state = shared.stepper.lock(|s| s.state());
        match (run_state, event) {
            (StepperState::Stopped, Event::Pressed(Button::Left)) => {
                // Use very low number for moving left
                // FIXME: explicit support for -+INF?
                let target = self.limits.0.unwrap_or(-1_000_000_000);
                shared.stepper.lock(|s| s.move_to(target)).unwrap();
            }

            (StepperState::Stopped, Event::Pressed(Button::Right)) => {
                // Use very high number for moving right
                let target = self.limits.1.unwrap_or(1_000_000_000);
                shared.stepper.lock(|s| s.move_to(target)).unwrap();
            }

            (
                StepperState::Running {
                    dir: Direction::Left,
                    ..
                },
                Event::Unpressed(Button::Left),
            )
            | (
                StepperState::Running {
                    dir: Direction::Right,
                    ..
                },
                Event::Unpressed(Button::Right),
            ) => shared.stepper.lock(|s| s.stop()),

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
        r.reload_stepper_settings();

        // Pre-compute steps-per-inch
        let steps_per_inch = settings::steps_per_inch(r.flash);

        let mut encoder = r
            .encoder
            .set_current_limit(self.slow_speed.rate() - 1, settings::MAX_IPM.read(r.flash));

        r.display.clear();

        let mut nav = Navigation::new();
        loop {
            let event = r.controls.read_event();
            let rpm = r.shared.hall.lock(|hall| hall.rpm());

            let feed = self.handle_feed_rate(event, &mut encoder);
            self.update_speed(&mut r.shared, feed.to_speed(steps_per_inch, rpm));
            self.update_movement(event, &mut r.shared);
            self.update_rpm(rpm);
            self.update_screen(&mut r.shared, r.display, r.controls, feed);

            if let Some(status) = nav.check(r.estop, event) {
                self.stop_and_wait(&mut r.shared, r.display);
                return status;
            }
        }
    }

    fn stop_and_wait(&self, shared: &mut SharedResources, display: &mut Display) {
        shared.stepper.lock(|s| s.stop());
        {
            display.clear();
            display.position(0, 0);
            write!(display, "Stopping").unwrap();
            display.position(0, 1);
            write!(display, "  ...").unwrap();
        }

        steputil::wait_stopped(shared);
    }
}

impl MenuItem for FeedOperation {
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

    // fn is_active_by_default(&self, _r: &mut MenuResources) -> bool {
    //     true
    // }
}
