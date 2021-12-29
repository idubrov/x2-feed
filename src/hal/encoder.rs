use stm32_hal::gpio::Pin;
use stm32f1::stm32f103::TIM3;

pub struct QuadEncoder {
    tim3: TIM3,
    dt: Pin,
    clk: Pin,
}

impl QuadEncoder {
    pub fn new(tim3: TIM3, dt: Pin, clk: Pin) -> QuadEncoder {
        let encoder = QuadEncoder { tim3, dt, clk };
        encoder.init();
        encoder
    }

    // Note that we require an explicit ownership of I/O port peripheral to guard against
    // concurrent access when we modify shared register of the peripheral (CRL)
    fn init(&self) {
        self.dt.config().floating();
        self.clk.config().floating();

        // Configure timer
        // Configure timer as rotary encoder
        // FIXME: was sms().encoder_ti2()
        self.tim3.smcr.write(|w| w.sms().encoder_mode_2());

        // Count on rising edges
        self.tim3
            .ccer
            .write(|w| w.cc1p().clear_bit().cc2p().clear_bit());

        // FIXME: verify unsafe, maybe can do safe?
        self.tim3.ccmr1_output().write(|w| unsafe {
            w.bits(
                (0b1111 << 4/* Input capture 1 filter */)
                    | (0b1111 << 12/* Input capture 2 filter */),
            )
        });

        self.tim3.cr1.write(|w| w.cen().enabled());
    }

    /// Get rotary encoder limit.
    pub fn get_limit(&self) -> u16 {
        (self.tim3.arr.read().arr().bits() + 1) / 2
    }

    /// Set rotary encoder limit.
    pub fn set_limit(&mut self, limit: u16) {
        self.tim3.arr.write(|w| w.arr().bits((limit * 2) - 1));
    }

    /// Get current value of the rotary encoder.
    pub fn current(&self) -> u16 {
        self.tim3.cnt.read().cnt().bits() / 2
    }

    /// Set current value of the rotary encoder.
    pub fn set_current(&mut self, pos: u16) {
        self.tim3.cnt.write(|w| w.cnt().bits(pos * 2));
    }
}
