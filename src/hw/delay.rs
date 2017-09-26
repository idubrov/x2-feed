use stm32f103xx::SYST;

// FIXME: 1.8sec is about maximum (SYST is 24-bit timer)
/// Delay for a given amount of microseconds. Should not be used for precise delays.
/// Assumes SYST ticks with frequency of 9Mhz and the reload value of 0xffffff (maximum).
/// `delay` must be less than 0x8000_0000 (SYST is only 24-bit)
pub fn us(delay: u32) {
    // SYST should not be reconfigured by any other code, so it's safe to get it via `unsafe`
    let syst = unsafe { &*SYST.get() };

    // Essentialy, we do modulo 24-bit arithmetic.
    // SYST is running at the frequency of AHB/8, which is 9Mhz (72Mhz SYSCLK)
    let stop_at: u32 = syst.get_current().wrapping_sub((delay * 9) - 1);
    // Run while `stop_at` is less than the counter value ("sign" bit of the difference is zero)
    // "sign" bit is 24th bit as SYST is 24-bit timer
    // Run while "(current - (start - delay)) | mod 0x800000 >= 0"
    while (syst.get_current().wrapping_sub(stop_at) & 0x00800000) == 0 { }
}

pub fn ms(delay: u32) {
    for _ in 0 .. delay {
        us(1000);
    }
}
