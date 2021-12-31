use crate::hal::{Button, Event};
use crate::menu::util::{printable_position, wait_proceed, NavStatus, Navigation};
use crate::menu::{steputil, MenuItem, MenuResources};
use crate::stepper::StepperError;
use crate::{settings, stepper};
use core::fmt::Write;
use rtic::Mutex;
use stepgen::Error as StepgenError;

const METRIC_100TH_PER_INCH: u32 = 2540;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ThreadSize {
    /// Threads per inch
    Tpi(u16),
    /// 1/100 of millimeter
    Metric(u16),
    /// British Association threads
    Ba(u16),
}

impl ThreadSize {
    fn to_steps_per_thread(self, steps_per_inch: u32) -> u32 {
        match self {
            ThreadSize::Tpi(tpi) => steps_per_inch / u32::from(tpi),
            ThreadSize::Metric(metric) => {
                u32::from(metric) * steps_per_inch / METRIC_100TH_PER_INCH
            }
            ThreadSize::Ba(ba) => {
                let metric = british_association_mm(ba);
                u32::from(metric) * steps_per_inch / METRIC_100TH_PER_INCH
            }
        }
    }
}

/// Convert [British Association](https://en.wikipedia.org/wiki/British_Association_screw_threads)
/// thread size to metric.
fn british_association_mm(ba: u16) -> u16 {
    let mut size = 100u16;
    for _ in 0..ba {
        size = (size * 9 + 5) / 10;
    }
    size
}

impl core::fmt::Display for ThreadSize {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match *self {
            ThreadSize::Tpi(tpi) => write!(f, "{: >3} TPI", tpi),
            ThreadSize::Metric(m100th) => write!(f, "{}.{:0>2}mm", m100th / 100, m100th % 100),
            ThreadSize::Ba(size) => write!(f, "{}BA", size),
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
            thread: ThreadSize::Tpi(18),
            phase: 0,
            left: 0,
            right: 0,
        }
    }
}

impl MenuItem for ThreadingOperation {
    fn run(&mut self, r: &mut MenuResources) {
        self.run_impl(r);
    }
}

impl ThreadingOperation {
    fn run_impl(&mut self, r: &mut MenuResources) -> Option<()> {
        r.reload_stepper_settings();
        let steps_per_inch = settings::steps_per_inch(r.flash) as i32;

        self.thread = select_thread_size(r)?;

        // FIXME: allow using feed to go to the desired position precisely?
        r.display.position(0, 0);
        write!(r.display, "At shoulder?    ").unwrap();
        wait_proceed(r);

        self.left = r.shared.stepper.lock(|s| s.position());
        self.right = capture_retract_position(r, steps_per_inch)?;

        // FIXME: warn if not enough space to accelerate?

        // Main thread cutting thread
        loop {
            // Retract to the starting position (if needed)
            if r.shared.stepper.lock(|s| s.position()) != self.right {
                r.display.position(0, 0);
                write!(r.display, "Retracting...   ").unwrap();
                r.shared.stepper.lock(|s| s.move_to(self.right)).unwrap();
                steputil::wait_stopped(&mut r.shared);
            }

            // Cutting thread
            r.display.position(0, 0);
            write!(r.display, "Start cutting?  ").unwrap();
            wait_proceed(r)?;

            cut_thread_to(r, self.thread, self.left);

            // Ask to retract back
            r.display.position(0, 0);
            write!(r.display, "Retract?        ").unwrap();
            wait_proceed(r)?;
        }
    }
}

