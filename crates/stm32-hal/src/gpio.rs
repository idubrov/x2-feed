/// Individual pin access with bitband-driven reconfiguration
pub struct PinT<R> {
    pub(crate) port: *const R,
    pub(crate) pin: u8,
}

unsafe impl<R> Send for PinT<R> {}

impl<R> PinT<R> {
    /// Safety: port pointer must be valid and should have an exclusive ownership of a given pin.
    pub(crate) unsafe fn new(port: *const R, pin: u8) -> PinT<R> {
        PinT { port, pin }
    }
}

/// Convenient access to the individual pins on GPIO ports. Implementation uses bitbanding to
/// allow reconfiguring pins without taking any locks.
pub trait Port {
    /// Type of the associated register block
    type RegisterBlock;
    /// Turn port into 16 individual pins accessible via bitbanding.
    fn into_bitband(self) -> [PinT<Self::RegisterBlock>; 16];
}

/// GPIO pin type
#[cfg(feature = "stm32f103")]
pub type Pin = PinT<stm32f1::stm32f103::gpioa::RegisterBlock>;
