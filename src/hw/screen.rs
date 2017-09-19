extern crate lcd;

use hw::config::{RS, RW, E, DATA};
use stm32f103xx::{GPIOB, SYST, RCC};

pub struct Screen;

impl Screen {
    pub fn init(&self, gpiob: &GPIOB, rcc: &RCC) {
        rcc.apb2enr.modify(|_, w| w.iopben().enabled());

        // PB1 is RS
        gpiob.crl.modify(|_, w| w.cnf1().push().mode1().output());
        gpiob.crh.modify(|_, w| w
            // PB10 is R/W
            .cnf10().push().mode10().output()
            // PB11 is E
            .cnf11().push().mode11().output()
            // PB12-PB15 are DB4-DB7
            .cnf12().push().mode12().output()
            .cnf13().push().mode13().output()
            .cnf14().push().mode14().output()
            .cnf15().push().mode15().output());

        // R/W is always 0 -- we don't use wait flag
        RW.set(gpiob, 0);
    }

    pub fn materialize<'a>(&self, syst: &'a SYST, gpiob: &'a GPIOB) -> lcd::Display<ScreenHAL<'a>> {
        lcd::Display::new(ScreenHAL {
            syst,
            gpiob,
        })
    }
}

/// Binding of HD44780 instance to real hardware
pub struct ScreenHAL<'a> {
    syst: &'a SYST,
    gpiob: &'a GPIOB,
}

impl<'a> lcd::Hardware for ScreenHAL<'a> {
    fn rs(&self, bit: bool) {
        RS.set(self.gpiob, if bit { 1 } else { 0 });
    }

    fn enable(&self, bit: bool) {
        E.set(self.gpiob, if bit { 1 } else { 0 });
    }

    fn data(&self, data: u8) {
        DATA.set(self.gpiob, u16::from(data));
    }
}

impl<'a> lcd::Delay for ScreenHAL<'a> {
    fn delay_us(&self, delay_usec: u32) {
        ::hw::delay::us(self.syst, delay_usec);
    }
}