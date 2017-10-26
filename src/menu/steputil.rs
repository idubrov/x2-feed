use stepper;
use cortex_m;
use rtfm::{Resource, Threshold};
use idle::Resources;
use config::StepperDriverResource;
use stepper::{Stepper, Target, Params};
use settings;


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
pub fn move_delta<D, S>(t: &mut Threshold, delta: i32, driver: &mut D, stepper: &mut S)
    where D: Resource<Data = StepperDriverResource>, S: Resource<Data = Stepper> {

    stepper.claim_mut(t, |stepper, t| {
        driver.claim_mut(t, |driver, _t| {
            let driver: &mut StepperDriverResource = driver;
            let target = stepper.position() + delta;
            stepper.set_target(driver, Target::Position(target)).expect("move is ok");
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