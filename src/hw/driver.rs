use stm32f103xx::{GPIOA, TIM1, RCC};
use hw::config::{DRIVER_TICK_FREQUENCY, FREQUENCY, ENABLE, RESET, STEP, DIR};
use hal::StepperDriver;

const fn ns2ticks(ns: u32) -> u16 {
    const NANOS_IN_SECOND: u32 = 1_000_000_000 / DRIVER_TICK_FREQUENCY;
    ((ns + NANOS_IN_SECOND - 1) / NANOS_IN_SECOND) as u16
}

const STEP_PULSE_WIDTH_TICKS: u16 = ns2ticks(75);

pub struct Driver;

impl Driver {
    // Note that we require an explicit ownership of I/O port peripheral to guard against
    // concurrent access when we modify shared register of the peripheral (CRH)
    pub fn init(&mut self, rcc: &RCC, gpioa: &GPIOA) {
        let tim1 = self.unsafe_timer(); // Timer is completely owned by `Driver`

        rcc.apb2enr.modify(|_, w| w.tim1en().enabled());
        rcc.apb2enr.modify(|_, w| w.iopaen().enabled());
        rcc.apb2enr.modify(|_, w| w.afioen().enabled());

        STEP.set(gpioa, 1);
        DIR.set(gpioa, 1);
        ENABLE.set(gpioa, 0);
        RESET.set(gpioa, 0); // Start in reset mode

        gpioa.crh.modify(|_, w| w
            .mode8().output50().cnf8().alt_open()
            .mode9().output50().cnf9().open()
            .mode10().output50().cnf10().open()
            .mode11().output50().cnf11().open());

        // Prescaler
        tim1.psc.write(|w| w.psc().bits(((FREQUENCY / DRIVER_TICK_FREQUENCY) - 1) as u16));

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
        RESET.set(gpioa, 1);
    }

    // Unsafe accessor for the port. Should only be used when both concurrent access is
    // guaranteed by ownership of mutable Driver reference and access is safe in regard
    // of modifying registers not owned by the Driver.
    fn unsafe_port(&self) -> &GPIOA {
        unsafe { &*GPIOA.get() }
    }

    fn unsafe_timer(&self) -> &TIM1 {
        unsafe { &*TIM1.get() }
    }
}


impl StepperDriver for Driver {
    // Controls


    fn enable(&mut self, enable: bool) {
        ENABLE.set(self.unsafe_port(), if enable { 1 } else { 0 });
    }

    fn direction(&mut self, dir: bool) {
        // Note: reversed
        DIR.set(self.unsafe_port(), if dir { 0 } else { 1 });
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
