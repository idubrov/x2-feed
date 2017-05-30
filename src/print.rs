use hd44780;
use hw::lcd;

#[derive(Clone, Copy, PartialEq)]
pub enum Alignment {
    Left,
    Right,
}

pub fn print_formatted(lcd: &hd44780::HD44780<lcd::LcdHw>, number: u32,
                       alignment: Alignment, padding: usize) {
    const MAX_LEN: usize = 10;
    const RADIX: u32 = 10;
    let mut buf: [u8; MAX_LEN] = [0; MAX_LEN];

    let mut pos = MAX_LEN;
    let mut n = number;

    loop {
        let m = n;
        n /= RADIX;
        let c = (m - RADIX * n) as u8;
        pos -= 1;
        buf[pos] = if c < 10 { c + ('0' as u8) } else { c + ('a' as u8) - 10 };

        if n == 0 {
            break;
        }
    }

    let digits = MAX_LEN - pos;
    if alignment == Alignment::Right {
        for _ in digits..padding {
            lcd.write(' ' as u8);
        }
    }

    while pos < MAX_LEN {
        lcd.write(buf[pos]);
        pos += 1;
    }

    if alignment == Alignment::Left {
        for _ in digits..padding {
            lcd.write(' ' as u8);
        }
    }
}