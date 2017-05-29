use stm32f103xx::{Gpioa, Tim1, Rcc, Syst};
use stepgen;

pub struct Driver {
    // If should reverse direction signal
    reverse: bool,

    // Current state
    stepgen: stepgen::Stepgen,
    direction: bool,

    base_step: u32,
    position: i32,

    // Stop signal
    // FIXME: atomic/refcell?
    stop_requested: bool,
}

impl Driver {
    pub const fn new() -> Driver {
        Driver {
            reverse: false,
            stepgen: stepgen::Stepgen::new(::hw::TICK_FREQUENCY),
            direction: true,
            base_step: 0,
            position: 0,
            stop_requested: false,
        }
    }

    pub fn init(&self, tim1: &Tim1, gpioa: &Gpioa, rcc: &Rcc) {
        rcc.apb2enr.modify(|_, w| w.tim1en().enabled());
        rcc.apb2enr.modify(|_, w| w.iopaen().enabled());

        ::hw::STEP.set(gpioa, 1);
        ::hw::DIR.set(gpioa, 1);
        ::hw::ENABLE.set(gpioa, 1);
        ::hw::RESET.set(gpioa, 1);

        gpioa.crh.write(|w| w
            .mode8().output50().cnf8().alt_open()
            .mode9().output50().cnf9().open()
            .mode10().output50().cnf10().open()
            .mode11().output50().cnf11().open());

        // Start in reset mode
        ::hw::ENABLE.set(gpioa, 0);
        ::hw::RESET.set(gpioa, 0);

        // Prescaler
        tim1.psc.write(|w| w.psc().bits(((::hw::FREQUENCY / ::hw::TICK_FREQUENCY) - 1) as u16));

        // Initialize timer
        tim1.cr1.write(|w| w
            .dir().up()
            .ckd().div1()
            // Preload ARR (gets loaded once timer update event triggers)
            .arpe().set()
            // Only counter overflow/underflow generates an update interrupt
            // reload_timer() should not generate an event.
            .urs().set());

        tim1.cr2.write(|w| w
            // Output '1' when idle
            .ois1().set());

        tim1.ccmr1_output.write(|w| w
            // Preload CCR1 (gets loaded once timer update event triggers)
            .oc1pe().set()
            // Inactive till CCR1, then active
            .oc1m().pwm2());

        // Configure PWM channel 1
        tim1.ccer.write(|w| w
            // Active low
            .cc1p().set()
            // Enable channel 1
            .cc1e().set());

        tim1.bdtr.write(|w| w
            // Enable PWM outputs
            .moe().set());

        // Enable interrupts
        tim1.sr.modify(|_, w| w.uif().clear());
        tim1.dier.write(|w| w.uie().set());
    }

    // Hardware-specific


    fn start(&self, tim1: &Tim1, is_last: bool) {
        if is_last {
            tim1.cr1.modify(|_, w| w.opm().one_pulse().cen().enabled());
        } else {
            tim1.cr1.modify(|_, w| w.opm().continuous().cen().enabled());
        }
    }

    /// Generate event to reload timer values from preload registers.
    fn reload_timer(&self, tim1: &Tim1) {
        tim1.egr.write(|w| w.ug().set());
    }

    fn set_direction_hw(&self, gpioa: &Gpioa, dir: bool) {
        ::hw::DIR.set(gpioa, if dir { 1 } else { 0 });
    }

    fn check_stopped(&self, tim1: &Tim1) -> bool {
        // Step generation is still running
        if tim1.cr1.read().cen().is_set() {
            return false;
        }

        // If there is a pending interrupt, wait until it is cleared.
        // (for instance, we got here just after last timer overflow and it wasn't processed yet).
        while tim1.sr.read().uif().is_pending() {}
        true
    }

    fn disable(&self, gpioa: &Gpioa) {
        ::hw::ENABLE.set(gpioa, 0);
    }

    fn enable(&self, gpioa: &Gpioa) {
        ::hw::ENABLE.set(gpioa, 1);
    }

