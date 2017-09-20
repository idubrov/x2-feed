pub trait Driver {
    // Control aspect of stepper motor driver (setting directions, enabling/disabling outputs).

    /// Enable/disable driver outputs.
    fn enable(&mut self, enable: bool);

    /// Set stepper driver direction.
    fn direction(&mut self, bit: bool);

    // Pulse generating aspect of stepper motor driver.

    /// Enable PWM generating stepper motor pulses.
    /// `first_delay` is the first delay to load in the timer. Pulse generation starts immediately.
    fn start(&mut self, first_delay: u16);

    /// Preload delay for the next step into the pulse generator. This delay will be used once
    /// current step completes.
    fn preload_delay(&mut self, delay: u16);

    /// Indicate that no new delay is available, should stop once current step completes.
    fn set_last(&mut self);

    /// Returns `true` if timer generating pulses is running, `false` otherwise.
    fn is_running(&self) -> bool;

    /// Check for pending interrupt and handle it (reset pending flag). Returns `true` if interrupt
    /// was pending.
    fn interrupt<'a>(&mut self) -> bool;
}
