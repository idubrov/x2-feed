use stm32_hal::gpio::Pin;

pub struct Led {
    pin: Pin,
}

impl Led {
    pub fn new(pin: Pin) -> Led {
        let led = Led { pin };
        led.init();
        led
    }

    fn init(&self) {
        self.pin.config().output2();
        self.pin.write(true); // off
    }

    pub fn set(&self, on: bool) {
        self.pin.write(!on);
    }
}
