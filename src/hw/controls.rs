use hw::config::{LEFT, RIGHT, FAST};

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

    // Unsafe accessor for the port. Should only be used when both concurrent access is
    // guaranteed by ownership of mutable/non-mutable Driver reference and access is safe
    // in regard of modifying registers not owned by the Driver.
    fn unsafe_port(&self) -> &'static GPIOA {
        unsafe { &*GPIOA.get() }
    }

    pub fn get(&self) -> State {
        let values = self.unsafe_port().idr.read().bits();

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
