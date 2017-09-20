extern crate lcd;

pub mod config; // FIXME: hide..
pub mod gpio;
pub mod clock;
pub mod delay;
mod screen;
mod led;
mod encoder;
pub mod driver;
pub mod stepper;
mod controls;
pub mod hall;

pub use self::screen::Screen;
pub type Display<'a> = lcd::Display<self::screen::ScreenHAL<'a>>;
pub use self::led::Led;
pub use self::encoder::Encoder;
pub use self::controls::Controls;
pub use self::controls::State as ControlsState;
pub use self::driver::Driver;
