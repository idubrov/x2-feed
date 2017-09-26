use stm32f103xx::RCC;
use hw::config::led::{port, PIN};
use stm32_extras::GPIOExtras;

pub struct Led;

impl Led {
    pub fn init(&self, rcc: &RCC) {
        rcc.apb2enr.modify(|_, w| w.iopaen().enabled());

        let port = port();
        port.pin_config(PIN).open_drain().output2();
        port.write_pin(PIN, true); // off
    }

    pub fn set<'a>(&self, on: bool) {
        port().write_pin(PIN, !on);
    }
}