    fn set_delay(&mut self, tim1: &Tim1, delay: u16) {
        // FIXME: delay could be 0?
        tim1.arr.write(|w| w.arr().bits(delay - 1));

        if delay >= ::hw::STEP_PULSE_WIDTH_TICKS {
            tim1.ccr1.write(|w| w.ccr1().bits(delay - ::hw::STEP_PULSE_WIDTH_TICKS));
        } else {
            tim1.ccr1.write(|w| w.ccr1().bits(0));
        }
    }

    fn set_last_pulse(&self, tim1: &Tim1) {
        tim1.cr1.modify(|_, w| w.opm().one_pulse());
    }

    /// Initialize step generation
    pub fn reset(&mut self) {
        // FIXME: params...
        let accel = 1200;
        let microsteps = 16;
        self.stepgen.set_acceleration((accel * microsteps) << 8);
    }

    fn load_delay(&mut self, tim1: &Tim1) -> u32 {
        let delay = self.stepgen.next();
        if delay != 0 {
            // Load new step into ARR, start pulse at the end
            let d = (delay + 128) >> 8; // Delay is in 16.8 format
            self.set_delay(tim1, d as u16);
        } else {
            // FIXME: do we need this branch?
            // Load idle values. This is important to do on the last update
            // when timer is switched into one-pulse mode.
            self.set_delay(tim1, 1 /* FIXME: IDLE delay */);
        }
        delay
    }

    pub fn step_completed(&mut self, tim1: &Tim1, gpioa: &Gpioa) {
        if self.check_stopped(tim1) {
            self.disable(gpioa);
            return;
        }

        if self.stop_requested {
            self.stepgen.set_target_step(0);
            self.stop_requested = false;
        }

        if self.load_delay(tim1) == 0 {
            // Stop on the next update, one pulse mode
            // Note that load_delay() should have already loaded ARR and
            // CCR1 with idle values.
            self.set_last_pulse(tim1);
        }
    }

    // Incorporate outstanding steps from the stepgen into current position
    fn update_position(&mut self) -> i32 {
        let step = self.stepgen.current_step();
        let offset = (step - self.base_step) as i32;
        self.base_step = step;
        if self.direction {
            self.position += offset;
        } else {
            self.position -= offset;
        }
        self.position
    }

    fn set_direction(&mut self, syst: &Syst, gpioa: &Gpioa, dir: bool) {
        ::hw::delay::ns(syst, ::hw::DIR_SETUP_NS);
        self.set_direction_hw(gpioa, if self.reverse { dir } else { !dir });
        self.direction = dir;
        ::hw::delay::ns(syst, ::hw::DIR_HOLD_NS);
    }

    /// Move to given position. Note that no new move commands will be accepted while stepper is
    /// running. However, other target parameter, target speed, could be changed any time.
    /// FIXME: technically, we can actually change target on the fly...
    pub fn move_to(&mut self, syst: &Syst, tim1: &Tim1, gpioa: &Gpioa, target: i32) -> bool {
        if !self.check_stopped(tim1) {
            return false;
        }

        let pos = self.update_position();
        let delta;
        if pos < target {
            delta = (target - pos) as u32;
            self.set_direction(syst, gpioa, true);
        } else if pos > target {
            delta = (pos - target) as u32;
            self.set_direction(syst, gpioa, false);
        } else {
            // Nothing to do!
            return true;
        }
        self.stepgen.set_target_step(self.base_step + delta);
        self.stop_requested = false;

        // Load first delay into ARR/CC1, if not stopped
        if self.load_delay(tim1) == 0 {
            // Not making even a single step
            return false;
        }

        // FIXME: should be configurable... Enable driver outputs
        self.enable(gpioa);
        self.reload_timer(tim1);

        // Load second delay into ARR & CC1.
        let is_last = self.load_delay(tim1) == 0;

        // Start pulse generation
        self.start(tim1, is_last);

        return true;
    }

    /// Set slew speed (maximum speed stepper motor would run). Note that stepper motor would only
    /// reach this speed if target step is far enough, so there is enough time for deceleration.
    /// FIXME: no Javadoc!
    /// @param speed target slew speed to reach, in steps per second, 24.8 format
    pub fn set_speed(&mut self, speed: u32) {
        self.stepgen.set_target_speed(speed);
    }
}
