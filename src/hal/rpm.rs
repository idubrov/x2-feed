use bare_metal::Peripheral;
use stm32f103xx::gpioa;
use core::ops::Deref;
use stm32_extras::GPIOExtras;

use stm32f103xx::TIM2;
use super::clock::FREQUENCY;

pub trait RpmSensor {
    /// Get latest captured RPM, in 24.8 format
    fn rpm(&self) -> u32;

    /// Check for pending interrupt and handle it (reset pending flag). Returns `true` if interrupt
    /// was pending.
    fn interrupt(&mut self) -> bool;
}

const HALL_TICK_FREQUENCY: u32 = 100_000; // 0.01 ms
const HALL_MAX_RPM: u32 = 6000;
const HALL_MIN_RPM: u32 = 50;

// Compute what could be the shortest period between two hall sensor triggers (fastest RPM).
// Used to filter the noise and also to account for the case when timer overflowed just before
// the capture.
const MIN_PERIOD: u32 = 60 * HALL_TICK_FREQUENCY / HALL_MAX_RPM;

// Compute what could be the longest period between two hall sensor triggers (slowest RPM) and take
// its high 16 bits. If computed period is longer than that, it is assumed that spindle is stopped.
const MAX_MSB: u32 = ((60 * HALL_TICK_FREQUENCY) / HALL_MIN_RPM + 0xffffu32) >> 16;

pub struct RpmSensorImpl<Port: 'static> {
    port: Peripheral<Port>,
    pin: usize,
    captured: u32,
    msb: u32,
}
unsafe impl <Port> Send for RpmSensorImpl<Port> { }

impl <Port> RpmSensorImpl<Port> where Port: Deref<Target = gpioa::RegisterBlock> {
    pub const fn new(port: Peripheral<Port>, pin: usize) -> RpmSensorImpl<Port> {
        RpmSensorImpl { port, pin, captured: 0, msb: 0 }
    }

    pub fn init(&mut self) {
        let tim2 = self.unsafe_timer(); // Timer is completely owned by the sensor
        let port = self.port();

        port.pin_config(self.pin).input().pull_up_down();
        port.write_pin(self.pin, true); // pull-up

        tim2.psc.write(|w| w.psc().bits(((FREQUENCY / HALL_TICK_FREQUENCY) - 1) as u16));
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

    /// Completely owned by `Hall`
    fn unsafe_timer(&self) -> &'static TIM2 {
        unsafe { &*TIM2.get() }
    }

    fn port(&self) -> &Port {
        unsafe { &*self.port.get() }
    }
}

impl <Port> RpmSensor for RpmSensorImpl<Port> where Port: Deref<Target = gpioa::RegisterBlock> {
    fn interrupt(&mut self) -> bool {
        let tim2 = self.unsafe_timer();

        if tim2.sr.read().cc1if().bit_is_set() {
            // FIXME: check if we can get away with write...
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
                // FIXME: check if we can get away with write...
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
            true
        } else if tim2.sr.read().uif().is_pending() {
            // FIXME: check if we can get away with write...
            tim2.sr.modify(|_, w| w.uif().clear());
            self.msb += 1;

            // Period is too long -- assume that spindle is stopped
            if self.msb >= MAX_MSB {
                self.captured = 0;
            }
            true
        } else {
            false
        }
    }

    fn rpm(&self) -> u32 {
        if self.captured != 0 { ((60 * HALL_TICK_FREQUENCY) << 8) / self.captured } else { 0 }
    }
}