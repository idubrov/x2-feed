extern crate lcd;

use hw::config::lcd::{PORT, RS, RW, E, DATA};
use stm32f103xx::{GPIOB, RCC};
use stm32_extras::GPIOExtras;

pub struct Screen;

impl Screen {
    pub fn init(&self, rcc: &RCC) {
        rcc.apb2enr.modify(|_, w| w.iopben().enabled());

        let port = self.port();

        // Init data port, 4 bits
        for i in 0..4 {
            port.write_pin(DATA + i, false);
            port.pin_config(DATA + i).push_pull().output2();
        }

        // Init control ports
        port.pin_config(RS).push_pull().output2();
        port.pin_config(RW).push_pull().output2();
        port.pin_config(E).push_pull().output2();

        port.write_pin(RS, false);
        port.write_pin(RW, false);
        port.write_pin(E, false);
    }

    fn port(&self) -> &'static PORT {
        unsafe { &*GPIOB.get() }
    }
}


impl lcd::Hardware for Screen {
    fn rs(&self, bit: bool) {
        self.port().write_pin(RS, bit);
    }

    fn enable(&self, bit: bool) {
        self.port().write_pin(E, bit);
    }

    fn data(&self, data: u8) {
        self.port().write_pin_range(DATA, 4, u16::from(data));
    }
}

impl lcd::Delay for Screen {
    fn delay_us(&self, delay_usec: u32) {
        ::hw::delay::us(delay_usec);
    }
}