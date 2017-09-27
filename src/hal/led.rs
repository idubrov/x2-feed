use bare_metal::Peripheral;
use stm32f103xx::gpioa;
use core::ops::Deref;
use stm32_extras::GPIOExtras;


pub struct Led<Port: 'static> {
    port: Peripheral<Port>,
    pin: usize
}
unsafe impl <Port> Send for Led<Port> { }

impl <Port> Led<Port> where Port: Deref<Target = gpioa::RegisterBlock> {
    pub const fn new(port: Peripheral<Port>, pin: usize) -> Led<Port> {
        Led { port, pin }
    }

    pub fn init(&self) {
        let port = self.port();
        port.pin_config(self.pin).open_drain().output2();
        port.write_pin(self.pin, true); // off
    }

    pub fn set(&self, on: bool) {
        self.port().write_pin(self.pin, !on);
    }

    fn port(&self) -> &Port {
        unsafe { &*self.port.get() }
    }
}