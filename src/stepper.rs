use crate::hal::StepperDriver;

/// Direction of stepper motor movement
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum State {
    /// Not stepping
    Stopped,
    /// Stepping and stop command was requested
    StopRequested(Direction),
    /// Stopping the motor
    Stopping(Direction),
    /// Stepper motor is running
    Running {
        dir: Direction,
        /// If chasing the thread
        is_cutting_thread: bool,
    },
    /// Awaiting for the spindle sync event to initiate thread cutting
    ThreadStart,
    /// Awaiting for the delay before we start accelerating our stepper
    ThreadDelay,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum StepperError {
    /// Stepper is not stopped to run given command
    NotStopped,

    /// Stepgen error
    StepgenError(stepgen::Error),
}

impl From<stepgen::Error> for StepperError {
    fn from(err: stepgen::Error) -> Self {
        StepperError::StepgenError(err)
    }
}

pub struct Stepper<S: StepperDriver> {
    stepgen: stepgen::Stepgen,
    threads: crate::threads::ThreadInfo,
    driver: S,
    reversed: bool,
    // We never manually control screw on a the lathe, so we never want to disable the driver,
    // to prevent accidental screw changes.
    disable_at_stop: bool,

    base_step: u32,
    position: i32,

    state: State,
}

// Round value from 16.8 format to u16
fn round16_8(delay: u32) -> u16 {
    let d = (delay + 128) >> 8;
    d as u16
}

impl<S: StepperDriver> Stepper<S> {
    pub fn new(freq: u32, driver: S, disable_at_stop: bool) -> Stepper<S> {
        Stepper {
            driver,
            stepgen: stepgen::Stepgen::new(freq),
            threads: crate::threads::ThreadInfo::new(freq),
            reversed: false,
            disable_at_stop,
            base_step: 0,
            position: 0,
            state: State::Stopped,
        }
    }

    /// Set reversed flag
    pub fn set_reversed(&mut self, reversed: bool) {
        self.reversed = reversed;
    }

    /// Set new acceleration (steps per second per second), in 24.8 format.
    pub fn set_acceleration(&mut self, acceleration: u32) -> Result<(), StepperError> {
        self.stepgen.set_acceleration(acceleration)?;
        self.threads.set_acceleration(acceleration);
        Ok(())
    }

    /// Set slew speed (maximum speed stepper motor would run).
    ///
    /// Sets desired slew speed, a maximum speed stepper motor would accelerate to. Note that
    /// stepper motor would only reach this speed if destination step is far enough, so there is
    /// enough time for deceleration.
    ///
    /// * `speed` - target slew speed to reach, in (micro-)steps per second, 24.8 format
    pub fn set_speed(&mut self, speed: u32) -> Result<(), StepperError> {
        Ok(self.stepgen.set_target_speed(speed)?)
    }

    /// Returns `false` no new delay was loaded
    fn preload_delay(&mut self) {
        match self.stepgen.next() {
            Some(delay) => self.driver.preload_delay(round16_8(delay)),
            None => {
                if let State::Running { dir, .. } = self.state {
                    self.state = State::Stopping(dir);
                }
                self.driver.set_last()
            }
        }
    }

    pub fn interrupt(&mut self) {
        if !self.driver.interrupt() {
            return;
        }

        match self.state {
            State::StopRequested(dir) => {
                // Initiate stopping sequence -- set target step to 0
                self.stepgen.set_target_step(0).unwrap();
                self.state = State::Stopping(dir);
                self.preload_delay();
            }
            State::Stopping(dir) if !self.driver.is_running() => {
                if self.disable_at_stop {
                    self.driver.set_enable(false);
                }
                self.state = State::Stopped;
                // FIXME: reset thread cutting info

                // Update internal position counter. We do it at the end to reduce amount of work
                // we do per step (direction could not be changed while running, so all steps go
                // in one direction).
                self.update_position(dir);
                // Do not preload the delay -- we are stopped now
            }
            State::Stopping(_) | State::Running { .. } => {
                // Just preload the delay
                self.preload_delay();
            }
            State::Stopped => panic!("Should not receive interrupts when stopped!"),
            State::ThreadStart => {
                panic!("Should not receive interrupts when waiting for the spindle!")
            }
            State::ThreadDelay if !self.driver.is_running() => {
                // Finished our delay, need to initiate thread cutting
                self.driver.set_timer_output(true);
                self.move_to(self.threads.target_position()).unwrap();
            }
            State::ThreadDelay => match self.threads.next_wait_delay() {
                0 => self.driver.set_last(),
                delay => self.driver.preload_delay(delay),
            },
        };
    }

    // Incorporate outstanding steps from the stepgen into current position
    fn update_position(&mut self, dir: Direction) {
        let step_pos = self.calc_position(dir);
        self.base_step = step_pos.0;
        self.position = step_pos.1;
    }

    // Compute current position based on stepgen step + last position
    fn calc_position(&self, dir: Direction) -> (u32, i32) {
        let step = self.stepgen.current_step();
        let offset = (step - self.base_step) as i32;
        match dir {
            Direction::Left => (step, self.position - offset),
            Direction::Right => (step, self.position + offset),
        }
    }

    /// Move to given position. Note that no new move commands will be accepted while stepper is
    /// running. However, other target parameter, target speed, could be changed any time.
    pub fn move_to(&mut self, target: i32) -> Result<(), StepperError> {
        if self.state != State::Stopped && self.state != State::ThreadDelay {
            return Err(StepperError::NotStopped);
        }

        if self.position == target {
            // Nothing to do!
            self.state = State::Stopped;
            return Ok(());
        }

        let delta = target - self.position;
        let dir = if delta > 0 {
            Direction::Right
        } else {
            Direction::Left
        };

        self.state = State::Running {
            dir,
            is_cutting_thread: self.state == State::ThreadDelay,
        };
        self.stepgen
            .set_target_step(self.base_step + delta.unsigned_abs())?;

        // Set direction and enable driver outputs
        let dir_bit = match dir {
            Direction::Left => self.reversed,
            Direction::Right => !self.reversed,
        };
        self.driver.set_direction(dir_bit);
        self.driver.set_enable(true);

        // Start pulse generation
        let delay = self.stepgen.next().unwrap();
        self.driver.start(round16_8(delay));

        // Immediately preload the second delay
        self.preload_delay();
        Ok(())
    }

    pub fn stop(&mut self) {
        if let State::Running { dir, .. } = self.state {
            self.state = State::StopRequested(dir);
        }
    }

    /// Get the stepper state
    pub fn state(&self) -> State {
        self.state
    }

    pub fn position(&self) -> i32 {
        if let State::Running { dir, .. } = self.state {
            self.calc_position(dir).1
        } else {
            self.position
        }
    }

    /// Move to given position. Note that no new move commands will be accepted while stepper is
    /// running. However, other target parameter, target speed, could be changed any time.
    pub fn thread_start(
        &mut self,
        target: i32,
        steps_per_thread: u32,
        phase: u16,
        estimated_rpm: u32,
    ) -> Result<(), StepperError> {
        if self.state != State::Stopped {
            return Err(StepperError::NotStopped);
        }

        self.threads
            .setup_thread_cutting(target, steps_per_thread, phase, estimated_rpm)?;
        let target_speed = self.threads.calculate_speed(estimated_rpm, 0);
        self.stepgen.set_target_speed(target_speed)?;
        self.state = State::ThreadStart;
        Ok(())
    }

    /// Synchronize stepper driver with the spindle rotation
    pub fn spindle_sync(&mut self, rpm: u32) {
        match self.state {
            State::ThreadStart => {
                // Initiate timer-driven delay for thread cutting
                self.driver.set_timer_output(false);
                let delay = self.threads.next_wait_delay();
                self.state = State::ThreadDelay;
                self.driver.start(delay);

                // Preload the next delay
                match self.threads.next_wait_delay() {
                    0 => self.driver.set_last(),
                    delay => self.driver.preload_delay(delay),
                }
            }
            State::Running {
                is_cutting_thread, ..
            } if is_cutting_thread => {
                let step = self.stepgen.current_step();
                let steps_since_start = step - self.base_step;
                let target_speed = self.threads.calculate_speed(rpm, steps_since_start);
                self.stepgen.set_target_speed(target_speed).unwrap();
            }

            _ => {
                // Not cutting threads, do nothing
            }
        }
    }

    /// Get last out-of-phase error for thread cutting
    pub fn last_error_degrees(&self) -> i32 {
        self.threads.last_error_degrees()
    }
}
