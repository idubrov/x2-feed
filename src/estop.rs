use config::ESTOP_PIN;
use stm32f103xx;
use stm32_hal::gpio::Port;

pub fn check() {
    let gpiob = unsafe { &(*stm32f103xx::GPIOB.get()) };
    if !gpiob.read_pin(ESTOP_PIN) {
        panic!("*E-STOP*");
    }
}
