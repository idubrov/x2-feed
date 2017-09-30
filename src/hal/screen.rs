use bare_metal::Peripheral;
use stm32f103xx::gpioa;
use core::ops::Deref;
use stm32_hal::gpio::Port;
use lcd;

pub struct Screen<Port: 'static> {
    port: Peripheral<Port>,
    rs: usize,
    rw: usize,
    e: usize,
    data: usize
}

impl <Port> Screen<Port> where Port: Deref<Target = gpioa::RegisterBlock> {
    pub const fn new(port: Peripheral<Port>, rs: usize, rw: usize, e: usize, data: usize) -> Screen<Port> {
        Screen { port, rs, rw, e, data }
    }

    pub fn init(&self) {
        let port = self.port();

        // Init data port, 4 bits
        for i in 0..4 {
            port.write_pin(self.data + i, false);
            port.pin_config(self.data + i).push_pull().output2();
        }

        // Init control ports
        port.pin_config(self.rs).push_pull().output2();
        port.pin_config(self.rw).push_pull().output2();
        port.pin_config(self.e).push_pull().output2();

        port.write_pin(self.rs, false);
        port.write_pin(self.rw, false);
        port.write_pin(self.e, false);
    }

    fn port(&self) -> &Port {
        unsafe { &*self.port.get() }
    }
}


impl <Port: Deref<Target = gpioa::RegisterBlock>> lcd::Hardware for Screen<Port> {
    fn rs(&self, bit: bool) {
        self.port().write_pin(self.rs, bit);
    }

    fn enable(&self, bit: bool) {
        self.port().write_pin(self.e, bit);
    }

    fn data(&self, data: u8) {
        self.port().write_pin_range(self.data, 4, u16::from(data));
    }
}

impl <Port> lcd::Delay for Screen<Port> where Port: Deref<Target = gpioa::RegisterBlock> {
    fn delay_us(&self, delay_usec: u32) {
        super::delay::us(delay_usec);
    }
}