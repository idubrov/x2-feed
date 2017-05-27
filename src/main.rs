#![feature(const_fn)]
#![feature(used)]
#![feature(struct_field_attributes)]
#![no_std]

#[macro_use]
extern crate cortex_m;
extern crate cortex_m_rt;
extern crate stm32f103xx;
extern crate stepgen;
extern crate hd44780;

#[macro_use]
extern crate cortex_m_rtfm as rtfm;

use rtfm::{Local, P0, P1, T0, T1, TMax};
use stm32f103xx::interrupt::Tim4;
use stm32f103xx::{Gpioa, Gpiob, Syst};

mod clock;
mod led;
mod timer;
mod hal;


type GpioaLed = led::Led<Gpioa>;
static LED: GpioaLed = GpioaLed::new(4, false);

type Tim4Timer = timer::Timer<stm32f103xx::Tim4>;
static TIMER: Tim4Timer = Tim4Timer::new();


type Lcd = hd44780::Lcd<hal::Display<'static>>;
static LCD: Lcd = Lcd::new();

tasks!(stm32f103xx, {
    tick: Task {
        interrupt: Tim4,
        priority: P1,
        enabled: true,
    },
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
        ceiling: C1,
    },
    GPIOB: Peripheral {
        register_block: Gpiob,
        ceiling: C1,
    },
});

fn init(ref priority: P0, threshold: &TMax) {
    let rcc = RCC.access(priority, threshold);
    let syst = SYST.access(priority, threshold);
    let flash = FLASH.access(priority, threshold);
    let gpioa = GPIOA.access(priority, threshold);
    let gpiob = GPIOB.access(priority, threshold);
    let tim4 = TIM4.access(priority, threshold);
    let hw = hal::Display::new(&gpiob);

    clock::setup(&rcc, &syst, &flash);
    LED.init(&gpioa, &rcc);
    LED.set(&gpioa, true);
    TIMER.init(&tim4, &rcc);

    //hal::delay_us(500000);

    hw.init(&rcc);
    let lcd = LCD.borrow(hw);

    lcd.init();
    lcd.display(hd44780::DisplayMode::DisplayOn, hd44780::DisplayCursor::CursorOff, hd44780::DisplayBlink::BlinkOff);
    lcd.entry_mode(hd44780::EntryModeDirection::EntryRight, hd44780::EntryModeShift::NoShift);
    lcd.position(0, 0);
    lcd.print('H');
    lcd.print('e');
    lcd.print('l');
    lcd.print('l');
    lcd.print('o');

    lcd.position(0, 1);
    lcd.print('w');
    lcd.print('o');
    lcd.print('r');
    lcd.print('l');
    lcd.print('d');
    lcd.print('!');

    loop {
        hal::delay_us(500000);
        LED.set(&gpioa, true);

        hal::delay_us(500000);
        LED.set(&gpioa, false);
    }

}

#[inline(never)]
#[allow(dead_code)]
#[used]
fn idle(_priority: P0, _threshold: T0) -> ! {
    loop {
        cortex_m::asm::nop();
    }
}

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