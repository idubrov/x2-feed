use stepper;
use cortex_m;
use rtfm::{Resource, Threshold};
use idle::Resources;
use config::StepperDriverResource;
use stepper::{Stepper, Target, Params};
use settings;
use core::fmt;


// Reload stepper settings from EEPROM
pub fn reload_stepper_settings(t: &mut Threshold, r: &mut Resources) {
    let reversed = settings::IS_REVERSED.read(r.FLASH) != 0;
    let acceleration = (u32::from(settings::ACCELERATION.read(r.FLASH)) *
        u32::from(settings::MICROSTEPS.read(r.FLASH))) << 8;
    let disable_on_stop = settings::DISABLE_ON_STOP.read(r.FLASH) != 0;
    r.STEPPER.claim_mut(t, |s, _t|
        s.set_params(Params { reversed, disable_on_stop, acceleration }).expect("stepgen params"));
}

// FIXME: unify...
pub fn move_to<D, S>(t: &mut Threshold, driver: &mut D, stepper: &mut S, target: Target)
    where D: Resource<Data = StepperDriverResource>, S: Resource<Data = Stepper> {

    stepper.claim_mut(t, |stepper, t| {
        driver.claim_mut(t, |driver, _t| {
            let driver: &mut StepperDriverResource = driver;
            stepper.set_target(driver, target).expect("move is ok");
        })
    })
}

pub fn wait_stopped<S: Resource<Data = Stepper>>(t: &mut Threshold, stepper: &mut S) {
    while stepper.claim(t, |s, _t| {
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

// Feed rate


#[derive(Copy, Clone)]
pub enum FeedRate {
    IPM(u16),
    IPR(u16)
}

impl FeedRate {
    pub fn with_rate(&self, rate: u16) -> Self {
        match *self {
            FeedRate::IPM(_ipm) => FeedRate::IPM(rate),
            FeedRate::IPR(_ipr) => FeedRate::IPR(rate)
        }
    }

    pub fn rate(&self) -> u16 {
        match *self {
            FeedRate::IPM(rate) | FeedRate::IPR(rate) => rate
        }
    }

    pub fn to_speed(&self, steps_per_inch: u32, rpm: u32) -> u32 {
        // Update stepper speed based on current setting
        // Shift by 8 to convert to 24.8 format
        match *self {
            FeedRate::IPM(ipm) => ((u32::from(ipm) * steps_per_inch) << 8) / 60,
            FeedRate::IPR(ipr) => {
                // IPR are in thou, so additionally divide by 1_000
                // Also, RPM is already in 24.8 format, so no need to shift
                // FIXME: wrapping?
                (u64::from(ipr) * u64::from(rpm) * u64::from(steps_per_inch) / 60_000) as u32
            },
        }
    }
}

impl fmt::Display for FeedRate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FeedRate::IPM(ipm) => write!(f, "{: >3} IPM", ipm),
            FeedRate::IPR(ipr) => write!(f, "0.{:0>3} IPR", ipr)
        }
    }
}