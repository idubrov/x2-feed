use stepgen;

use hw::config::DRIVER_TICK_FREQUENCY;
use hal::stepper::Driver;

#[derive(Clone, Copy, PartialEq)]
enum State {
    /// Not stepping
    Stopped,
    /// Stepping and stop command was requested
    StopRequested,
    /// Stopping the motor
    Stopping,
    /// Running in the negative direction (`direction` is `false`)
    RunningNegative,
    /// Running in the positive direction (`direction` is `true`)
    RunningPositive,
}

pub struct Stepper {
    // If should reverse direction signal
    // FIXME: remove...
    reverse: bool,

    stepgen: stepgen::Stepgen,
    direction: bool,

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
    pub const fn new() -> Stepper {
        Stepper {
            reverse: true,
            stepgen: stepgen::Stepgen::new(DRIVER_TICK_FREQUENCY),
            direction: true,
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
    fn preload_delay(&mut self, driver: &mut Driver) {
        match self.stepgen.next() {
            Some(delay) => driver.preload_delay(round16_8(delay)),
            None => driver.set_last(),
        }
    }

    pub fn step_completed(&mut self, driver: &mut Driver) {
        match self.state {
            State::StopRequested => {
                // Initiate stopping sequence -- set target step to 0
                self.stepgen.set_target_step(0);
                self.state = State::Stopping
            },
            State::Stopping if !driver.is_running() => {
                driver.enable(false);
                self.state = State::Stopped;
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
    fn update_position(&mut self) -> i32 {
        let step = self.stepgen.current_step();
        let offset = (step - self.base_step) as i32;
        self.base_step = step;
        if self.direction {
            self.position += offset;
        } else {
            self.position -= offset;
        }
        self.position
    }

    fn set_direction(&mut self, driver: &mut Driver, dir: bool) {
        driver.direction(if self.reverse { dir } else { !dir });
        self.direction = dir;
    }

    /// Move to given position. Note that no new move commands will be accepted while stepper is
    /// running. However, other target parameter, target speed, could be changed any time.
    pub fn move_to(&mut self, driver: &mut Driver, target: i32) -> bool {
        if !self.is_stopped() {
            return false;
        }

        let pos = self.update_position();
        let delta;
        if pos < target {
            delta = (target - pos) as u32;
            self.set_direction(driver, true);
            self.state = State::RunningPositive;
        } else if pos > target {
            delta = (pos - target) as u32;
            self.set_direction(driver, false);
            self.state = State::RunningNegative;
        } else {
            // Nothing to do!
            return true;
        }
        self.stepgen.set_target_step(self.base_step + delta);

        // Enable driver outputs
        driver.enable(true);

        // Start pulse generation
        let delay = self.stepgen.next().unwrap();
        driver.start(round16_8(delay));

        // Immediately preload the second delay
        self.preload_delay(driver);
        true
    }


    pub fn stop(&mut self) {
        match self.state {
            State::RunningNegative | State::RunningPositive =>
                self.state = State::StopRequested,
            _ => ()
        }
    }

    pub fn is_stopped(&self) -> bool {
        self.state == State::Stopped
    }
}