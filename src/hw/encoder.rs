use stm32f103xx::{Gpioa, Tim3, Rcc};

pub struct Encoder {}

impl Encoder {
    pub const fn new() -> Encoder {
        Encoder {}
    }

    pub fn init(&self, tim3: &Tim3, gpioa: &Gpioa, rcc: &Rcc) {
        rcc.apb1enr.modify(|_, w| w.tim3en().enabled());
        rcc.apb2enr.modify(|_, w| w.iopaen().enabled());
        rcc.apb2enr.modify(|_, w| w.afioen().enabled()); // FIXME: Do we need this?


        // CNF of '1' is floating input
        gpioa.crl.modify(|_, w| w
            .mode5().input().cnf5().bits(1)
            .mode6().input().cnf6().bits(1)
            .mode7().input().cnf7().bits(1));

        // Configure timer
        // Configure timer as rotary encoder

        tim3.smcr.write(|w| w.sms().encoder_ti2());

        // Count on rising edges
        tim3.ccer.write(|w| w.cc1p().clear().cc2p().clear());

        tim3.ccmr1_output.write(|w| unsafe {
            w.bits((0b1111 << 4 /* Input capture 1 filter */) |
                (0b1111 << 12 /* Input capture 2 filter */))
        });

        tim3.cr1.write(|w| w.cen().enabled());
    }

    pub fn set_limit(&self, tim3: &Tim3, limit : u16) {
        tim3.arr.write(|w| w.arr().bits((limit * 2) - 1));
    }

    pub fn current(&self, tim3: &Tim3) -> u16{
        tim3.cnt.read().cnt().bits() / 2
    }

    pub fn set_current(&self, tim3: &Tim3, pos: u16) {
        tim3.cnt.write(|w| w.cnt().bits(pos * 2));
    }
}