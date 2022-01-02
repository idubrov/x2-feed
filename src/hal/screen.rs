use stm32f1xx_hal::gpio::{ErasedPin, Output, PinState, PushPull};

type Pin = ErasedPin<Output<PushPull>>;

pub struct Screen {
    rs: Pin,
    rw: Pin,
    e: Pin,
    data: [Pin; 4],
}

impl Screen {
    pub fn new(rs: Pin, rw: Pin, e: Pin, data: [Pin; 4]) -> Screen {
        Screen { rs, rw, e, data }
    }
}

fn into_pin_state(bit: bool) -> PinState {
    if bit { PinState::High } else { PinState::Low }
}

impl lcd::Hardware for Screen {
    fn rs(&mut self, bit: bool) {
        self.rs.set_state(into_pin_state(bit));
    }

    fn enable(&mut self, bit: bool) {
        self.e.set_state(into_pin_state(bit));
    }

    fn data(&mut self, data: u8) {
        self.data[0].set_state(into_pin_state((data & 1) != 0));
        self.data[1].set_state(into_pin_state(((data >> 1) & 1) != 0));
        self.data[2].set_state(into_pin_state(((data >> 2) & 1) != 0));
        self.data[3].set_state(into_pin_state(((data >> 3) & 1) != 0));
    }

    fn rw(&mut self, bit: bool) {
        self.rw.set_state(into_pin_state(bit));
    }
}

impl lcd::Delay for Screen {
    fn delay_us(&mut self, delay_usec: u32) {
        super::delay::us(delay_usec);
    }
}
