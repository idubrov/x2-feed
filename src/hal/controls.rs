use bare_metal::Peripheral;
use stm32f103xx::gpioa;
use core::ops::Deref;
use stm32_extras::GPIOExtras;

#[derive(Clone, Copy, Debug)]
pub struct State {
    pub left: bool,
    pub right: bool,
    pub fast: bool,
}

pub struct Controls<Port: 'static> {
    port: Peripheral<Port>,
    left: usize,
    right: usize,
    fast: usize
}

impl <Port: Deref<Target = gpioa::RegisterBlock>> Controls<Port> {
    pub const fn new(port: Peripheral<Port>, left: usize, right: usize, fast: usize) -> Controls<Port> {
        Controls { port, left, right, fast }
    }

    pub fn init(&self) {
        let port = self.port();
        port.pin_config(self.left).input().floating();
        port.pin_config(self.right).input().floating();
        port.pin_config(self.fast).input().floating();
    }

    pub fn state(&self) -> State {
        let values = self.port().idr.read().bits();

        let left = (values & (1 << self.left)) != 0;
        let right = (values & (1 << self.right)) != 0;
        let fast = (values & (1 << self.fast)) != 0;

        State { left, right, fast }
    }

    fn port(&self) -> &Port {
        unsafe { &*self.port.get() }
    }
}
