use stm32f103xx::{Tim2, Gpioa, Rcc};

const MIN_PERIOD: u32 = 60 * ::hw::HALL_TICK_FREQUENCY / ::hw::HALL_MAX_RPM;
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

    pub fn init(&mut self, tim2: &Tim2, gpioa: &Gpioa, rcc: &Rcc) {
        rcc.apb2enr.modify(|_, w| w.iopaen().enabled());
        rcc.apb1enr.modify(|_, w| w.tim2en().enabled());

        gpioa.crl.modify(|_, w| w
            // Input with pull-up/pull-down
            .mode0().input().cnf0().bits(0b10)
        );
        gpioa.odr.modify(|_, w| w.odr0().set());


        tim2.psc.write(|w| w.psc().bits(((::hw::FREQUENCY / ::hw::HALL_TICK_FREQUENCY) - 1) as u16));
        tim2.arr.write(|w| w.arr().bits(0xffffu16));

        let ccmr1 =
            0 |
                // CC1 channel is configured as input, IC1 is mapped on TI1
                (0b01 << 0) |
                // Prescaler: no prescaler
                (0b00 << 2) |
                // Filter fSAMPLING=fDTS/32,N=8
                (0b1111 << 4);
        tim2.ccmr1_output.write(|w| unsafe { w.bits(ccmr1) });

        // Slave mode -- reset on capture
        tim2.ccer.write(|w| w
            // Trigger polarity: falling
            .cc1p().set()
            // Enable channel
            .cc1e().set());

        // FIXME: make trigger safe!
        tim2.smcr.write(|w| w
            .sms().reset()
            // Filtered Timer Input 1 (TI1FP1)
            .ts().ti1fp1());

        tim2.dier.write(|w| w
            .uie().set()
            .cc1ie().set());

        tim2.cr1.write(|w| w
            .ckd().div1()
            .dir().up()
            // Only counter overflow/underflow generates an update interrupt
            // reload_timer() should not generate an event.
            .urs().set()
            .cen().enabled());
    }

    pub fn interrupt(&mut self, tim2: &Tim2) {
        if tim2.sr.read().cc1if().is_set() {
            tim2.sr.modify(|_, w| w.cc1if().clear());

            let lsb = tim2.ccr1.read().bits();
            // Captured value is very low and overflow is pending -- need to account for "msb" increment, as it happened
            // before the capture event. Otherwise, we don't care -- if overflow interrupt is pending, we will handle
            // it on the next handler invocation.
            if lsb < MIN_PERIOD && tim2.sr.read().uif().is_pending() {
                tim2.sr.modify(|_, w| w.uif().clear());
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
            // FIXME: if too long, set RPM to zero

            // Reset RPM if MSB is too high
            if self.msb >= MAX_MSB {
                self.captured = 0;
            }
        }
    }

    /// Get latest captured RMP, in 24.8 format
    pub fn rpm(&self) -> u32 {
        if self.captured != 0 { (60 * ::hw::HALL_TICK_FREQUENCY << 8) / self.captured } else { 0 }
    }
}