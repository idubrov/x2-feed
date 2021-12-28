use crate::stepper;
use rtic::Mutex;

pub fn move_delta(delta: i32, r: &mut crate::app::idle::SharedResources) {
    r.stepper
        .lock(|s| {
            let target = s.position() + delta;
            s.move_to(target)
        })
        .unwrap()
}

pub fn wait_stopped(r: &mut crate::app::idle::SharedResources) {
    while r.stepper.lock(|s| {
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
