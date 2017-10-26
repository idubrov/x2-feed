use stm32f103xx::{GPIOA, GPIOB};
use hal;
use lcd;

pub const STEPS_PER_ROTATION: u32 = 200;

pub type ScreenResource = hal::Screen<GPIOB>;
pub type Display<'a> = lcd::Display<'a, ScreenResource>;
pub const fn screen() -> ScreenResource {
    // PB1 is RS, PB10 is RW, PB11 is E, PB12-PB15 are DB4-DB7
    ScreenResource::new(GPIOB, 1, 10, 11, 12)
}

pub type QuadEncoderResource = hal::QuadEncoder<GPIOA>;
pub const fn encoder() -> QuadEncoderResource {
    // PA5 is button (not used), PA6 is DT, PA7 is CLK
    QuadEncoderResource::new(GPIOA, 6, 7)
}

// Emergency stop
// FIXME: make a HAL resource...
pub const ESTOP_PIN: usize = 0;
pub const DRIVER_ENABLE_PIN: usize = 10;

// Stepper driver
pub type StepperDriverResource = hal::StepperDriverImpl<GPIOA>;
pub const fn driver() -> StepperDriverResource {
    // PA8 is step, PA9 is dir, PA10 is enable, PA11 is reset
    StepperDriverResource::new(GPIOA, 8, 9, 10, 11)
}

pub type LedResource = hal::Led<GPIOA>;
pub const fn led() -> LedResource {
    // PA4 is LED
    LedResource::new(GPIOA, 4)
}

pub type ControlsResource = hal::Controls<GPIOA>;
pub const fn controls() -> ControlsResource {
    // PA1 is left, PA2 is right, PA3 is fast, PA5 is encoder button
    ControlsResource::new(GPIOA, 1, 2, 3, 5)
}

pub type RpmSensorResource = hal::RpmSensor<GPIOA>;
pub const fn hall() -> RpmSensorResource {
    // PA0 is hall encoder
    RpmSensorResource::new(GPIOA, 0)
}