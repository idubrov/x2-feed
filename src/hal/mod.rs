pub mod clock;
mod controls;
pub mod delay;
mod driver;
mod encoder;
mod led;
mod rpm;
mod screen;

pub const STEPS_PER_ROTATION: u32 = 200;

pub use self::clock::FREQUENCY;
pub use self::controls::{Button, Controls, ControlsState, Event};
pub use self::driver::DRIVER_TICK_FREQUENCY;
pub use self::driver::{StepperDriver, StepperDriverImpl};
pub use self::encoder::QuadEncoder;
pub use self::led::Led;
pub use self::rpm::RpmSensor;
pub use self::screen::Screen;

pub type Display = lcd::Display<Screen>;
