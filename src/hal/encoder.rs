use bare_metal::Peripheral;
use stm32f103xx::{gpioa, TIM3};
use core::ops::Deref;
use stm32_extras::GPIOExtras;

pub trait QuadEncoder {
    /// Set rotary encoder limit.
    fn set_limit(&mut self, limit : u16);

    /// Get current value of the rotary encoder.
    fn current(&self) -> u16;

    /// Set current value of the rotary encoder.
    fn set_current(&mut self, pos: u16);
}

pub struct QuadEncoderImpl<Port: 'static> {
    port: Peripheral<Port>,
    btn: usize,
    dt: usize,
    clk: usize
}
unsafe impl <Port> Send for QuadEncoderImpl<Port> { }

impl <Port> QuadEncoderImpl<Port> where Port: Deref<Target = gpioa::RegisterBlock> {
    pub const fn new(port: Peripheral<Port>, btn: usize, dt: usize, clk: usize) -> QuadEncoderImpl<Port> {
        QuadEncoderImpl { port, btn, dt, clk }
    }

    // Note that we require an explicit ownership of I/O port peripheral to guard against
    // concurrent access when we modify shared register of the peripheral (CRL)
    pub fn init(&self) {
        let tim3 = self.unsafe_timer();

        let port = self.port();
        port.pin_config(self.btn).floating();
        port.pin_config(self.dt).floating();
        port.pin_config(self.clk).floating();

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

    fn port(&self) -> &Port {
        unsafe { &*self.port.get() }
    }
}

impl <Port> QuadEncoder for QuadEncoderImpl<Port> where Port: Deref<Target = gpioa::RegisterBlock> {

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