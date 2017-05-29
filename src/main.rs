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
use stm32f103xx::{Syst, Gpioa, Gpiob};
use hw::{gpio, delay, clock, lcd, led, encoder, driver, ESTOP};

mod timer;
mod hw;

type Tim4Timer = timer::Timer<stm32f103xx::Tim4>;

static TIMER: Tim4Timer = Tim4Timer::new();
static LCD: lcd::Lcd = lcd::Lcd::new();
static LED: led::Led = led::Led::new();
static ENC: encoder::Encoder = encoder::Encoder::new();
static DRIVER: driver::Driver = driver::Driver::new();

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
    TIM1: Peripheral {
        register_block: Tim1,
        ceiling: C0,
    },
    TIM3: Peripheral {
        register_block: Tim3,
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
    let tim1 = TIM1.access(priority, threshold);
    let tim3 = TIM3.access(priority, threshold);
    let tim4 = TIM4.access(priority, threshold);

    clock::setup(&rcc, &syst, &flash);

    // FIXME: ...
    LED.init(&gpioa, &rcc);

    TIMER.init(&tim4, &rcc);
    LCD.init(&gpiob, &rcc);
    ENC.init(&tim3, &gpioa, &rcc);
    ENC.set_limit(&tim3, 20);
    DRIVER.init(&tim1, &gpioa, &rcc);

    ::hw::DIR.set(&gpioa, 0);
}

fn estop(syst: &Syst, lcd: &hd44780::HD44780<lcd::LcdHw>) -> ! {
    ::delay::ms(syst, 1); // Wait till power is back to normal
    lcd.position(0, 0);
    lcd.print("*E-STOP*");
    lcd.position(0, 1);
    lcd.print("        ");
    loop {
        cortex_m::asm::nop();
    }
}

fn idle(priority: P0, threshold: T0) -> ! {
    let syst = SYST.access(&priority, &threshold);
    let gpioa = GPIOA.access(&priority, &threshold);
    let gpiob = GPIOB.access(&priority, &threshold);
    let tim3 = TIM3.access(&priority, &threshold);
    let lcd = LCD.materialize(&syst, &gpiob);
    lcd.init();
    lcd.display(hd44780::DisplayMode::DisplayOn, hd44780::DisplayCursor::CursorOff, hd44780::DisplayBlink::BlinkOff);
    lcd.entry_mode(hd44780::EntryModeDirection::EntryRight, hd44780::EntryModeShift::NoShift);

    // Need to wait at least 40ms after Vcc rises to 2.7V
    // STM32 could start much earlier than that
    ::delay::ms(&syst, 50);

    loop {
        lcd.position(0, 0);
        lcd.print(if gpioa.idr.read().idr1().is_set() { "1" } else { "0" });
        lcd.print(if gpioa.idr.read().idr2().is_set() { "1" } else { "0" });
        lcd.print(if gpioa.idr.read().idr3().is_set() { "1" } else { "0" });
        lcd.print(if gpiob.idr.read().idr0().is_set() { " 1" } else { " 0" });

        lcd.position(0, 1);
        if ESTOP.get(&gpiob) == 0 {
            estop(&syst, &lcd);
        }
        lcd.print(if gpioa.idr.read().idr0().is_set() { "1 " } else { "0 " });

        let cnt = ENC.current(&tim3);
        let cnt0 = cnt % 10;
        let cnt1 = (cnt / 10) % 10;
        let cnt2 = (cnt / 100) % 10;
        lcd.write((cnt2 as u8) + ('0' as u8));
        lcd.write((cnt1 as u8) + ('0' as u8));
        lcd.write((cnt0 as u8) + ('0' as u8));

        /*::hw::delay::ms(&syst, 10);
        ::hw::STEP.set(&gpioa, 0);
        ::hw::delay::ms(&syst, 1);
        ::hw::STEP.set(&gpioa, 1);*/
    }
}