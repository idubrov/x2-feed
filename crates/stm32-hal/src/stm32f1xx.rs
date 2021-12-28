use crate::gpio::{PinT, Port};
use core::ops::Deref;
#[cfg(feature = "stm32f103")]
use stm32f1::stm32f103::gpioa::RegisterBlock;
use vcell::VolatileCell;

const PERIPHERALS_BASE: usize = 0x4000_0000;
const PERIPHERALS_ALIAS: usize = 0x4200_0000;

unsafe fn to_bitband_address<S, T>(port: *const T) -> &'static S {
    let byte_offset = (port as usize) - PERIPHERALS_BASE;
    let address = PERIPHERALS_ALIAS + byte_offset * 32;
    let ptr = address as *const S;
    &*ptr
}

// Common features for STM32F1/STM32W1 series.

/// Pin configuration registers for STM32F1/STM32W1
pub struct GPIOBitbandPinConfigBits {
    mode_low: VolatileCell<u32>,
    mode_high: VolatileCell<u32>,
    cnf_low: VolatileCell<u32>,
    cnf_high: VolatileCell<u32>,
}

impl GPIOBitbandPinConfigBits {
    /// Input mode (reset state)
    pub fn input(&self) -> &Self {
        self.mode_low.set(0);
        self.mode_high.set(0);
        self
    }

    /// Output mode, max speed 2 MHz.
    pub fn output2(&self) -> &Self {
        self.mode_low.set(0);
        self.mode_high.set(1);
        self
    }

    /// Output mode, max speed 10 MHz.
    pub fn output10(&self) -> &Self {
        self.mode_low.set(1);
        self.mode_high.set(0);
        self
    }

    /// Output mode, max speed 50 MHz.
    pub fn output50(&self) -> &Self {
        self.mode_low.set(1);
        self.mode_high.set(1);
        self
    }

    // Output config

    /// Push-pull
    pub fn push_pull(&self) -> &Self {
        self.cnf_low.set(0);
        self
    }

    /// Open-drain
    pub fn open_drain(&self) -> &Self {
        self.cnf_low.set(1);
        self
    }

    /// General purpose
    pub fn general(&self) -> &Self {
        self.cnf_high.set(0);
        self
    }

    /// Alternate function
    pub fn alternate(&self) -> &Self {
        self.cnf_high.set(1);
        self
    }

    // Input config

    /// Analog mode
    pub fn analog(&self) -> &Self {
        self.cnf_low.set(0);
        self.cnf_high.set(0);
        self
    }

    /// Floating input (reset state)
    pub fn floating(&self) -> &Self {
        // Ordering is important: should never get reserved value of `11`
        self.cnf_high.set(0);
        self.cnf_low.set(1);
        self
    }

    /// Input with pull-up / pull-down
    pub fn pull_up_down(&self) -> &Self {
        self.cnf_low.set(0);
        self.cnf_high.set(1);
        self
    }
}

/// GPIO port configuration bits
#[repr(C)]
pub struct GPIOBitbandConfigBits {
    config: [GPIOBitbandPinConfigBits; 16],
}

impl GPIOBitbandConfigBits {
    /// Get pin configuration bits
    pub fn config(&self, pin: u8) -> &GPIOBitbandPinConfigBits {
        &self.config[usize::from(pin)]
    }
}

#[cfg(feature = "stm32f103")]
impl<T: Deref<Target = RegisterBlock>> Port for T {
    type RegisterBlock = RegisterBlock;
    fn into_bitband(self) -> [PinT<RegisterBlock>; 16] {
        let ptr: *const RegisterBlock = &*self;
        unsafe {
            [
                PinT::new(ptr, 0),
                PinT::new(ptr, 1),
                PinT::new(ptr, 2),
                PinT::new(ptr, 3),
                PinT::new(ptr, 4),
                PinT::new(ptr, 5),
                PinT::new(ptr, 6),
                PinT::new(ptr, 7),
                PinT::new(ptr, 8),
                PinT::new(ptr, 9),
                PinT::new(ptr, 10),
                PinT::new(ptr, 11),
                PinT::new(ptr, 12),
                PinT::new(ptr, 13),
                PinT::new(ptr, 14),
                PinT::new(ptr, 15),
            ]
        }
    }
}

impl PinT<RegisterBlock> {
    /// Set corresponding output GPIO pin to "on".
    pub fn on(&self) {
        self.write(true);
    }

    /// Set corresponding output GPIO pin to "off".
    pub fn off(&self) {
        self.write(false);
    }

    /// Set corresponding output GPIO pin.
    pub fn write(&self, bit: bool) {
        // Set '1's and clear '0's
        let offset = if bit { self.pin } else { self.pin + 16 };
        self.register()
            .bsrr
            .write(|w| unsafe { w.bits(1 << offset) });
    }

    /// Read corresponding output GPIO pin.
    pub fn read(&self) -> bool {
        (((self.register().idr.read().bits() >> self.pin) as u16) & 1) != 0
    }

    /// Get access to the configuration registers for a given pin.
    pub fn config(&self) -> &GPIOBitbandPinConfigBits {
        let registers: &GPIOBitbandConfigBits = unsafe { to_bitband_address(self.port) };
        &registers.config(self.pin)
    }

    fn register(&self) -> &RegisterBlock {
        unsafe { &*self.port }
    }
}