fn cut_thread_to(r: &mut MenuResources, thread: ThreadSize, position: i32) {
    let rpm: u32 = r.shared.hall.lock(|hall| hall.rpm());
    let steps_per_inch = settings::steps_per_inch(r.flash);
    let steps_per_thread = thread.to_steps_per_thread(steps_per_inch);
    while let Err(err) = r
        .shared
        .stepper
        .lock(|s| s.thread_start(position, steps_per_thread, rpm))
    {
        r.display.position(0, 0);
        match err {
            StepperError::StepgenError(StepgenError::TooSlow) => {
                write!(r.display, "RPM is too low! ").unwrap()
            }
            StepperError::StepgenError(StepgenError::TooFast) => {
                write!(r.display, "RPM is too high!").unwrap()
            }
            _ => unreachable!(),
        };

        r.display.position(0, 1);
        write!(r.display, "Retry?          ").unwrap();
        wait_proceed(r);
    }

    r.display.position(0, 0);
    write!(r.display, "Cutting...      ").unwrap();

    // FIXME: allow using encoder to adjust the thread phase
    loop {
        let (state, last_error) = r
            .shared
            .stepper
            .lock(|s| (s.state(), s.last_error_degrees()));
        if state == stepper::State::Stopped {
            break;
        }

        // Display thread cutting error
        r.display.position(0, 1);
        let sign = if last_error < 0 { "-" } else { " " };
        let le = last_error.abs();
        write!(r.display, "Err: {}{}.{}        ", sign, le / 10, le % 10).unwrap();
    }
}

enum ThreadSystem {
    Inch,
    Metric,
    BritishAssociation,
}

impl core::fmt::Display for ThreadSystem {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str(match self {
            ThreadSystem::Inch => "Inch",
            ThreadSystem::Metric => "Metric",
            ThreadSystem::BritishAssociation => "BA",
        })
    }
}

fn select_thread_size(r: &mut MenuResources) -> Option<ThreadSize> {
    const INCH_THREADS: [u16; 20] = [
        4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 16, 18, 20, 24, 32, 40, 48, 56, 64,
    ];
    const METRIC_THREADS: [u16; 20] = [
        35, 40, 45, 50, 60, 70, 80, 100, 125, 150, 175, 200, 250, 300, 350, 400, 450, 500, 550, 600,
    ];
    const BA_THREADS: [u16; 11] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let kind = super::util::run_selection(
        r,
        "Thread Type?",
        &[
            ThreadSystem::Inch,
            ThreadSystem::Metric,
            ThreadSystem::BritishAssociation,
        ],
        0,
    )?;
    // Select thread size
    match kind {
        ThreadSystem::Inch => {
            // Well, there is also 4 1/2, but we just ignore it for simplicity :)
            let sizes = INCH_THREADS.map(ThreadSize::Tpi);
            let default = sizes
                .iter()
                .copied()
                .position(|t| t == ThreadSize::Tpi(16))
                .unwrap();
            super::util::run_selection(r, "Thread Size?", &sizes, default).copied()
        }
        ThreadSystem::Metric => {
            let sizes = METRIC_THREADS.map(ThreadSize::Metric);
            let default = sizes
                .iter()
                .copied()
                .position(|t| t == ThreadSize::Metric(100))
                .unwrap();
            super::util::run_selection(r, "Thread Size?", &sizes, default).copied()
        }
        ThreadSystem::BritishAssociation => {
            let sizes = BA_THREADS.map(ThreadSize::Ba);
            let default = sizes
                .iter()
                .copied()
                .position(|t| t == ThreadSize::Ba(6))
                .unwrap();
            super::util::run_selection(r, "Thread Size?", &sizes, default).copied()
        }
    }
}

fn capture_retract_position(r: &mut MenuResources, steps_per_inch: i32) -> Option<i32> {
    let mut deltaenc = r.encoder.delta_encoder();
    let mut nav = Navigation::new();

    r.display.clear();
    r.display.position(0, 0);
    write!(r.display, "Retract Distance").unwrap();
    let start = r.shared.stepper.lock(|s| s.position());
    loop {
        let delta = i32::from(deltaenc.delta());
        if delta != 0 {
            // Update stepper position; unit is 0.100 inch
            steputil::move_delta(delta * steps_per_inch / 10, &mut r.shared);
            // FIXME: print "MOVING..."
            steputil::wait_stopped(&mut r.shared);
        }

        let current = r.shared.stepper.lock(|s| s.position());
        let distance = current - start;

        // Update screen
        r.display.position(0, 1);
        write!(
            r.display,
            "{} inch",
            printable_position(distance, steps_per_inch)
        )
        .unwrap();

        let event = r.controls.read_event();
        match nav.check(r.estop, event) {
            Some(NavStatus::Exit) => return None,
            Some(NavStatus::Select) => return Some(current),
            _ if matches!(event, Event::Pressed(Button::Fast)) => return Some(current),
            _ => {}
        }
    }
}
