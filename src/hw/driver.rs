use stm32f103xx::{Gpioa, Tim1, Rcc};

pub struct Driver {}

impl Driver {
    pub const fn new() -> Driver {
        Driver {}
    }

    pub fn init(&self, tim1: &Tim1, gpioa: &Gpioa, rcc: &Rcc) {
        rcc.apb2enr.modify(|_, w| w.tim1en().enabled());
        rcc.apb2enr.modify(|_, w| w.iopaen().enabled());

        ::hw::STEP.set(gpioa, 1);
        ::hw::DIR.set(gpioa, 1);
        ::hw::ENABLE.set(gpioa, 1);
        ::hw::RESET.set(gpioa, 1);

        // CNF of '1' is floating input
        gpioa.crh.write(|w| w
            .mode8().output50().cnf8().open()
            .mode9().output50().cnf8().open()
            .mode10().output50().cnf8().open()
            .mode11().output50().cnf8().open());
    }
}