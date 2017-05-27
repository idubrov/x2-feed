use stm32f103xx::{Gpioa, Rcc};

pub struct Led {}

impl Led {
    pub const fn new() -> Led {
        Led {}
    }

    pub fn init(&self, gpioa: &Gpioa, rcc: &Rcc) {
        rcc.apb2enr.modify(|_, w| w.iopaen().enabled());

        // PA4 is LED
        gpioa.crl.modify(|_, w| w.cnf4().open().mode4().output2());
        ::hw::LED.set(gpioa, 1); // Turn it off
    }

    pub fn set<'a>(&self, gpioa: &'a Gpioa, on: bool) {
        ::hw::LED.set(gpioa, if on { 0 } else { 1 }); // '0' is on
    }
}