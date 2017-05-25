extern crate stm32f103xx;
extern crate cortex_m;

use stm32f103xx::{Rcc};
use core::ops::Deref;
use core::marker::PhantomData;

pub struct Led<T> where T: Deref<Target=stm32f103xx::gpioa::RegisterBlock> + 'static {
    phantom: PhantomData<*const T>,
    pin: u8,
    on_is_high: bool,
}

unsafe impl<T> Sync for Led<T> where T: Deref<Target=stm32f103xx::gpioa::RegisterBlock> + 'static
{}

impl<T> Led<T>
    where T: Deref<Target=stm32f103xx::gpioa::RegisterBlock> + 'static {
    pub const fn new(pin: u8, on_is_high: bool) -> Self {
        Led {
            phantom: PhantomData,
            pin: pin,
            on_is_high: on_is_high,
        }
    }

    pub fn init(&self, port: &T, rcc: &Rcc) {
        // Power up the relevant peripherals
        // FIXME: use port...
        rcc.apb2enr.modify(|_, w| w.iopcen().enabled());

        // Configure the pin PC13 as an output pin
        // Open-drain, 2Mhz

        if self.pin < 8 {
            port.crl.modify(|_, w| unsafe { w.bits(0b0110 << (self.pin * 4)) });
        } else {
            port.crh.modify(|_, w| unsafe { w.bits(0b0110 << ((self.pin - 8) * 4)) });
        }
        port.bsrr.write(|w| w.bs13().set());
    }

    pub fn set(&self, port: &T, on: bool) {
        if on == self.on_is_high {
            port.bsrr.write(|w| w.br13().reset());
        } else {
            port.bsrr.write(|w| w.bs13().set());
        }
    }
}
