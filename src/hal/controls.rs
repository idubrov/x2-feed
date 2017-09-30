use bare_metal::Peripheral;
use stm32f103xx::gpioa;
use core::ops::Deref;
use stm32_hal::gpio::Port;

#[derive(Clone, Copy, Debug)]
pub struct ControlsState {
    pub left: bool,
    pub right: bool,
    pub fast: bool,
    pub button: bool
}

#[derive(Clone, Copy, Debug)]
pub enum Button {
    Left, Right, Fast, Encoder
}

const BUTTONS: [Button; 4] = [Button::Left, Button::Right, Button::Fast, Button::Encoder];

#[derive(Clone, Copy, Debug)]
pub enum Event {
    Pressed(Button),
    Unpressed(Button),
    None
}

pub struct Controls<Port: 'static> {
    port: Peripheral<Port>,
    pins: [u8; 4],
    last: [bool; 4]
}
unsafe impl <Port> Send for Controls<Port> { }

impl <Port> Controls<Port> where Port: Deref<Target = gpioa::RegisterBlock> {
    pub const fn new(port: Peripheral<Port>, left: u8, right: u8, fast: u8, encoder: u8) -> Controls<Port> {
        Controls {
            port,
            pins: [left, right, fast, encoder],
            last: [false; 4]
        }
    }

    pub fn init(&self) {
        let port = self.port();
        for pin in &self.pins {
            port.pin_config(*pin as usize).input().floating();
        }
    }

    pub fn state(&self) -> ControlsState {
        let values = self.port().idr.read().bits();

        let mut pressed: [bool; 4] = [false; 4];
        for (idx, pin) in self.pins.iter().enumerate() {
            pressed[idx] = (values & (1 << *pin)) != 0;
        }

        ControlsState {
            left: pressed[0],
            right: pressed[1],
            fast: pressed[2],
            button: pressed[3]
        }
    }

    /// Check buttons state, compare with the previous one and return Pressed/Unpressed event if
    /// state was changed.
    /// # Note
    /// Only handles one pin at a time.
    pub fn read_event(&mut self) -> Event {
        let data = self.port().idr.read().bits();

        for (idx, pin) in self.pins.iter().enumerate() {
            let state = (data & (1 << *pin)) != 0;
            if state && !self.last[idx] {
                self.last[idx] = true;
                return Event::Pressed(BUTTONS[idx])
            } else if !state && self.last[idx] {
                self.last[idx] = false;
                return Event::Unpressed(BUTTONS[idx])
            }
        }
        Event::None
    }


    fn port(&self) -> &Port {
        unsafe { &*self.port.get() }
    }
}
