use stm32f1::stm32f103::TIM3;
use stm32f1xx_hal::gpio::{ErasedPin, Floating, Input};

type Pin = ErasedPin<Input<Floating>>;

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
        // Configure timer
        // Configure timer as rotary encoder
        // FIXME: was sms().encoder_ti2()
        self.tim3.smcr.write(|w| w.sms().encoder_mode_3());

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

    /// Set rotary encoder limit. Note that this function is "unsafe" because it changes the
    /// configuration of the encoder without resetting it back.
    pub fn set_limit_unsafe(&mut self, limit: u16) {
        self.tim3.arr.write(|w| w.arr().bits((limit * 4) - 1));
    }

    /// Get current value of the rotary encoder.
    pub fn current(&self) -> u16 {
        self.tim3.cnt.read().cnt().bits() / 4
    }

    /// Set current value of the rotary encoder.
    pub fn set_current(&mut self, pos: u16) {
        self.tim3.cnt.write(|w| w.cnt().bits(pos * 4 + 1));
    }

    /// Set `limit` and `current` value temporarily. Once return value is dropped, encoder is
    /// reset back to its original settings.
    pub fn set_current_limit(&mut self, current: u16, limit: u16) -> QuadEncoderWithSettings {
        let (old_limit, old_current) = (self.get_limit(), self.current());
        self.set_limit_unsafe(limit);
        self.set_current(current);
        QuadEncoderWithSettings {
            limit: old_limit,
            current: old_current,
            encoder: self,
        }
    }

    pub fn delta_encoder(&mut self) -> EncoderDelta {
        EncoderDelta::new(self)
    }
}

pub struct QuadEncoderWithSettings<'a> {
    limit: u16,
    current: u16,
    encoder: &'a mut QuadEncoder,
}

impl<'a> core::ops::Deref for QuadEncoderWithSettings<'a> {
    type Target = QuadEncoder;

    fn deref(&self) -> &QuadEncoder {
        self.encoder
    }
}

impl<'a> core::ops::DerefMut for QuadEncoderWithSettings<'a> {
    fn deref_mut(&mut self) -> &mut QuadEncoder {
        self.encoder
    }
}

impl<'a> Drop for QuadEncoderWithSettings<'a> {
    fn drop(&mut self) {
        self.encoder.set_limit_unsafe(self.limit);
        self.encoder.set_current(self.current);
    }
}

// Any reasonably big number to make sure you cannot crank half of it on the encoder between 'ticks'
const LIMIT: u16 = 20_000;

/// Helper structure to use encoder as encoder producing "deltas".
pub struct EncoderDelta<'a> {
    last: u16,
    encoder: QuadEncoderWithSettings<'a>,
}

impl<'a> EncoderDelta<'a> {
    fn new(encoder: &'a mut QuadEncoder) -> Self {
        Self {
            last: LIMIT / 2,
            encoder: encoder.set_current_limit(LIMIT / 2, LIMIT),
        }
    }

    pub fn delta(&mut self) -> i16 {
        let current = self.encoder.current();
        // Substract unsigned wrapping around LIMIT
        let delta = if current < self.last {
            current + LIMIT - self.last
        } else {
            current - self.last
        };
        self.last = current;
        // Convert delta to signed -LIMIT/2 to LIMIT/2
        if delta < LIMIT / 2 {
            delta as i16
        } else {
            (delta as i16) - LIMIT as i16
        }
    }
}
