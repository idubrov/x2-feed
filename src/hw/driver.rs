use stm32f103xx::{GPIOA, TIM1, RCC};

const fn ns2ticks(ns: u32) -> u16 {
    const NANOS_IN_SECOND: u32 = 1_000_000_000 / ::hw::DRIVER_TICK_FREQUENCY;
    ((ns + NANOS_IN_SECOND - 1) / NANOS_IN_SECOND) as u16
}

const STEP_PULSE_WIDTH_TICKS: u16 = ns2ticks(75);


pub struct DriverRef;

impl DriverRef {
    pub fn materialize<'a>(&self, tim1: &'a TIM1, gpioa: &'a GPIOA) -> Driver<'a> {
        Driver {
            tim1: tim1,
            gpioa: gpioa,
        }
    }
}

pub struct Driver<'a> {
    tim1: &'a TIM1,
    gpioa: &'a GPIOA,
}

impl<'a> Driver<'a> {
    pub fn init(&self, rcc: &RCC) {
        let tim1 = self.tim1;

        rcc.apb2enr.modify(|_, w| w.tim1en().enabled());
        rcc.apb2enr.modify(|_, w| w.iopaen().enabled());
        rcc.apb2enr.modify(|_, w| w.afioen().enabled());

        ::hw::STEP.set(self.gpioa, 1);
        ::hw::DIR.set(self.gpioa, 1);
        // Start in reset mode
        ::hw::ENABLE.set(self.gpioa, 0);
        ::hw::RESET.set(self.gpioa, 0);

        self.gpioa.crh.write(|w| w
            .mode8().output50().cnf8().alt_open()
            .mode9().output50().cnf9().open()
            .mode10().output50().cnf10().open()
            .mode11().output50().cnf11().open());

        // Prescaler
        tim1.psc.write(|w| w.psc().bits(((::hw::FREQUENCY / ::hw::DRIVER_TICK_FREQUENCY) - 1) as u16));

        // Initialize timer
        tim1.cr1.write(|w| w
            .dir().up()
            .ckd().div1()
            // Preload ARR (gets loaded once timer update event triggers)
            .arpe().set_bit()
            // Only counter overflow/underflow generates an update interrupt
            // reload_timer() should not generate an event.
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
    }

    pub fn set_direction(&self, dir: bool) {
        ::hw::DIR.set(self.gpioa, if dir { 1 } else { 0 });
    }

    pub fn disable(&self) {
        ::hw::ENABLE.set(self.gpioa, 0);
    }

    pub fn enable(&self) {
        ::hw::ENABLE.set(self.gpioa, 1);
    }

    pub fn reset(&self) {
        ::hw::RESET.set(self.gpioa, 0);
    }

    pub fn release(&self) {
        ::hw::RESET.set(self.gpioa, 1);
    }

    pub fn start(&self, is_last: bool) {
        if is_last {
            self.tim1.cr1.modify(|_, w| w.opm().one_pulse().cen().enabled());
        } else {
            self.tim1.cr1.modify(|_, w| w.opm().continuous().cen().enabled());
        }
    }

    /// Generate event to reload timer values from preload registers.
    pub fn reload_timer(&self) {
        self.tim1.egr.write(|w| w.ug().set_bit());
    }

    pub fn check_stopped(&self) -> bool {
        // Step generation is still running
        if self.tim1.cr1.read().cen().bit_is_set() {
            return false;
        }

        // Last update is pending, consider as non-stopped yet
        if self.tim1.sr.read().uif().is_pending() {
            return false;
        }
        true
    }

    pub fn set_delay(&self, delay: u16) {
        // FIXME: delay could be 0?
        self.tim1.arr.write(|w| w.arr().bits(delay - 1));

        if delay >= STEP_PULSE_WIDTH_TICKS {
            self.tim1.ccr1.write(|w| w.ccr1().bits(delay - STEP_PULSE_WIDTH_TICKS));
        } else {
            self.tim1.ccr1.write(|w| w.ccr1().bits(0));
        }
    }

    pub fn set_last_pulse(&self) {
        self.tim1.cr1.modify(|_, w| w.opm().one_pulse());
    }
}
