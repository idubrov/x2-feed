use stm32f103xx::{GPIOA, TIM1, RCC};
use hw::config::{DRIVER_TICK_FREQUENCY, FREQUENCY, ENABLE, RESET, STEP, DIR};
use hal::stepper::{DriverControl, PulseGen};

const fn ns2ticks(ns: u32) -> u16 {
    const NANOS_IN_SECOND: u32 = 1_000_000_000 / DRIVER_TICK_FREQUENCY;
    ((ns + NANOS_IN_SECOND - 1) / NANOS_IN_SECOND) as u16
}

const STEP_PULSE_WIDTH_TICKS: u16 = ns2ticks(75);

pub struct SingletonDriver;

impl SingletonDriver {
    pub fn materialize<'a>(&self, tim1: &'a TIM1, gpioa: &'a GPIOA) -> BoundDriver<'a> {
        BoundDriver {
            tim1,
            gpioa,
        }
    }
}

pub struct BoundDriver<'a> {
    tim1: &'a TIM1,
    gpioa: &'a GPIOA,
}

impl<'a> DriverControl for BoundDriver<'a> {
    fn enable(&mut self, enable: bool) {
        ENABLE.set(self.gpioa, if enable { 1 } else { 0 });
    }

    fn direction(&mut self, dir: bool) {
        // Note: reversed
        DIR.set(self.gpioa, if dir { 0 } else { 1 });
    }
}

impl<'a> PulseGen for BoundDriver<'a> {
    fn start(&mut self, first_delay: u16) {
        self.preload_delay(first_delay);

        // Generate event to reload timer values from the preload registers.
        self.tim1.egr.write(|w| w.ug().set_bit());
        self.tim1.cr1.modify(|_, w| w.opm().continuous().cen().enabled());
    }

    fn preload_delay(&mut self, delay: u16) {
        // FIXME: delay could be 0?
        self.tim1.arr.write(|w| w.arr().bits(delay - 1));

        // FIXME: reject too short delays?
        if delay >= STEP_PULSE_WIDTH_TICKS {
            self.tim1.ccr1.write(|w| w.ccr1().bits(delay - STEP_PULSE_WIDTH_TICKS));
        } else {
            self.tim1.ccr1.write(|w| w.ccr1().bits(0));
        }
    }

    fn set_last(&mut self) {
        // Switch to one-pulse mode (current pulse is the last one)
        self.tim1.cr1.modify(|_, w| w.opm().one_pulse());
    }

    fn is_running(&self) -> bool {
        // Check if timer is still running
        self.tim1.cr1.read().cen().bit_is_set()
    }
}

impl<'a> BoundDriver<'a> {
    pub fn init(&self, rcc: &RCC) {
        let tim1 = self.tim1;

        rcc.apb2enr.modify(|_, w| w.tim1en().enabled());
        rcc.apb2enr.modify(|_, w| w.iopaen().enabled());
        rcc.apb2enr.modify(|_, w| w.afioen().enabled());

        STEP.set(self.gpioa, 1);
        DIR.set(self.gpioa, 1);
        // Start in reset mode
        ENABLE.set(self.gpioa, 0);
        RESET.set(self.gpioa, 0);

        self.gpioa.crh.write(|w| w
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
        RESET.set(self.gpioa, 1);
    }
}
