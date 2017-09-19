use hw::{LEFT, RIGHT, FAST};

use stm32f103xx::{GPIOA, RCC};

#[derive(Clone, Copy)]
pub struct State {
    pub left: bool,
    pub right: bool,
    pub fast: bool,
}

pub struct Controls;

impl Controls {
    pub fn init(&self, gpioa: &GPIOA, rcc: &RCC) {
        rcc.apb2enr.modify(|_, w| w.iopaen().enabled());

        gpioa.crl.write(|w| w
            // open == 01 == floating input
            .mode1().input().cnf1().open()
            .mode2().input().cnf2().open()
            .mode3().input().cnf3().open()
        );
    }

    pub fn get(&self, gpioa: &GPIOA) -> State {
        let values = gpioa.idr.read().bits();

        let left = ((values >> LEFT.shift) as u16) & LEFT.mask;
        let right = ((values >> RIGHT.shift) as u16) & RIGHT.mask;
        let fast = ((values >> FAST.shift) as u16) & FAST.mask;

        State {
            left: left == 1,
            right: right == 1,
            fast: fast == 1,
        }
    }
}
