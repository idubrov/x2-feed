#![allow(dead_code)]
use stm32f103xx::{GPIOA, RCC};
use hw::config::LED;

pub struct Led;

impl Led {
    pub fn init(&self, gpioa: &GPIOA, rcc: &RCC) {
        rcc.apb2enr.modify(|_, w| w.iopaen().enabled());

        // PA4 is LED
        gpioa.crl.modify(|_, w| w.cnf4().open().mode4().output2());
        LED.set(gpioa, 1); // Turn it off
    }

    pub fn set<'a>(&self, gpioa: &'a GPIOA, on: bool) {
        LED.set(gpioa, if on { 0 } else { 1 }); // '0' is on
    }
}