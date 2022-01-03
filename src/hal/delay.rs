use stm32f1::stm32f103::SYST;

/// Delay for a given amount of microseconds. Should not be used for precise delays.
/// Assumes SYST ticks with frequency of 9Mhz and the reload value of 0xffffff (maximum).
/// `delay` must be less than `0x8000_0000` (SYST is only 24-bit)
pub fn us(delay: u32) {
    // Essentialy, we do modulo 24-bit arithmetic.
    // SYST is running at the frequency of AHB/8, which is 9Mhz (72Mhz SYSCLK)
    let stop_at: u32 = SYST::get_current().wrapping_sub((delay * 9) - 1);
    // Run while `stop_at` is less than the counter value ("sign" bit of the difference is zero)
    // "sign" bit is 24th bit as SYST is 24-bit timer
    // Run while "(current - (start - delay)) | mod 0x800000 >= 0"
    while (SYST::get_current().wrapping_sub(stop_at) & 0x0080_0000) == 0 {}
}

pub fn ms(delay: u32) {
    for _ in 0..delay {
        us(1000);
    }
}

/// Current tick in 1/9th of microsecond (SYST frequency is 9Mhz)
pub fn current() -> u32 {
    SYST::get_current()
}

/// Utility to measure durations using SysTick timer. For correctness requires that
/// `Duration::duration` to be called often, with time between two calls less than
/// time required to overflow SysTick timer.
pub struct Duration {
    last: u32,
    duration: u32,
}

impl Duration {
    pub fn new() -> Self {
        Duration {
            last: current(),
            duration: 0,
        }
    }

    /// Check the duration passed since this helper was created. Note that this function should be
    /// called with intervals between calls less than time required for SysTick timer to go through
    /// the whole interval
    pub fn duration(&mut self) -> u32 {
        let current = current();
        self.duration += (self.last.wrapping_sub(current) & 0xffffff) / 9;
        self.last = current;
        self.duration
    }
}
