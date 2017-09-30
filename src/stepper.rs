use stepgen;

use hal::StepperDriver;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum State {
    /// Not stepping
    Stopped,
    /// Stepping and stop command was requested
    StopRequested(bool),
    /// Stopping the motor
    Stopping(bool),
    /// Stepper motor is running
    Running(bool),
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Error {
    /// Stepper is not stopped to run given command
    NotStopped
}

pub struct Stepper {
    stepgen: stepgen::Stepgen,

    base_step: u32,
    position: i32,

    state: State,
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
            base_step: 0,
            position: 0,
            state: State::Stopped,
        }
    }

    /// Set new acceleration (steps per second per second), in 24.8 format.
    pub fn set_acceleration(&mut self, acceleration: u32) -> stepgen::Result {
        self.stepgen.set_acceleration(acceleration)
    }

    /// Set slew speed (maximum speed stepper motor would run).
    ///
    /// Sets desired slew speed, a maximum speed stepper motor would accelerate to. Note that
    /// stepper motor would only reach this speed if destination step is far enough, so there is
    /// enough time for deceleration.
    ///
    /// * `speed` - target slew speed to reach, in steps per second, 24.8 format
    pub fn set_speed(&mut self, speed: u32) -> stepgen::Result {
        self.stepgen.set_target_speed(speed)
    }

    /// Returns `false` no new delay was loaded
    fn preload_delay(&mut self, driver: &mut StepperDriver) {
        match self.stepgen.next() {
            Some(delay) => driver.preload_delay(round16_8(delay)),
            None => driver.set_last(),
        }
    }

    pub fn step_completed(&mut self, driver: &mut StepperDriver) {
        match self.state {
            State::StopRequested(dir) => {
                // Initiate stopping sequence -- set target step to 0
                self.stepgen.set_target_step(0);
                self.state = State::Stopping(dir)
            },
            State::Stopping(dir) if !driver.is_running() => {
                driver.enable(false);
                self.state = State::Stopped;

                // Update internal position counter. We do it at the end to reduce amount of work
                // we do per step (direction could not be changed while running, so all steps go
                // in one direction).
                self.update_position(dir);
                return; // Do not preload the delay -- we are stopped now
            },
            State::Stopped =>
                panic!("Should not receive interrupts when stopped!"),
            // Nothing otherwise, just preload the delay
            _ => ()
        };
        self.preload_delay(driver);
    }

    // Incorporate outstanding steps from the stepgen into current position
    fn update_position(&mut self, dir: bool) {
        let step = self.stepgen.current_step();
        let offset = (step - self.base_step) as i32;
        self.base_step = step;
        if dir {
            self.position += offset;
        } else {
            self.position -= offset;
        }
    }

    /// Move to given position. Note that no new move commands will be accepted while stepper is
    /// running. However, other target parameter, target speed, could be changed any time.
    pub fn move_to(&mut self, driver: &mut StepperDriver, target: i32) -> Result<(), Error> {
        if self.state != State::Stopped {
            return Err(Error::NotStopped);
        }

        if self.position == target {
            // Nothing to do!
            return Ok(());
        }

        let delta = target - self.position;
        let dir = delta > 0;

        self.state = State::Running(dir);
        self.stepgen.set_target_step(self.base_step + (delta.abs() as u32));

        // Set direction and enable driver outputs
        driver.direction(dir);
        driver.enable(true);

        // Start pulse generation
        let delay = self.stepgen.next().unwrap();
        driver.start(round16_8(delay));

        // Immediately preload the second delay
        self.preload_delay(driver);
        Ok(())
    }


    pub fn stop(&mut self) {
        if let State::Running(dir) = self.state {
            self.state = State::StopRequested(dir);
        }
    }

    /// Get the stepper state
    pub fn state(&self) -> State {
        self.state
    }
}