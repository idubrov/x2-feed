extern crate stm32f103xx;

use core::ops::Deref;
use core::marker::PhantomData;

pub struct PinRange<T> where T: Deref<Target=stm32f103xx::gpioa::RegisterBlock> + 'static {
    phantom: PhantomData<*const T>,
    pub shift: u8,
    pub mask: u16,
}


unsafe impl<T> Sync for PinRange<T> where T: Deref<Target=stm32f103xx::gpioa::RegisterBlock> + 'static
{}

impl<T> PinRange<T>
    where T: Deref<Target=stm32f103xx::gpioa::RegisterBlock> + 'static {
    pub const fn new(shift: u8, count: u8) -> Self {
        PinRange {
            phantom: PhantomData,
            shift: shift,
            mask: (1 << count) - 1,
        }
    }

    pub fn set(&self, port: &T, data: u16) {
        let bits = u32::from(data & self.mask) | // Set '1's
            (u32::from(!data & self.mask) << 16); // Clear '0's
        port.bsrr.write(|w| unsafe { w.bits(bits << self.shift) });
    }

    pub fn get(&self, port: &T) -> u16 {
        ((port.idr.read().bits() >> self.shift) as u16) & self.mask
    }
}
