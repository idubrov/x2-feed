use bare_metal::Peripheral;
use stm32f103xx::gpioa;
use core::ops::Deref;
use stm32_extras::GPIOExtras;

#[derive(Clone, Copy, Debug)]
pub struct ControlsState {
    pub left: bool,
    pub right: bool,
    pub fast: bool,
}

pub trait Controls {
    fn state(&self) -> ControlsState;
}

pub struct ControlsImpl<Port: 'static> {
    port: Peripheral<Port>,
    left: usize,
    right: usize,
    fast: usize
}
unsafe impl <Port> Send for ControlsImpl<Port> { }

impl <Port> ControlsImpl<Port> where Port: Deref<Target = gpioa::RegisterBlock> {
    pub const fn new(port: Peripheral<Port>, left: usize, right: usize, fast: usize) -> ControlsImpl<Port> {
        ControlsImpl { port, left, right, fast }
    }

    pub fn init(&self) {
        let port = self.port();
        port.pin_config(self.left).input().floating();
        port.pin_config(self.right).input().floating();
        port.pin_config(self.fast).input().floating();
    }

    fn port(&self) -> &Port {
        unsafe { &*self.port.get() }
    }
}

impl <Port> Controls for ControlsImpl<Port> where Port: Deref<Target = gpioa::RegisterBlock> {
    fn state(&self) -> ControlsState {
        let values = self.port().idr.read().bits();

        let left = (values & (1 << self.left)) != 0;
        let right = (values & (1 << self.right)) != 0;
        let fast = (values & (1 << self.fast)) != 0;

        ControlsState { left, right, fast }
    }
}
