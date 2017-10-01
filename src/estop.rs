use config::{Display, ESTOP_PIN, DRIVER_ENABLE_PIN};
use hal::delay;
use stm32f103xx;
use cortex_m;
use core::fmt::Write;
use stm32_hal::gpio::Port;

pub fn check(lcd: &mut Display) {
    let gpiob = unsafe { &(*stm32f103xx::GPIOB.get()) };
    if !gpiob.read_pin(ESTOP_PIN) {
        estop(lcd);
    }
}

fn estop(lcd: &mut Display) -> ! {
    // Immediately disable driver outputs, do it without claiming the driver
    let gpioa = unsafe { &(*stm32f103xx::GPIOA.get()) };
    gpioa.write_pin(DRIVER_ENABLE_PIN, false);

    delay::ms(1); // Wait till power is back to normal

    // Try to print something on the screen
    lcd.position(0, 0);
    write!(lcd, "*E-STOP*").unwrap();
    lcd.position(0, 1);
    write!(lcd, "        ").unwrap();
    loop {
        cortex_m::asm::nop();
    }
}
