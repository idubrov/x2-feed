mod controls;
pub mod delay;
mod driver;
mod encoder;
mod led;
mod rpm;
mod screen;
mod estop;

pub const FREQUENCY: u32 = 72_000_000;
pub use self::controls::{Button, Controls, ControlsState, Event};
pub use self::driver::DRIVER_TICK_FREQUENCY;
pub use self::driver::{StepperDriver, StepperDriverImpl};
pub use self::encoder::QuadEncoder;
pub use self::led::Led;
pub use self::rpm::RpmSensor;
pub use self::screen::Screen;
pub use self::estop::EStop;

pub type Display = lcd::Display<Screen>;
