extern crate stm32f103xx;
extern crate cortex_m;

use stm32f103xx::{Rcc, tim2};
use core::ops::Deref;
use core::marker::PhantomData;

pub struct Timer<T> where T: Deref<Target=tim2::RegisterBlock> + 'static {
    phantom: PhantomData<*const T>,
}

unsafe impl<T> Sync for Timer<T> where T: Deref<Target=tim2::RegisterBlock> + 'static
{}

impl<T> Timer<T> where T: Deref<Target=tim2::RegisterBlock> + 'static {

    pub const fn new() -> Self {
        Timer {
            phantom: PhantomData
        }
    }

    pub fn init(&self, tim: &T, rcc: &Rcc) {
        rcc.apb1enr.modify(|_, w| w.tim2en().enabled()); // Enable the peripheral
        rcc.apb1enr.modify(|_, w| w.tim4en().enabled()); // Enable the peripheral

        // Assuming 72Mhz, configure at 1Hz
        let psc = 10_000 - 1;
        tim.psc.write(|w| w.psc().bits(psc));
        let arr = 7_200 - 1;
        tim.arr.write(|w| w.arr().bits(arr));
        tim.cr1.write(|w| w.opm().continuous());
        tim.cr1.modify(|_, w| w.cen().enabled());
        tim.dier.write(|w| w.uie().set());
    }

    pub fn check_pending(&self, tim: &T) -> bool {
        if tim.sr.read().uif().is_pending() {
            tim.sr.modify(|_, w| w.uif().clear());
            true
        } else {
            false
        }
    }
}
