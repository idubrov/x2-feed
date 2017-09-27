use bare_metal::Peripheral;
use stm32f103xx::gpioa;
use core::ops::Deref;
use stm32_extras::GPIOExtras;
use stm32f103xx::TIM1;

pub const DRIVER_TICK_FREQUENCY: u32 = 1_000_000; // 1us timer resolution

pub trait StepperDriver {
    // Control aspect of stepper motor driver (setting directions, enabling/disabling outputs).

    /// Enable/disable driver outputs.
    fn enable(&mut self, enable: bool);

    /// Set stepper driver direction.
    fn direction(&mut self, bit: bool);

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

const STEP_PULSE_WIDTH_TICKS: u16 = ns2ticks(75);

pub struct StepperDriverImpl<Port: 'static> {
    port: Peripheral<Port>,
    step: usize,
    dir: usize,
    enable: usize,
    reset: usize
}
unsafe impl <Port> Send for StepperDriverImpl<Port> { }

impl <Port> StepperDriverImpl<Port> where Port: Deref<Target = gpioa::RegisterBlock> {
    pub const fn new(port: Peripheral<Port>, step: usize, dir: usize, enable: usize, reset: usize) -> StepperDriverImpl<Port> {
        StepperDriverImpl { port, step, dir, enable, reset }
    }

    // Note that we require an explicit ownership of I/O port peripheral to guard against
    // concurrent access when we modify shared register of the peripheral (CRH)
    pub fn init(&mut self) {
        let tim1 = self.unsafe_timer(); // Timer is completely owned by `Driver`
        let port = self.port();

        port.write_pin(self.step, true);
        port.write_pin(self.dir, true);
        port.write_pin(self.enable, false);
        port.write_pin(self.reset, false); // Start in reset mode

        port.pin_config(self.step).output50().open_drain().alternate();
        port.pin_config(self.dir).output50().open_drain();
        port.pin_config(self.enable).output50().open_drain();
        port.pin_config(self.reset).output50().open_drain();

        // Prescaler
        tim1.psc.write(|w| w.psc().bits(((super::clock::FREQUENCY / DRIVER_TICK_FREQUENCY) - 1) as u16));

        // Initialize timer
        tim1.cr1.write(|w| w
            .dir().up()
            .ckd().div1()
            // Preload ARR (gets loaded once timer update event triggers)
            .arpe().set_bit()
            // Only counter overflow/underflow generates an update interrupt
            // reloading timer in start() should not generate an event.
            .urs().set_bit());

        tim1.cr2.write(|w| w
            // Output '1' when idle
            .ois1().set_bit());

        tim1.ccmr1_output.write(|w| w
            // Preload CCR1 (gets loaded once timer update event triggers)
            .oc1pe().set_bit()
            // Inactive till CCR1, then active
            .oc1m().pwm2());

        // Configure PWM channel 1
        tim1.ccer.write(|w| w
            // Active low
            .cc1p().set_bit()
            // Enable channel 1
            .cc1e().set_bit());

        tim1.bdtr.write(|w| w
            // Enable PWM outputs
            .moe().set_bit());

        // Enable interrupts
        tim1.sr.modify(|_, w| w.uif().clear());
        tim1.dier.write(|w| w.uie().set_bit());

        // Enable the driver
        port.write_pin(self.reset, true);
    }

    /// Completely owned by `Driver`
    fn unsafe_timer(&self) -> &'static TIM1 {
        unsafe { &*TIM1.get() }
    }

    fn port(&self) -> &Port {
        unsafe { &*self.port.get() }
    }
}


impl <Port> StepperDriver for StepperDriverImpl<Port> where Port: Deref<Target = gpioa::RegisterBlock> {
    // Controls
    fn enable(&mut self, enable: bool) {
        self.port().write_pin(self.enable, enable);
    }

    fn direction(&mut self, dir: bool) {
        // Note: reversed
        self.port().write_pin(self.dir, !dir);
    }

    // Pulse generation


    fn start(&mut self, first_delay: u16) {
        self.preload_delay(first_delay);

        // Generate event to reload timer values from the preload registers.
        let tim1 = self.unsafe_timer();
        tim1.egr.write(|w| w.ug().set_bit());
        tim1.cr1.modify(|_, w| w.opm().continuous().cen().enabled());
    }

    fn preload_delay(&mut self, delay: u16) {
        let tim1 = self.unsafe_timer();
        // FIXME: delay could be 0?
        tim1.arr.write(|w| w.arr().bits(delay - 1));

        // FIXME: reject too short delays?
        if delay >= STEP_PULSE_WIDTH_TICKS {
            tim1.ccr1.write(|w| w.ccr1().bits(delay - STEP_PULSE_WIDTH_TICKS));
        } else {
            tim1.ccr1.write(|w| w.ccr1().bits(0));
        }
    }

    fn set_last(&mut self) {
        // Switch to one-pulse mode (current pulse is the last one)
        self.unsafe_timer().cr1.modify(|_, w| w.opm().one_pulse());
    }

    fn is_running(&self) -> bool {
        // Check if timer is still running
        self.unsafe_timer().cr1.read().cen().bit_is_set()
    }

    fn interrupt(&mut self) -> bool {
        let tim1 = self.unsafe_timer();
        if tim1.sr.read().uif().is_pending() {
            tim1.sr.modify(|_, w| w.uif().clear());
            true
        } else {
            false
        }
    }
}
