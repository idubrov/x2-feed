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

pub struct Controls<Port: 'static> {
    port: Peripheral<Port>,
    left: usize,
    right: usize,
    fast: usize,
    button: usize
}
unsafe impl <Port> Send for Controls<Port> { }

impl <Port> Controls<Port> where Port: Deref<Target = gpioa::RegisterBlock> {
    pub const fn new(port: Peripheral<Port>, left: usize, right: usize, fast: usize, button: usize) -> Controls<Port> {
        Controls { port, left, right, fast, button }
    }

    pub fn init(&self) {
        let port = self.port();
        port.pin_config(self.left).input().floating();
        port.pin_config(self.right).input().floating();
        port.pin_config(self.fast).input().floating();
        port.pin_config(self.button).input().floating();
    }

    pub fn state(&self) -> ControlsState {
        let values = self.port().idr.read().bits();

        let left = (values & (1 << self.left)) != 0;
        let right = (values & (1 << self.right)) != 0;
        let fast = (values & (1 << self.fast)) != 0;
        let button = (values & (1 << self.button)) != 0;

        ControlsState { left, right, fast, button }
    }

    fn port(&self) -> &Port {
        unsafe { &*self.port.get() }
    }
}
