use hw::config::controls::{port, LEFT, RIGHT, FAST};

use stm32f103xx::RCC;
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

        let port = port();

        port.pin_config(LEFT).input().floating();
        port.pin_config(RIGHT).input().floating();
        port.pin_config(FAST).input().floating();
    }

    pub fn get(&self) -> State {
        let values = port().idr.read().bits();

        let left = (values & (1 << LEFT)) != 0;
        let right = (values & (1 << RIGHT)) != 0;
        let fast = (values & (1 << FAST)) != 0;

        State { left, right, fast }
    }
}
