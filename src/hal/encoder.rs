pub trait Encoder {
    /// Set rotary encoder limit.
    fn set_limit(&mut self, limit : u16);

    /// Get current value of the rotary encoder.
    fn current(&self) -> u16;

    /// Set current value of the rotary encoder.
    fn set_current(&mut self, pos: u16);
}