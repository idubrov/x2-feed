pub trait DriverControl {
    /// Enable/disable driver outputs.
    fn enable(&mut self, enable: bool);

    /// Set stepper driver direction.
    fn direction(&mut self, bit: bool);
}

pub trait PulseGen {
    /// Enable PWM generating stepper motor pulses.
    /// `first_delay` is the first delay to load in the timer. Pulse generation starts immediately.
    fn start(&mut self, first_delay: u16);

    /// Preload delay for the next step into the pulse generator. This delay will be used once
    /// current step completes.
    fn preload_delay(&mut self, delay: u16);

    /// Indicate that no new delay is available, should stop once current step completes.
    fn set_last(&mut self);

    fn is_running(&self) -> bool;
}

pub trait Driver: DriverControl+PulseGen {}
impl<T> Driver for T where T: PulseGen+DriverControl {}