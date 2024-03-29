use stm32f1xx_hal::gpio::{ErasedPin, Floating, Input};

type Pin = ErasedPin<Input<Floating>>;

#[derive(Clone, Copy, Debug)]
pub struct ControlsState {
    pub left: bool,
    pub right: bool,
    pub fast: bool,
    pub button: bool,
}

#[derive(Clone, Copy, Debug)]
pub enum Button {
    Left,
    Right,
    Fast,
    Encoder,
}

const BUTTONS: [Button; 4] = [Button::Left, Button::Right, Button::Fast, Button::Encoder];

#[derive(Clone, Copy, Debug)]
pub enum Event {
    Pressed(Button),
    Unpressed(Button),
    None,
}

pub struct Controls {
    pins: [Pin; 4],
    last: [bool; 4],
}

impl Controls {
    pub fn new(left: Pin, right: Pin, fast: Pin, encoder: Pin) -> Controls {
        Controls {
            pins: [left, right, fast, encoder],
            last: [false; 4],
        }
    }

    pub fn state(&self) -> ControlsState {
        let mut pressed: [bool; 4] = [false; 4];
        for (idx, pin) in self.pins.iter().enumerate() {
            pressed[idx] = pin.is_high();
        }

        ControlsState {
            left: pressed[0],
            right: pressed[1],
            fast: pressed[2],
            button: pressed[3],
        }
    }

    /// Check buttons state, compare with the previous one and return Pressed/Unpressed event if
    /// state was changed.
    /// # Note
    /// Only handles one pin at a time.
    pub fn read_event(&mut self) -> Event {
        for (idx, pin) in self.pins.iter().enumerate() {
            let state = pin.is_high();
            if state && !self.last[idx] {
                self.last[idx] = true;
                return Event::Pressed(BUTTONS[idx]);
            } else if !state && self.last[idx] {
                self.last[idx] = false;
                return Event::Unpressed(BUTTONS[idx]);
            }
        }
        Event::None
    }
}
