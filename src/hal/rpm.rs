
pub trait Sensor {
    /// Get latest captured RPM, in 24.8 format
    fn rpm(&self) -> u32;

    /// Check for pending interrupt and handle it (reset pending flag). Returns `true` if interrupt
    /// was pending.
    fn interrupt(&mut self) -> bool;
}