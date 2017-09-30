use hal::{StepperDriver, RpmSensor, Event};
use hal::Event::*;
use hal::Button::*;
use idle;
use config::{Display, StepperDriverResource};
use font;
use rtfm::{Resource, Threshold};
use stepper;
use estop;
use super::{Menu, MenuResult};
use core::fmt::Write;

// FIXME: move to EEPROM?
const PITCH: u32 = 16;
const MICROSTEPS: u32 = 16;
const MAX_IPM: u16 = 30;
const ACCELERATION: u32 = 1200; // Steps per second per second
const STEPS_PER_ROTATION: u32 = 200;


pub struct FeedMenu {
    speed: u32,
    slow_ipm: u16,
    fast_ipm: u16,
    ipm: u16,
    rpm: u32
}

impl FeedMenu {
    pub fn new() -> FeedMenu {
        FeedMenu {
            speed: 0,
            // Offset by 1, as IPM of 0 is not allowed.
            slow_ipm: 10 - 1,
            fast_ipm: 30 - 1,
            rpm: 0,
            ipm: 0,
        }
    }

    fn update_screen(&self, t: &mut Threshold, r: &mut idle::Resources) {
        let run_state = stepper_command(t, r, |s, _| s.state());
        let is_fast = r.CONTROLS.state().fast;

        let mut lcd = Display::new(r.SCREEN);
        lcd.position(0, 0);
        let rrpm = (self.rpm + 128) >> 8;
        write!(lcd, "{: >4} RPM", rrpm).unwrap();

        lcd.position(0, 1);
        let c = match (run_state, is_fast) {
            (stepper::State::Running(false), false) => font::LEFT,
            (stepper::State::Running(true), false) => font::RIGHT,
            (stepper::State::Running(false), true) => font::FAST_LEFT,
            (stepper::State::Running(true), true) => font::FAST_RIGHT,
            _ => ' '
        };
        write!(lcd, "{}{: >3} IPM", c, u32::from(self.ipm + 1)).unwrap();
    }

    fn handle_ipm(&mut self, event: Event, t: &mut Threshold, r: &mut idle::Resources) {
        let mut ipm = r.ENCODER.current() + 1; // Encoder is off by one (as it starts from 0)
        match event {
            Pressed(Fast) => {
                // Switch to fast IPM
                self.slow_ipm = ipm;
                ipm = self.fast_ipm;
                r.ENCODER.set_current(ipm - 1);
            },
            Unpressed(Fast) => {
                // Switch to slow IPM
                self.fast_ipm = ipm;
                ipm = self.slow_ipm;
                r.ENCODER.set_current(ipm - 1);
            },
            _ => {}
        }

        // Update stepper speed based on current setting
        // Shift by 8 to convert to 24.8 format
        let speed = (u32::from(ipm << 8) * PITCH * STEPS_PER_ROTATION * MICROSTEPS) / 60;
        if self.speed != speed {
            stepper_command(t, r, |s, _| { s.set_speed(speed) }).unwrap();
            self.speed = speed;
        }
        self.ipm = ipm
    }

    fn handle_feed(&mut self, event: Event, t: &mut Threshold, r: &mut idle::Resources) {
        let run_state = stepper_command(t, r, |s, _| s.state());
        match (run_state, event) {
            (stepper::State::Stopped, Pressed(Left)) => {
                // Use very low number for moving left
                stepper_command(t, r, |s, d| { s.move_to(d, -1_000_000_000).unwrap(); });
            }

            (stepper::State::Stopped, Pressed(Right)) => {
                // Use very high number for moving right
                stepper_command(t, r, |s, d| { s.move_to(d, 1_000_000_000).unwrap(); });
            }

            (stepper::State::Running(false), Unpressed(Left)) |
            (stepper::State::Running(true), Unpressed(Right)) =>
                stepper_command(t, r, |s, _| s.stop()),

            _ => {}
        }
    }

    fn handle_rpm(&mut self, t: &mut Threshold, r: &idle::Resources) {
        use rtfm::Resource;

        let rpm = r.HALL.claim(t, |hall, _t| hall.rpm());

        // Only capture if difference is big enough (more than .5%)
        if self.rpm == 0 || rpm * 200 > self.rpm * 201 || rpm * 200 < self.rpm * 199 {
            self.rpm = rpm;
        }
    }

    fn run_feed(&mut self, t: &mut Threshold, r: &mut idle::Resources) -> MenuResult {
        stepper_command(t, r, |s, _|
            s.set_acceleration((ACCELERATION * MICROSTEPS) << 8).unwrap());
        r.ENCODER.set_current(self.slow_ipm - 1);
        r.ENCODER.set_limit(MAX_IPM);

        loop {
            estop::check(&mut Display::new(r.SCREEN));

            let event = r.CONTROLS.read_event();
            self.handle_ipm(event, t, r);
            self.handle_feed(event, t, r);
            self.handle_rpm(t, r);
            self.update_screen(t, r);
        }
    }
}

impl Menu for FeedMenu {
    fn run(&mut self, t: &mut Threshold, r: &mut idle::Resources) -> MenuResult {
        self.run_feed(t, r)
    }

    fn label(&self) -> &'static str {
        "Feed (IPM)"
    }
}

// Helper function to run stepper command. Claims both driver and stepper.
fn stepper_command<T, CB>(t: &mut Threshold, r: &mut idle::Resources, cb: CB) -> T
    where
        CB: for<'a> FnOnce(&mut stepper::Stepper, &mut StepperDriver) -> T {

    let stepper = &mut r.STEPPER;
    let driver = &mut r.DRIVER;
    stepper.claim_mut(t, |stepper, t| {
        driver.claim_mut(t, |driver, _t| {
            cb(stepper, driver as &mut StepperDriverResource)
        })
    })
}