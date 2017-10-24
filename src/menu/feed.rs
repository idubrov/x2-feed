use hal::Event;
use hal::Event::*;
use hal::Button::*;
use idle;
use config::{Display, StepperDriverResource};
use core::fmt;
use font;
use rtfm::{Resource, Threshold};
use stepper;
use estop;
use super::{MenuItem, MenuResult};
use core::fmt::Write;
use cortex_m;
use settings;

const STEPS_PER_ROTATION: u32 = 200;

pub struct FeedMenu {
    speed: u32,
    slow_ipm: u16,
    fast_ipm: u16,
    ipm: u16,
    rpm: u32,
}

impl FeedMenu {
    pub fn new() -> FeedMenu {
        FeedMenu {
            speed: 0,
            slow_ipm: 10,
            fast_ipm: 30,
            rpm: 0,
            ipm: 0,
        }
    }

    fn update_screen(&self, t: &mut Threshold, r: &mut idle::Resources) {
        let run_state = r.STEPPER.claim(t, |s, _t| s.state());
        let is_fast = r.CONTROLS.state().fast;

        let mut lcd = Display::new(r.SCREEN);
        lcd.position(0, 0);
        let rrpm = (self.rpm + 128) >> 8;
        write!(lcd, "{: >4} RPM", rrpm).unwrap();

        lcd.position(0, 1);
        let c = match (run_state, is_fast) {
            (stepper::State::Running(false), false) => font::LEFT,
            (stepper::State::Running(true), false) => font::RIGHT,
            (stepper::State::Running(false), true) => font::FAST_LEFT,
            (stepper::State::Running(true), true) => font::FAST_RIGHT,
            _ => ' '
        };
        write!(lcd, "{}{: >3} IPM", c, u32::from(self.ipm)).unwrap();
    }

    fn handle_feed_rate(&mut self, event: Event, t: &mut Threshold, r: &mut idle::Resources, steps_per_inch: u32) {
        let mut ipm = r.ENCODER.current() + 1; // Encoder is off by one (as it starts from 0)
        match event {
            Pressed(Fast) => {
                // Switch to fast IPM
                self.slow_ipm = ipm;
                ipm = self.fast_ipm;
                r.ENCODER.set_current(ipm - 1);
            }
            Unpressed(Fast) => {
                // Switch to slow IPM
                self.fast_ipm = ipm;
                ipm = self.slow_ipm;
                r.ENCODER.set_current(ipm - 1);
            }
            _ => {}
        }

        // Update stepper speed based on current setting
        // Shift by 8 to convert to 24.8 format
        let speed = (u32::from(ipm << 8) * steps_per_inch) / 60;
        if self.speed != speed {
            r.STEPPER.claim_mut(t, |s, _t| s.set_speed(speed)).unwrap();
            self.speed = speed;
        }
        self.ipm = ipm
    }

    fn handle_feed(&mut self, event: Event, t: &mut Threshold, r: &mut idle::Resources) {
        let run_state = r.STEPPER.claim(t, |s, _t| s.state());
        match (run_state, event) {
            (stepper::State::Stopped, Pressed(Left)) => {
                // Use very low number for moving left
                move_to(t, r, -1_000_000_000);
            }

            (stepper::State::Stopped, Pressed(Right)) => {
                // Use very high number for moving right
                move_to(t, r, 1_000_000_000);
            }

            (stepper::State::Running(false), Unpressed(Left)) |
            (stepper::State::Running(true), Unpressed(Right)) =>
                r.STEPPER.claim_mut(t, |s, _t| s.stop()),

            _ => {}
        }
    }

    fn handle_rpm(&mut self, t: &mut Threshold, r: &idle::Resources) {
        use rtfm::Resource;

        let rpm = r.HALL.claim(t, |hall, _t| hall.rpm());

        // Only capture if difference is big enough (more than .5%)
        if self.rpm == 0 || rpm * 200 > self.rpm * 201 || rpm * 200 < self.rpm * 199 {
            self.rpm = rpm;
        }
    }

    fn run_feed(&mut self, t: &mut Threshold, r: &mut idle::Resources) -> MenuResult {
        reload_stepper_settings(t, r);

        // Pre-compute steps-per-inch
        let steps_per_inch = u32::from(settings::PITCH.read(r.FLASH)) *
            u32::from(settings::MICROSTEPS.read(r.FLASH)) *
            STEPS_PER_ROTATION;

        r.ENCODER.set_current(self.slow_ipm - 1);
        r.ENCODER.set_limit(settings::MAX_IPM.read(r.FLASH));

        Display::new(r.SCREEN).clear();
        loop {
            estop::check(&mut Display::new(r.SCREEN));

            let event = r.CONTROLS.read_event();
            self.handle_feed_rate(event, t, r, steps_per_inch);
            self.handle_feed(event, t, r);
            self.handle_rpm(t, r);
            self.update_screen(t, r);

            if let Pressed(Encoder) = event {
                self.stop_and_wait(t, r);
                return MenuResult::Ok;
            }
        }
    }

    fn stop_and_wait(&self, t: &mut Threshold, r: &mut idle::Resources) {
        r.STEPPER.claim_mut(t, |s, _t| s.stop());
        {
            let mut lcd = Display::new(r.SCREEN);
            lcd.clear();
            lcd.position(0, 0);
            write!(lcd, "Stopping").unwrap();
            lcd.position(0, 1);
            write!(lcd, "  ...").unwrap();
        }

        while r.STEPPER.claim(t, |s, _t| {
            if let stepper::State::Stopped = s.state() {
                return false;
            }
            // Enter WFI while we block stepper interrupt (via claim above), to avoid race conditions.
            // We should still wake up if interrupt happens (but it won't be handled until we exit
            // the claim block).
            cortex_m::asm::wfi();
            true
        }) {}
    }
}

impl MenuItem for FeedMenu {
    fn run(&mut self, t: &mut Threshold, r: &mut idle::Resources) -> MenuResult {
        self.run_feed(t, r)
    }

    fn is_active_by_default(&self, _t: &mut Threshold, r: &mut idle::Resources) -> bool {
        settings::IS_LATHE.read(r.FLASH) != 0
    }
}

impl fmt::Display for FeedMenu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Feed (IPM)")
    }
}

// Helper function to run stepper command. Claims both driver and stepper.
fn move_to(t: &mut Threshold, r: &mut idle::Resources, target: i32) {
    let driver = &mut r.DRIVER;
    r.STEPPER.claim_mut(t, |stepper, t| {
        driver.claim_mut(t, |driver, _t| {
            let driver: &mut StepperDriverResource = driver;
            stepper.move_to(driver, target).unwrap()
        })
    })
}

// Reload stepper settings from EEPROM
fn reload_stepper_settings(t: &mut Threshold, r: &mut idle::Resources) {
    let reversed = settings::IS_REVERSED.read(r.FLASH) != 0;
    let acceleration = u32::from(settings::ACCELERATION.read(r.FLASH)) *
        u32::from(settings::MICROSTEPS.read(r.FLASH)) << 8;

    r.STEPPER.claim_mut(t, |s, _t| {
        s.set_reversed(reversed);
        s.set_acceleration(acceleration).unwrap();
    });
}