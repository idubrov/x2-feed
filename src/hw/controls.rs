use hw::config::controls::{LEFT, RIGHT, FAST, PORT};

use stm32f103xx::{GPIOA, RCC};
use stm32_extras::GPIOExtras;

#[derive(Clone, Copy)]
pub struct State {
    pub left: bool,
    pub right: bool,
    pub fast: bool,
}

pub struct Controls;

impl Controls {
    pub fn init(&self, rcc: &RCC) {
        rcc.apb2enr.modify(|_, w| w.iopaen().enabled());

        let port = self.port();

        port.pin_config(LEFT).input().floating();
        port.pin_config(RIGHT).input().floating();
        port.pin_config(FAST).input().floating();
    }

    // Unsafe accessor for the port. Should only be used when both concurrent access is
    // guaranteed by ownership of mutable/non-mutable Driver reference and access is safe
    // in regard of modifying registers not owned by the Driver.
    fn port(&self) -> &'static PORT {
        unsafe { &*GPIOA.get() }
    }

    pub fn get(&self) -> State {
        let values = self.port().idr.read().bits();

        let left = (values & (1 << LEFT)) != 0;
        let right = (values & (1 << RIGHT)) != 0;
        let fast = (values & (1 << FAST)) != 0;

        State { left, right, fast }
    }
}
