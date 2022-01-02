use stm32f1xx_hal::gpio::{ErasedPin, Input, PullUp};

pub struct EStop {
  estop: ErasedPin<Input<PullUp>>,
}

impl EStop {
  pub fn new(estop: ErasedPin<Input<PullUp>>) -> EStop {
    EStop {
      estop
    }
  }

  /// Check if we are in the emergency stop condition.
  pub fn is_emergency_stop(&self) -> bool {
    self.estop.is_low()
  }
}