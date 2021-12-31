use crate::menu::util::{NavStatus, Navigation};
use crate::menu::{MenuItem, MenuResources, limits, steputil};
use crate::{settings, stepper};
use crate::stepper::{StepperError as StepperError, Stepper};
use core::fmt::{self, Write};
use crate::hal::{Button, Event, StepperDriverImpl};
use rtic::Mutex;
use stepgen::Error as StepgenError;

const MICRON_PER_INCH: u32 = 25400;

#[derive(Copy, Clone)]
pub enum ThreadSize {
    /// Threads per inch
    TPI(u16),
    /// Microns per thread
    MICRON(u16),
}

impl ThreadSize {
    fn to_steps_per_thread(self, steps_per_inch: u32) -> u32 {
        match self {
            ThreadSize::TPI(tpi) => steps_per_inch / u32::from(tpi),
            ThreadSize::MICRON(micron) => u32::from(micron) * steps_per_inch / MICRON_PER_INCH,
        }
    }
}

impl fmt::Display for ThreadSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ThreadSize::TPI(tpi) => write!(f, "{: >3} TPI", tpi),
            ThreadSize::MICRON(micron) => write!(f, "{}.{:0>3}mm", micron / 1000, micron % 1000),
        }
    }
}

pub struct ThreadingOperation {
    thread: ThreadSize,
    /// Phase for starting the thread cutting, from `0` to `steps_per_thread`. Defines how many steps
    /// do we need to offset our target. If `0`, end of the thread (`left` position) would be
    /// exactly at the location of the magnetic sensor. If `steps_per_thread / 2`, it will be offset
    /// by 180 degree.
    /// FIXME: use 0..360, for degrees?
    phase: u32,
    /// Left limit
    left: i32,
    /// Right limit
    right: i32,
}

impl ThreadingOperation {
    pub fn new() -> ThreadingOperation {
        ThreadingOperation {
            thread: ThreadSize::TPI(18),
            phase: 0,
            left: 0,
            right: 0,
        }
    }
}

impl MenuItem for ThreadingOperation {
    fn run(&mut self, r: &mut MenuResources) {
        r.reload_stepper_settings();

        // let (left, status) = limits::capture_limit(r, "Left");
        // if let NavStatus::Exit = status {
        //     return;
        // }
        //
        let (right, status) = limits::capture_limit(r, "Right");
        if let NavStatus::Exit = status {
            return;
        }

        // FIXME: unwraps... should require setting limits!
        //self.left = left.unwrap();
        //self.right = right.unwrap();
        // FIXME: traversal speed?
        let steps_per_inch = settings::steps_per_inch(r.flash) as i32;

        let speed = ((10 * steps_per_inch) << 8) / 60;
        r.shared
          .stepper
          .lock(|s| s.set_speed(speed as u32))
          .unwrap();

        self.left = -steps_per_inch;
        self.right = 0;

        // FIXME: select thread size

        // Main thread cutting thread
        loop {
            return_to(r, self.right);
            if run_wait_next_operation(r, "Start cutting?") == NavStatus::Exit {
                // FIXME: could loose thread syncing if exit here...
                break;
            }

            cut_thread_to(r, self.thread, self.left);
            if run_wait_next_operation(r, "Retract?") == NavStatus::Exit {
                // FIXME: could loose thread syncing if exit here...
                break;
            }
        }

    }
}

/// Wait until operator signals to continue thread cutting by pressing `Fast` button.
fn run_wait_next_operation(r: &mut MenuResources, label: &str) -> NavStatus {
    r.display.clear();
    r.display.position(0, 0);
    write!(r.display, "{}", label).unwrap();
    let mut nav = Navigation::new();
    loop {
        // We use `Fast` button for continuing the operation instead of typical `Encoder` button.
        let event = r.controls.read_event();
        match nav.check(r.estop, event) {
            Some(NavStatus::Exit) => return NavStatus::Exit,
            Some(NavStatus::Select) => return NavStatus::Select,
            None if matches!(event, Event::Pressed(Button::Fast)) => return NavStatus::Select,
            _ => {}
        }
    }
}

/// Return stepper motor to the starting position
fn return_to(r: &mut MenuResources, position: i32) {
    r.display.clear();
    r.display.position(0, 0);
    write!(r.display, "Retracting...").unwrap();
    r.shared
      .stepper
      .lock(|s: &mut Stepper<StepperDriverImpl>| s.move_to(position))
      .unwrap();
    steputil::wait_stopped(&mut r.shared);
}

fn cut_thread_to(r: &mut MenuResources, thread: ThreadSize, position: i32) {
    r.display.clear();
    r.display.position(0, 0);
    write!(r.display, "Cutting...").unwrap();

    // FIXME: should start when FAST is pressed
    let rpm: u32 = r.shared.hall.lock(|hall| hall.rpm());
    let steps_per_inch = settings::steps_per_inch(r.flash);
    let steps_per_thread = thread.to_steps_per_thread(steps_per_inch);
    let result = r.shared
      .stepper
      .lock(|s: &mut Stepper<StepperDriverImpl>| s.thread_start(position, steps_per_thread, rpm));

    if let Err(err) = result {
        match err {
            StepperError::StepgenError(StepgenError::TooSlow) => {
                write!(r.display, "RPM is too low! ").unwrap()
            }
            StepperError::StepgenError(StepgenError::TooFast) => {
                write!(r.display, "RPM is too high!").unwrap()
            }
            _ => unreachable!(),
        };
        // FIXME: loop?
    }

    // FIXME: allow using encoder to adjust the thread phase
    loop {
        let (state, last_error) = r.shared.stepper.lock(|s| (s.state(), s.last_error_degrees()));
        if state == stepper::State::Stopped {
            break;
        }

        // Display thread cutting error
        r.display.position(0, 1);
        let sign = if last_error < 0 { "-" } else { " " };
        let le = last_error.abs();
        write!(r.display, "Err: {}{}.{}", sign, le / 10, le % 10).unwrap();
    }
}
