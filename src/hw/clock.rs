use stm32f103xx::{RCC, FLASH, SYST};

fn wait_condition<F>(syst: &SYST, f: F) -> bool
    where
        F: Fn() -> bool {
    syst.clear_current();
    while !f() {
        if syst.has_wrapped() {
            return false
        }
    }
    true
}

/// Enables `HSE` oscillator (assumes 8Mhz crystal).
/// Enables `PLL` with multiplier of 9 (72Mhz)
/// Sets up `SYSCLK` to use `PLL` as a source
/// Sets up `SysTick` to run at 1ms period.
pub fn setup(rcc: &RCC, syst: &SYST, flash: &FLASH) {
    if rcc.cr.read().pllrdy().is_locked() {
        panic!("PLL must be unlocked at this moment!");
    }

    // SysTick is AHB/8, which gives us 1Mhz
    syst.set_reload(50_000 - 1); // 50ms timeout ticks
    syst.enable_counter();

    // Use two wait states (48MHz < SYSCLK <= 72MHz)
    flash.acr.modify(|_, w| w.latency().two());

    // Start HSE
    rcc.cr.modify(|_, w| w.hseon().enabled()); // Enable HSE
    if !wait_condition(syst, || rcc.cr.read().hserdy().is_ready()) {
        panic!("HSE failed to start");
    }

    // Configure dividers
    rcc.cfgr.modify(|_, w| w
        .hpre().div1() // AHB clock prescaler
        .ppre1().div2() // APB low-speed prescaler
        .ppre2().div1() // APB high-speed prescaler
        .pllsrc().external() // Use HSE as source for PLL
        .pllxtpre().div1().pllmul().mul9() // /1*9 = 72Mhz
    );

    // Lock PLL
    rcc.cr.modify(|_, w| w.pllon().enabled());
    if !wait_condition(syst, || rcc.cr.read().pllrdy().is_locked()) {
        panic!("PLL failed to lock");
    }

    // Use PLL as a source for SYSCLK
    rcc.cfgr.modify(|_, w| w.sw().pll());
    if !wait_condition(syst, || rcc.cfgr.read().sws().is_pll()) {
        panic!("SYSCLK failed to switch to PLL");
    }

    // Setup SysTick to run at 1ms
    // SysTick is 1/8 AHB (9Mhz)
    syst.set_reload(9_000 - 1);
    syst.clear_current();
}
