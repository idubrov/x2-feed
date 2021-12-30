
pub struct ThreadInfo {
  /// Frequency of the timer we use for delay
  timer_freq: u32,
  /// Acceleration
  acceleration: u32,
  /// Remaining ticks to wait before we start cutting the thread
  delay_remaining: u32,
  /// How many stepper steps per step do we need to make
  steps_per_thread: u32,
  /// Target to stop when cutting threads
  target: i32,
  /// Thread cutting phase error (how many steps we were off on the last spindle event)
  last_error: u32,
}

impl ThreadInfo {
  pub fn new(timer_freq: u32) -> ThreadInfo {
    ThreadInfo {
      timer_freq,
      acceleration: 0,
      delay_remaining: 0,
      steps_per_thread: 0,
      target: 0,
      last_error: 0,
    }
  }

  /// Configure acceleration of the stepper motor. Used to calculate an initial delay when cutting
  /// threads.
  pub fn set_acceleration(&mut self, acceleration: u32) {
    self.acceleration = acceleration;
  }

  // FIXME: unwraps...

  /// Calculate the time we need to wait before starting to accelerate stepper motor to make stepper
  /// to run in sync with the spindle. This delay is calculated so our "out-of-phase error" is
  /// minimal once stepper is fully accelerated.
  pub fn setup_thread_cutting(&mut self, steps_per_thread: u32, target: i32, estimated_rpm: u32) -> Result<(), stepgen::Error> {
    self.steps_per_thread = steps_per_thread;
    self.target = target;
    let mut stepgen: stepgen::Stepgen = stepgen::Stepgen::new(self.timer_freq);
    // RPM is in 24.8 already
    let speed = estimated_rpm / 60 * steps_per_thread;
    stepgen.set_acceleration(self.acceleration)?;
    stepgen.set_target_speed(speed)?;
    stepgen.set_target_step(u32::MAX)?;
    // FIXME: limit in case we have an error in algorithm?...
    while !stepgen.is_at_speed() {
      let _delay = stepgen.next().unwrap();
    }
    let steps_to_accelerate = stepgen.current_step();

    let revolutions_to_accelerate = (steps_to_accelerate / steps_per_thread) + 1;
    let start_at_step = revolutions_to_accelerate * steps_per_thread - steps_to_accelerate;

    // Amount of driver timer ticks we need to wait before starting thread cutting. This is to time
    // our acceleration such that out-of-phase error is minimal once fully accelerated. Out-of-phase
    // error is amount of steps we are away from our projected thread. We assume thread begins at
    // spindle event at the starting position (so, at each spindle event the amount of steps taken
    // so far should be a multiple of `steps_per_thread`).
    // 60 seconds (RPM to revolutions per second)
    // 256 is RPM divider (RPM is in 24.8 format)
    self.delay_remaining = (60
      * 256
      * u64::from(self.timer_freq)
      * u64::from(start_at_step)
      / u64::from(steps_per_thread * estimated_rpm)) as u32;

    // We added +1 to revolutions_to_accelerate, so our delay should never be too short.
    assert!(self.delay_remaining > 10_000, "should never wait too short");
  }

  /// Calculate next delay for our waiting timer.
  pub fn next_wait_delay(&mut self) -> u16 {
    if self.delay_remaining < u32::from(u16::MAX) {
      // We can wait for the whole period in one go
      core::mem::replace(&mut self.delay_remaining, 0) as u16
    } else {
      // Wait for half of the period or longest delay, whichever is smaller.
      let first_delay = (self.delay_remaining / 2).min(u32::from(u16::MAX));
      self.delay_remaining -= first_delay;
      first_delay as u16
    }
  }

  /// Calculate target speed based on current RPM and error. Should be called at a spindle event.
  /// Speed is adjusted to reduce error to 0. Error is calculated as amount of phase we are off from
  /// desired thread location.
  pub fn calculate_speed(&mut self, rpm: u32, steps_since_start: u32) -> u32 {
    let target_speed = rpm * self.steps_per_thread / 60;
    // FIXME: adjust speed by error once accelerated!
    self.last_error = steps_since_start % self.steps_per_thread;
    target_speed
  }

  /// Last thread cutting error, in degrees.
  pub fn last_error_degrees(&self) -> i32 {
    let bias = self.steps_per_thread / 2;
    match (360 * self.last_error + bias) / self.steps_per_thread {
      degree if degree > 180 => (degree as i32) - 360,
      degree => degree as i32,
    }
  }
}
