use stepgen;

use hal::StepperDriver;
use core;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum State {
    /// Not stepping
    Stopped,
    /// Stepper motor is running
    Running(bool),
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Error {
    /// Stepper is not stopped to run given command
    NotStopped,

    /// Stepgen error
    StepgenError(stepgen::Error),
}

type Result = core::result::Result<(), Error>;

impl From<stepgen::Error> for Error {
    fn from(err: stepgen::Error) -> Self {
        Error::StepgenError(err)
    }
}

pub struct Stepper {
    stepgen: stepgen::Stepgen,
    reversed: bool,

    position: i32,
    state: State,
    target: Target,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Target {
    /// Move to the left indefinitely
    LeftInf,
    /// Move to the right indefinitely
    RightInf,
    /// Stop
    Stop,
    /// Move till the given position
    Position(i32)
}

// Round value from 16.8 format to u16
fn round16_8(delay: u32) -> u16 {
    let d = (delay + 128) >> 8;
    d as u16
}

impl Stepper {
    pub const fn new(freq: u32) -> Stepper {
        Stepper {
            stepgen: stepgen::Stepgen::new(freq),
            reversed: false,
            position: 0,
            state: State::Stopped,
            target: Target::Stop,
        }
    }

    /// Set reversed flag
    pub fn set_reversed(&mut self, reversed: bool) {
        self.reversed = reversed;
    }

    /// Set new acceleration (steps per second per second), in 24.8 format.
    pub fn set_acceleration(&mut self, acceleration: u32) -> Result {
        Ok(self.stepgen.set_acceleration(acceleration)?)
    }

    /// Set slew speed (maximum speed stepper motor would run).
    ///
    /// Sets desired slew speed, a maximum speed stepper motor would accelerate to. Note that
    /// stepper motor would only reach this speed if destination step is far enough, so there is
    /// enough time for deceleration.
    ///
    /// * `speed` - target slew speed to reach, in steps per second, 24.8 format
    pub fn set_speed(&mut self, speed: u32) -> Result {
        Ok(self.stepgen.set_target_speed(speed)?)
    }

    /// Preload next delay for PWM
    fn preload_delay(&mut self, driver: &mut StepperDriver) {
        match self.stepgen.next() {
            Some(delay) => driver.preload_delay(round16_8(delay)),
            None => {
                driver.set_last()
            }
        }
    }

    fn increment_position(&mut self) {
        match self.state {
            State::Running(true) => self.position += 1,
            State::Running(false) => self.position -= 1,
            _ => {}
        }
    }

    pub fn step_completed(&mut self, driver: &mut StepperDriver) {
        self.increment_position();

        // Preload next step if still running
        if driver.is_running() {
            self.preload_delay(driver);
        } else {
            self.start_chase_target(driver).expect("should chase target");
        }
    }

    fn start_chase_target(&mut self, driver: &mut StepperDriver) -> Result {
        match self.target {
            Target::LeftInf => {
                self.init_move(driver, false, core::u32::MAX)?;
            }
            Target::RightInf => {
                self.init_move(driver, true, core::u32::MAX)?;
            }
            Target::Position(target) => {
                let delta = target - self.position;
                if delta != 0 {
                    self.init_move(driver, delta > 0, delta.abs() as u32)?;
                }
            }
            _ => {
                self.state = State::Stopped;
                driver.set_enable(false);
            }
        }
        Ok(())
    }

    fn init_move(&mut self, driver: &mut StepperDriver, dir: bool, target_step: u32) -> Result {
        self.state = State::Running(dir);
        self.stepgen.set_target_step(target_step)?;

        // Set direction and enable driver outputs
        driver.set_direction(dir != self.reversed);
        driver.set_enable(true);

        // Start pulse generation
        let delay = self.stepgen.next().expect("must have pulse");
        driver.start(round16_8(delay));

        // Immediately preload the second delay
        self.preload_delay(driver);
        Ok(())
    }

    /// Move to given position. Note that no new move commands will be accepted while stepper is
    /// running. However, other target parameter, target speed, could be changed any time.
    pub fn move_to(&mut self, driver: &mut StepperDriver, target: Target) -> Result {
        self.target = target;

        match self.state {
            State::Running(dir) => {
                let (want_dir, want_step) = match target {
                    Target::LeftInf => (false, core::u32::MAX),
                    Target::RightInf => (true, core::u32::MAX),
                    Target::Position(target) => {
                        let delta = target - self.position;
                        // FIXME: remove current_step?...
                        (delta > 0, self.stepgen.current_step() + (delta.abs() as u32))
                    }
                    Target::Stop => (!dir, 0) // Request opposite direction to make it stop
                };

                if want_dir == dir {
                    self.stepgen.set_target_step(want_step)?;
                } else {
                    // Need to stop to change direction
                    self.stepgen.set_target_step(0)?;
                }
                Ok(())
            }
            State::Stopped => self.start_chase_target(driver),
        }
    }

    pub fn stop(&mut self) {
        // FIXME: remove guard if, otherwise cannot call stop if speed is not set...
        if self.stepgen.target_step() != 0 {
            self.stepgen.set_target_step(0).expect("should stop");
            self.target = Target::Stop;
        }
    }

    /// Get the stepper state
    pub fn state(&self) -> State {
        self.state
    }

    /// Current stepper position
    pub fn position(&self) -> i32 {
        self.position
    }
}