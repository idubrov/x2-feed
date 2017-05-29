use stm32f103xx::Syst;

// FIXME: 1.8sec is about maximum
pub fn us(syst: &Syst, delay: u32) {
    let old_reload = syst.get_reload();
    // SysTick is 1/8 AHB (9Mhz)
    let reload = 9 * delay - 1;
    if reload > 0x00ffffff {
        panic!("Delay is too long!");
    }
    syst.set_reload(reload);
    syst.clear_current();
    while !syst.has_wrapped() {}
    syst.set_reload(old_reload);
}

pub fn ms(syst: &Syst, delay: u32) {
    for _ in 0 .. delay {
        us(syst, 1000);
    }
}

pub fn ns(syst: &Syst, delay: u32) {
    // FIXME: !!!
    us(syst, (delay + 999) / 1000);
}