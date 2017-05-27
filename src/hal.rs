extern crate hd44780;

use stm32f103xx::{Gpiob, Syst, SYST, Rcc};

// FIXME: 1.8sec is about maximum
pub fn delay_us(delay: u32) {
    let syst = unsafe { &(*SYST.get()) };
    // SysTick is 1/8 AHB (9Mhz)
    syst.set_reload(9 * delay - 1);
    syst.clear_current();
    while !syst.has_wrapped() {}
}

pub struct Display<'a> {
    gpiob: &'a Gpiob
}

impl<'a> Display<'a> {
    pub fn new(gpiob: &Gpiob) -> Display {
        Display {
            gpiob
        }
    }

    pub fn init(&self, rcc: &Rcc) {
        rcc.apb2enr.modify(|_, w| w.iopben().enabled());

        // PB1 is RS
        self.gpiob.crl.modify(|_, w| w.cnf1().push().mode1().output());
        self.gpiob.crh.modify(|_, w| w
            // PB10 is R/W
            .cnf10().push().mode10().output()
            // PB11 is E
            .cnf11().push().mode11().output()
            // Data bits (PB12-PB15)
            .cnf12().push().mode12().output()
            .cnf13().push().mode13().output()
            .cnf14().push().mode14().output()
            .cnf15().push().mode15().output());

        // R/W is always 0
        self.gpiob.brr.write(|w| w.br10().set());

        // Need to wait at least 40ms after Vcc rises to 2.7V
        // STM32 could start much earlier than that
        delay_us(50000);
    }
}

impl<'a> hd44780::Hardware for Display<'a> {
    fn rs(&self, bit: bool) {
        if bit {
            self.gpiob.bsrr.write(|w| w.bs1().set());
        } else {
            self.gpiob.brr.write(|w| w.br1().set());
        }
    }

    fn enable(&self, bit: bool) {
        if bit {
            self.gpiob.bsrr.write(|w| w.bs11().set());
        } else {
            self.gpiob.brr.write(|w| w.br11().set());
        }
    }

    fn data(&self, data: u8) {
        self.gpiob.bsrr.write(|w| unsafe { w.bits(((data & 0xf) as u32) << 12) }); // Set '1's
        self.gpiob.brr.write(|w| unsafe { w.bits((((!data) & 0xf) as u32) << 12)}); // Clear '0's
    }

    fn delay_us(&self, delay: u32) {
        self::delay_us(delay);
    }
}