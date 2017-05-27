#![feature(const_fn)]
#![feature(used)]
#![feature(struct_field_attributes)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate stm32f103xx;
extern crate stepgen;
extern crate hd44780;

#[macro_use]
extern crate cortex_m_rtfm as rtfm;

use rtfm::{P0, T0, TMax};
use stm32f103xx::interrupt::Tim4;
use stm32f103xx::{Gpioa};
use hw::{gpio, delay, clock, lcd, led};

mod timer;
mod hw;

type Tim4Timer = timer::Timer<stm32f103xx::Tim4>;
static TIMER: Tim4Timer = Tim4Timer::new();

static LCD: lcd::Lcd = lcd::Lcd::new();
static LED: led::Led = led::Led::new();

tasks!(stm32f103xx, {
});

peripherals!(stm32f103xx, {
    RCC: Peripheral {
        register_block: Rcc,
        ceiling: C0,
    },
    SYST: Peripheral {
        register_block: Syst,
        ceiling: C0,
    },
    FLASH: Peripheral {
        register_block: Flash,
        ceiling: C0,
    },
    TIM4: Peripheral {
        register_block: Tim4,
        ceiling: C1,
    },
    GPIOA: Peripheral {
        register_block: Gpioa,
        ceiling: C0,
    },
    GPIOB: Peripheral {
        register_block: Gpiob,
        ceiling: C0,
    },
});

fn init(ref priority: P0, threshold: &TMax) {
    let rcc = RCC.access(priority, threshold);
    let syst = SYST.access(priority, threshold);
    let flash = FLASH.access(priority, threshold);
    let gpioa = GPIOA.access(priority, threshold);
    let gpiob = GPIOB.access(priority, threshold);
    let tim4 = TIM4.access(priority, threshold);

    clock::setup(&rcc, &syst, &flash);

    // FIXME: ...
    LED.init(&gpioa, &rcc);
    /*rcc.apb2enr.modify(|_, w| w.iopaen().enabled());
    gpioa.crl.modify(|_, w| w.cnf4().push().mode4().output());
    gpioa.crl.modify(|_, w| unsafe { w.bits(0b0110 << (4 * 4)) });*/


    TIMER.init(&tim4, &rcc);

    LCD.init(&gpiob, &rcc);
}

#[inline(never)]
#[allow(dead_code)]
#[used]
fn idle(priority: P0, threshold: T0) -> ! {
    let syst = SYST.access(&priority, &threshold);
    let gpioa = GPIOA.access(&priority, &threshold);
    let gpiob = GPIOB.access(&priority, &threshold);
    let lcd = LCD.materialize(&syst, &gpiob);
    lcd.init();
    lcd.display(hd44780::DisplayMode::DisplayOn, hd44780::DisplayCursor::CursorOff, hd44780::DisplayBlink::BlinkOff);
    lcd.entry_mode(hd44780::EntryModeDirection::EntryRight, hd44780::EntryModeShift::NoShift);

    // Need to wait at least 40ms after Vcc rises to 2.7V
    // STM32 could start much earlier than that
    ::delay::ms(&syst, 50);

    lcd.position(0, 0);
    lcd.print("Hello,");

    lcd.position(0, 1);
    lcd.print("World!!!");

    loop {
        LED.set(&gpioa, true);
        delay::ms(&syst, 500);

        LED.set(&gpioa, false);
        delay::ms(&syst, 500);
    }
}

/*
fn tick(mut task: Tim4, ref priority: P1, ref threshold: T1) {
    static STATE: Local<bool, Tim4> = Local::new(false);
    let gpioa = GPIOA.access(priority, threshold);
    let tim4 = TIM4.access(priority, threshold);

    if TIMER.check_pending(&tim4) {
        let state = STATE.borrow_mut(&mut task);
        *state = !*state;
        LED.set(&gpioa, *state);
    }
}
*/

/*
fn main() {
    let gpioa = unsafe { &(*stm32f103xx::GPIOA.get()) };
    let rcc = unsafe { &(*stm32f103xx::RCC.get()) };
    let syst = unsafe { &(*stm32f103xx::SYST.get()) };
    let flash = unsafe { &(*stm32f103xx::FLASH.get()) };
    hprintln!("Hello, world!");

    if rcc.cr.read().pllrdy().is_unlocked() {
        clock::setup(&rcc, &syst, &flash);
    }
    LED.init(&gpioa, &rcc);
    LED.set(&gpioa, true);

    hprintln!("Boo!");
    loop {

    }
}

// As we are not using interrupts, we just register a dummy catch all handler
#[allow(dead_code)]
#[used]
#[link_section = ".rodata.interrupts"]
static INTERRUPTS: [extern "C" fn(); 240] = [default_handler; 240];

extern "C" fn default_handler() {
    cortex_m::asm::bkpt();
}*/