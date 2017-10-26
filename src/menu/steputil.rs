use stepper;
use cortex_m;
use rtfm::{Resource, Threshold};
use config::StepperDriverResource;
use stepper::Stepper;

pub fn move_delta<D, S>(t: &mut Threshold, delta: i32, driver: &mut D, stepper: &mut S)
    where D: Resource<Data = StepperDriverResource>, S: Resource<Data = Stepper> {

    stepper.claim_mut(t, |stepper, t| {
        driver.claim_mut(t, |driver, _t| {
            let driver: &mut StepperDriverResource = driver;
            let target = stepper.position() + delta;
            stepper.move_to(driver, target).unwrap();
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