use stm32f1xx_hal::gpio::{ErasedPin, Output, PushPull};

type Pin = ErasedPin<Output<PushPull>>;

pub struct Led {
    pin: Pin,
}

impl Led {
    pub fn new(mut pin: Pin) -> Led {
        pin.set_high();
        Led { pin }
    }

    pub fn set(&mut self, on: bool) {
        if on {
            self.pin.set_low();
        } else {
            self.pin.set_high();
        }
    }
}
