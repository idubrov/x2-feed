//! Custom LCD characters for the powerfeed
extern crate lcd;
use hw::hwlcd;

pub const LEFT : char = 1 as char;
pub const RIGHT : char = 2 as char;
pub const FAST_LEFT : char = 3 as char;
pub const FAST_RIGHT : char = 4 as char;

#[cfg_attr(feature = "cargo-clippy", allow(unreadable_literal))]
const LEFT_CHAR: [u8; 8] = [
    0b00000,
    0b00001,
    0b00010,
    0b00100,
    0b01000,
    0b00100,
    0b00010,
    0b00001
];

#[cfg_attr(feature = "cargo-clippy", allow(unreadable_literal))]
const FAST_LEFT_CHAR: [u8; 8] = [
    0b00000,
    0b00001,
    0b00011,
    0b00111,
    0b01111,
    0b00111,
    0b00011,
    0b00001
];

#[cfg_attr(feature = "cargo-clippy", allow(unreadable_literal))]
const RIGHT_CHAR: [u8; 8] = [
    0b00000,
    0b01000,
    0b00100,
    0b00010,
    0b00001,
    0b00010,
    0b00100,
    0b01000
];

#[cfg_attr(feature = "cargo-clippy", allow(unreadable_literal))]
const FAST_RIGHT_CHAR: [u8; 8] = [
    0b00000,
    0b01000,
    0b01100,
    0b01110,
    0b01111,
    0b01110,
    0b01100,
    0b01000
];

pub fn upload_characters(lcd : &mut lcd::HD44780<hwlcd::LcdHw>) {
    lcd.upload_character(1, LEFT_CHAR);
    lcd.upload_character(2, RIGHT_CHAR);
    lcd.upload_character(3, FAST_LEFT_CHAR);
    lcd.upload_character(4, FAST_RIGHT_CHAR);
}