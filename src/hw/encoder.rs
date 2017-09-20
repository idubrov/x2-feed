use stm32f103xx::{GPIOA, TIM3, RCC};
use hal::QuadEncoder;

pub struct Encoder;

impl Encoder {
    // Note that we require an explicit ownership of I/O port peripheral to guard against
    // concurrent access when we modify shared register of the peripheral (CRL)
    pub fn init(&self, gpioa: &GPIOA, rcc: &RCC) {
        let tim3 = self.unsafe_timer();
        rcc.apb1enr.modify(|_, w| w.tim3en().enabled());
        rcc.apb2enr.modify(|_, w| w.iopaen().enabled());
        rcc.apb2enr.modify(|_, w| w.afioen().enabled());


        // CNF of '1' is floating input
        gpioa.crl.modify(|_, w| w
            .mode5().input().cnf5().bits(1)
            .mode6().input().cnf6().bits(1)
            .mode7().input().cnf7().bits(1));

        // Configure timer
        // Configure timer as rotary encoder

        tim3.smcr.write(|w| w.sms().encoder_ti2());

        // Count on rising edges
        tim3.ccer.write(|w| w.cc1p().clear_bit().cc2p().clear_bit());

        tim3.ccmr1_output.write(|w| unsafe {
            w.bits((0b1111 << 4 /* Input capture 1 filter */) |
                (0b1111 << 12 /* Input capture 2 filter */))
        });

        tim3.cr1.write(|w| w.cen().enabled());
    }

    fn unsafe_timer(&self) -> &'static TIM3 {
        unsafe { &*TIM3.get() }
    }
}

impl QuadEncoder for Encoder {

    fn set_limit(&mut self, limit : u16) {
        self.unsafe_timer().arr.write(|w| w.arr().bits((limit * 2) - 1));
    }

    fn current(&self) -> u16 {
        self.unsafe_timer().cnt.read().cnt().bits() / 2
    }

    fn set_current(&mut self, pos: u16) {
        self.unsafe_timer().cnt.write(|w| w.cnt().bits(pos * 2));
    }
}