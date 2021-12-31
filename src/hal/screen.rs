use stm32_hal::gpio::Pin;

pub struct Screen {
    rs: Pin,
    rw: Pin,
    e: Pin,
    data: [Pin; 4],
}

impl Screen {
    pub fn new(rs: Pin, rw: Pin, e: Pin, data: [Pin; 4]) -> Screen {
        let screen = Screen { rs, rw, e, data };
        screen.init();
        screen
    }

    fn init(&self) {
        // Init data port, 4 bits
        for i in 0..4 {
            self.data[i].off();
            self.data[i].config().push_pull().output2();
        }

        // Init control ports
        self.rs.config().push_pull().output2();
        self.rw.config().push_pull().output2();
        self.e.config().push_pull().output2();

        self.rs.write(false);
        self.rw.write(false);
        self.e.write(false);
    }
}

impl lcd::Hardware for Screen {
    fn rs(&mut self, bit: bool) {
        self.rs.write(bit);
    }

    fn enable(&mut self, bit: bool) {
        self.e.write(bit);
    }

    fn data(&mut self, data: u8) {
        self.data[0].write((data & 1) != 0);
        self.data[1].write(((data >> 1) & 1) != 0);
        self.data[2].write(((data >> 2) & 1) != 0);
        self.data[3].write(((data >> 3) & 1) != 0);
    }
}

impl lcd::Delay for Screen {
    fn delay_us(&mut self, delay_usec: u32) {
        super::delay::us(delay_usec);
    }
}
