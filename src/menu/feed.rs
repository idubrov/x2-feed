use hal::{Controls, ControlsState, StepperDriver, RpmSensor};
use idle;
use config::{Display, StepperDriverResource};
use font;
use rtfm::{Resource, Threshold};
use stepper;
use estop;
use super::Menu;
use core::fmt::Write;

// FIXME: move to EEPROM?
const PITCH: u32 = 16;
const MICROSTEPS: u32 = 16;
const MAX_IPM: u16 = 30;
const ACCELERATION: u32 = 1200; // Steps per second per second
const STEPS_PER_ROTATION: u32 = 200;


#[derive(Clone, Copy)]
enum RunState {
    Stopped,
    Stopping,
    RunningLeft,
    RunningRight
}

pub struct FeedMenu {
    run_state: RunState,
    fast: bool,
    speed: u32,
    slow_ipm: u16,
    fast_ipm: u16,
    ipm: u16,
    rpm: u32,
}

impl FeedMenu {
    pub fn new() -> FeedMenu {
        FeedMenu {
            run_state: RunState::Stopped,
            fast: false,
            speed: 0,
            // Offset by 1, as IPM of 0 is not allowed.
            slow_ipm: 10 - 1,
            fast_ipm: 30 - 1,
            rpm: 0,
            ipm: 0,
        }
    }

    fn update_screen(&self, r: &mut idle::Resources) {
        let mut lcd = Display::new(r.SCREEN);
        lcd.position(0, 0);
        let rrpm = (self.rpm + 128) >> 8;
        write!(lcd, "{: >4} RPM", rrpm).unwrap();

        lcd.position(0, 1);
        let s = (self.fast, self.run_state);
        let c = match s {
            (false, RunState::RunningLeft) => font::LEFT,
            (false, RunState::RunningRight) => font::RIGHT,
            (true, RunState::RunningLeft) => font::FAST_LEFT,
            (true, RunState::RunningRight) => font::FAST_RIGHT,
            _ => ' '
        };
        write!(lcd, "{}{: >3} IPM", c, u32::from(self.ipm + 1)).unwrap();
    }

    fn handle_ipm(&mut self, input: ControlsState, t: &mut Threshold, r: &mut idle::Resources) {
        let mut ipm = r.ENCODER.current() + 1; // Encoder is off by one (as it starts from 0)
        match (self.fast, input.fast) {
            (false, true) => {
                // Switch to fast IPM
                self.slow_ipm = ipm;
                ipm = self.fast_ipm;
                r.ENCODER.set_current(ipm - 1);
                self.fast = true;
            }

            (true, false) => {
                // Switch to slow IPM
                self.fast_ipm = ipm;
                ipm = self.slow_ipm;
                r.ENCODER.set_current(ipm - 1);
                self.fast = false;
            }

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

    fn handle_feed(&mut self, input: ControlsState, t: &mut Threshold, r: &mut idle::Resources) {
        match (self.run_state, input.left, input.right) {
            (RunState::Stopped, true, false) => {
                // Use very low number for moving left
                stepper_command(t, r, |s, d| { s.move_to(d, -1_000_000_000); });
                self.run_state = RunState::RunningLeft;
            }

            (RunState::Stopped, false, true) => {
                // Use very high number for moving right
                stepper_command(t, r, |s, d| { s.move_to(d, 1_000_000_000); });
                self.run_state = RunState::RunningRight;
            }

            (RunState::RunningLeft, false, false) | (RunState::RunningRight, false, false) => {
                stepper_command(t, r, |s, _| s.stop());
                self.run_state = RunState::Stopping;
            }

            (RunState::Stopping, _, _) => {
                if stepper_command(t, r, |s, _| s.is_stopped()) {
                    self.run_state = RunState::Stopped;
                }
            }

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

    fn run_feed(&mut self, t: &mut Threshold, r: &mut idle::Resources) {
        stepper_command(t, r, |s, _|
            s.set_acceleration((ACCELERATION * MICROSTEPS) << 8).unwrap());
        r.ENCODER.set_current(self.slow_ipm - 1);
        r.ENCODER.set_limit(MAX_IPM);


        loop {
            estop::check(&mut Display::new(r.SCREEN));

            let input = r.CONTROLS.state();
            self.handle_ipm(input, t, r);
            self.handle_feed(input, t, r);
            self.handle_rpm(t, r);
            self.update_screen(r);
        }
    }
}

impl Menu for FeedMenu {
    fn run(&mut self, t: &mut Threshold, r: &mut idle::Resources) {
        self.run_feed(t, r)
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