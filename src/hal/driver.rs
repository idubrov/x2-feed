use stm32f1::stm32f103::TIM1;
use stm32f1xx_hal::gpio::{Alternate, ErasedPin, OpenDrain, Output, Pin as PinT, CRH};

type Pin = ErasedPin<Output<OpenDrain>>;
type StepPin = PinT<Alternate<OpenDrain>, CRH, 'A', 8>;

pub const DRIVER_TICK_FREQUENCY: u32 = 1_000_000; // 1us timer resolution

pub trait StepperDriver {
    // Control aspect of stepper motor driver (setting directions, enabling/disabling outputs).

    /// Enable/disable driver outputs.
    fn set_enable(&mut self, enable: bool);

    /// Enable/disable timer output channel. In thread cutting, we use timer for other purposes,
    /// so we need a way to stop generating pulses.
    fn set_timer_output(&mut self, enable: bool);

    /// Set stepper driver direction.
    fn set_direction(&mut self, bit: bool);

    // Pulse generating aspect of stepper motor driver.

    /// Enable PWM generating stepper motor pulses.
    /// `first_delay` is the first delay to load in the timer. Pulse generation starts immediately.
    fn start(&mut self, first_delay: u16);

    /// Preload delay for the next step into the pulse generator. This delay will be used once
    /// current step completes.
    fn preload_delay(&mut self, delay: u16);

    /// Indicate that no new delay is available, should stop once current step completes.
    fn set_last(&mut self);

    /// Returns `true` if timer generating pulses is running, `false` otherwise.
    fn is_running(&self) -> bool;

    /// Check for pending interrupt and handle it (reset pending flag). Returns `true` if interrupt
    /// was pending.
    fn interrupt(&mut self) -> bool;
}

const fn ns2ticks(ns: u32) -> u16 {
    const NANOS_IN_SECOND: u32 = 1_000_000_000 / DRIVER_TICK_FREQUENCY;
    ((ns + NANOS_IN_SECOND - 1) / NANOS_IN_SECOND) as u16
}

/// Width of the step pulse we send.
const STEP_PULSE_WIDTH_TICKS: u16 = ns2ticks(75);

pub struct StepperDriverImpl {
    tim1: TIM1,
    dir: Pin,
    enable: Pin,
    reset: Pin,
}

impl StepperDriverImpl {
    pub fn new(tim1: TIM1, _step: StepPin, dir: Pin, enable: Pin, reset: Pin) -> StepperDriverImpl {
        let mut driver = StepperDriverImpl {
            tim1,
            dir,
            enable,
            reset,
        };
        driver.init();
        driver
    }

    // Note that we require an explicit ownership of I/O port peripheral to guard against
    // concurrent access when we modify shared register of the peripheral (CRH)
    fn init(&mut self) {
        self.dir.set_high();
        self.enable.set_low();
        // Start in reset mode
        self.reset.set_low();

        // Prescaler
        self.tim1.psc.write(|w| {
            w.psc()
                .bits(((crate::hal::FREQUENCY / DRIVER_TICK_FREQUENCY) - 1) as u16)
        });

        // Initialize timer
        self.tim1.cr1.write(|w| {
            w.dir()
                .up()
                .ckd()
                .div1()
                // Preload ARR (gets loaded once timer update event triggers)
                .arpe()
                .set_bit()
                // Only counter overflow/underflow generates an update interrupt
                // reloading timer in start() should not generate an event.
                .urs()
                .set_bit()
        });

        self.tim1.cr2.write(|w| {
            w
                // Output '1' when idle
                .ois1()
                .set_bit()
        });

        self.tim1.ccmr1_output().write(|w| {
            w
                // Preload CCR1 (gets loaded once timer update event triggers)
                .oc1pe()
                .set_bit()
                // Inactive till CCR1, then active
                .oc1m()
                .pwm_mode2()
        });

        // Configure PWM channel 1
        self.tim1.ccer.write(|w| {
            w
                // Active low
                .cc1p()
                .set_bit()
                // Enable channel 1
                .cc1e()
                .set_bit()
        });

        self.tim1.bdtr.write(|w| {
            w
                // Enable PWM outputs
                .moe()
                .set_bit()
        });

        // Enable interrupts
        self.tim1.sr.modify(|_, w| w.uif().clear());
        self.tim1.dier.write(|w| w.uie().set_bit());

        // Enable the driver
        self.reset.set_high();
    }
}

impl StepperDriver for StepperDriverImpl {
    // Controls
    fn set_enable(&mut self, enable: bool) {
        if enable {
            self.enable.set_high();
        } else {
            self.enable.set_low();
        }
    }

    // Low-level timer output control: enable/disable PWM channel
    fn set_timer_output(&mut self, enable: bool) {
        self.tim1.ccer.write(|w| w.cc1e().bit(enable));
    }

    fn set_direction(&mut self, dir: bool) {
        if dir {
            self.dir.set_high();
        } else {
            self.dir.set_low();
        }
    }

    // Pulse generation

    fn start(&mut self, first_delay: u16) {
        self.preload_delay(first_delay);

        // FIXME: does this cause second preload?...
        // Generate event to reload timer values from the preload registers.
        self.tim1.egr.write(|w| w.ug().set_bit());
        self.tim1
            .cr1
            .modify(|_, w| w.opm().disabled().cen().enabled());
    }

    fn preload_delay(&mut self, delay: u16) {
        // FIXME: delay could be 0?
        self.tim1.arr.write(|w| w.arr().bits(delay - 1));

        // FIXME: reject too short delays?
        if delay >= STEP_PULSE_WIDTH_TICKS {
            self.tim1
                .ccr1
                .write(|w| w.ccr().bits(delay - STEP_PULSE_WIDTH_TICKS));
        } else {
            self.tim1.ccr1.write(|w| w.ccr().bits(0));
        }
    }

    fn set_last(&mut self) {
        // Switch to one-pulse mode (current pulse is the last one)
        // FIXME: verify: was opm().one_pulse()
        self.tim1.cr1.modify(|_, w| w.opm().enabled());
    }

    fn is_running(&self) -> bool {
        // Check if timer is still running
        self.tim1.cr1.read().cen().bit_is_set()
    }

    fn interrupt(&mut self) -> bool {
        if self.tim1.sr.read().uif().is_update_pending() {
            self.tim1.sr.modify(|_, w| w.uif().clear());
            true
        } else {
            false
        }
    }
}
