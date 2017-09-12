use stm32f103xx::{TIM2, GPIOA, RCC};

// Compute what could be the shortest period between two hall sensor triggers (fastest RPM).
// Used to filter the noise and also to account for the case when timer overflowed just before
// the capture.
const MIN_PERIOD: u32 = 60 * ::hw::HALL_TICK_FREQUENCY / ::hw::HALL_MAX_RPM;

// Compute what could be the longest period between two hall sensor triggers (slowest RPM) and take
// its high 16 bits. If computed period is longer than that, it is assumed that spindle is stopped.
const MAX_MSB: u32 = ((60 * ::hw::HALL_TICK_FREQUENCY) / ::hw::HALL_MIN_RPM + 0xffffu32) >> 16;

pub struct Hall {
    captured: u32,
    msb: u32,
}

impl Hall {
    pub const fn new() -> Hall {
        Hall {
            captured: 0,
            msb: 0,
        }
    }

    pub fn init(&mut self, tim2: &TIM2, gpioa: &GPIOA, rcc: &RCC) {
        rcc.apb2enr.modify(|_, w| w.iopaen().enabled());
        rcc.apb1enr.modify(|_, w| w.tim2en().enabled());

        gpioa.crl.modify(|_, w| w
            // Input with pull-up/pull-down
            .mode0().input().cnf0().bits(0b10)
        );
        gpioa.odr.modify(|_, w| w.odr0().set_bit());

        tim2.psc.write(|w| w.psc().bits(((::hw::FREQUENCY / ::hw::HALL_TICK_FREQUENCY) - 1) as u16));
        tim2.arr.write(|w| w.arr().bits(0xffffu16));

        // Only output register is supported, see https://github.com/japaric/svd2rust/issues/16
        // Also, enumerations are not defined for the input register in SVD file.
        let ccmr1 =
                // CC1 channel is configured as input, IC1 is mapped on TI1
                0b01 |
                // Prescaler: no prescaler
                // (0b00 << 2) |
                // Filter fSAMPLING=fDTS/32,N=8
                (0b1111 << 4);
        tim2.ccmr1_output.write(|w| unsafe { w.bits(ccmr1) });

        // Slave mode -- reset on capture
        tim2.ccer.write(|w| w
            // Trigger polarity: falling
            .cc1p().set_bit()
            // Enable channel
            .cc1e().set_bit());

        tim2.smcr.write(|w| w
            .sms().reset()
            // Filtered Timer Input 1 (TI1FP1)
            .ts().ti1fp1());

        tim2.dier.write(|w| w
            .uie().set_bit()
            .cc1ie().set_bit());

        tim2.cr1.write(|w| w
            .ckd().div1()
            .dir().up()
            // Only counter overflow/underflow generates an update interrupt
            // reload_timer() should not generate an event.
            .urs().set_bit()
            .cen().enabled());
    }

    pub fn interrupt(&mut self, tim2: &TIM2) {
        if tim2.sr.read().cc1if().bit_is_set() {
            tim2.sr.modify(|_, w| w.cc1if().clear_bit());

            let lsb = tim2.ccr1.read().bits();
            // If we have overflow event pending (it wasn't processed yet), we need to distinguish
            // two scenarios:
            // * timer overflowed just before the capture
            // * timer overflowed just after the capture
            // In the former case, we need to account the overflow event for the current period
            // (increase `msb` by one). In the latter case, we don't need to -- overflow event
            // should be accounted for the next period and this will happen on the next interrupt
            // handler invocation.
            // The way we distinguish these two is by comparing captured value against smallest
            // allowed value (`MIN_PERIOD`). If it is lower, we assume that timer overflowed just
            // before the capture event (and hence capture event is that small).
            if lsb < MIN_PERIOD && tim2.sr.read().uif().is_pending() {
                tim2.sr.modify(|_, w| w.uif().clear_bit());
                // Capture happened just after the overflow: need to increment upper "msb"
                self.msb += 1;
            }
            let captured: u32 = (self.msb << 16) | lsb;

            // Capture only if period is long enough -- ignore the noise.
            if captured >= MIN_PERIOD {
                self.captured = captured;
            }
            self.msb = 0;
        } else if tim2.sr.read().uif().is_pending() {
            tim2.sr.modify(|_, w| w.uif().clear());
            self.msb += 1;

            // Period is too long -- assume that spindle is stopped
            if self.msb >= MAX_MSB {
                self.captured = 0;
            }
        }
    }

    /// Get latest captured RPM, in 24.8 format
    pub fn rpm(&self) -> u32 {
        if self.captured != 0 { ((60 * ::hw::HALL_TICK_FREQUENCY) << 8) / self.captured } else { 0 }
    }
}