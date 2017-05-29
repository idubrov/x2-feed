use stm32f103xx::Syst;
use stepgen;

use hw::driver::Driver;

pub struct Stepper {
    // If should reverse direction signal
    reverse: bool,

    // Current state
    stepgen: stepgen::Stepgen,
    direction: bool,

    base_step: u32,
    position: i32,

    // Stop signal
    // FIXME: atomic/refcell?
    stop_requested: bool,
}

impl Stepper {
    pub const fn new() -> Stepper {
        Stepper {
            reverse: true,
            stepgen: stepgen::Stepgen::new(::hw::TICK_FREQUENCY),
            direction: true,
            base_step: 0,
            position: 0,
            stop_requested: false,
        }
    }

    /// Set new acceleration (steps per second per second), in 24.8 format.
    pub fn set_acceleration(&mut self, acceleration: u32) {
        self.stepgen.set_acceleration(acceleration)
    }

    /// Set slew speed (maximum speed stepper motor would run). Note that stepper motor would only
    /// reach this speed if target step is far enough, so there is enough time for deceleration.
    /// FIXME: no Javadoc!
    /// @param speed target slew speed to reach, in steps per second, 24.8 format
    pub fn set_speed(&mut self, speed: u32) {
        self.stepgen.set_target_speed(speed);
    }

    fn load_delay(&mut self, driver: &Driver) -> u32 {
        let delay = self.stepgen.next();
        if delay != 0 {
            // Load new step into ARR, start pulse at the end
            let d = (delay + 128) >> 8; // Delay is in 16.8 format
            driver.set_delay(d as u16);
        } else {
            // FIXME: do we need this branch?
            // Load idle values. This is important to do on the last update
            // when timer is switched into one-pulse mode.
            driver.set_delay(1 /* FIXME: IDLE delay */);
        }
        delay
    }

    pub fn step_completed(&mut self, driver: &Driver) {
        if driver.check_stopped() {
            driver.disable();
            return;
        }

        if self.stop_requested {
            self.stepgen.set_target_step(0);
            self.stop_requested = false;
        }

        if self.load_delay(driver) == 0 {
            // Stop on the next update, one pulse mode
            // Note that load_delay() should have already loaded ARR and
            // CCR1 with idle values.
            driver.set_last_pulse();
        }
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

    fn set_direction(&mut self, syst: &Syst, driver: &Driver, dir: bool) {
        ::hw::delay::ns(syst, ::hw::DIR_SETUP_NS);
        driver.set_direction(if self.reverse { dir } else { !dir });
        self.direction = dir;
        ::hw::delay::ns(syst, ::hw::DIR_HOLD_NS);
    }

    /// Move to given position. Note that no new move commands will be accepted while stepper is
    /// running. However, other target parameter, target speed, could be changed any time.
    /// FIXME: technically, we can actually change target on the fly...
    pub fn move_to(&mut self, syst: &Syst, driver: &Driver, target: i32) -> bool {
        if !driver.check_stopped() {
            return false;
        }

        let pos = self.update_position();
        let delta;
        if pos < target {
            delta = (target - pos) as u32;
            self.set_direction(syst, driver, true);
        } else if pos > target {
            delta = (pos - target) as u32;
            self.set_direction(syst, driver, false);
        } else {
            // Nothing to do!
            return true;
        }
        self.stepgen.set_target_step(self.base_step + delta);
        self.stop_requested = false;

        // Load first delay into ARR/CC1, if not stopped
        if self.load_delay(driver) == 0 {
            // Not making even a single step
            return false;
        }

        // FIXME: should be configurable (disable/not disable outputs at the end of the move)...
        // Enable driver outputs
        driver.enable();
        driver.reload_timer();

        // Load second delay into ARR & CC1.
        let is_last = self.load_delay(driver) == 0;

        // Start pulse generation
        driver.start(is_last);

        return true;
    }


    pub fn stop(&mut self) {
        self.stop_requested = true;
    }

    pub fn is_stopped(&self, driver: &Driver) -> bool {
        driver.check_stopped()
    }
}