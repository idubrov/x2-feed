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

use rtfm::{P0, P4, T0, C4, T4, TMax, Resource};
use stm32f103xx::interrupt::Tim1UpTim10;
use stm32f103xx::{Syst};
use hw::{delay, clock, lcd, led, encoder, driver, stepper, ESTOP};
use core::cell::RefCell;

mod timer;
mod hw;

type Tim4Timer = timer::Timer<stm32f103xx::Tim4>;

static TIMER: Tim4Timer = Tim4Timer::new();
static LCD: lcd::Lcd = lcd::Lcd::new();
static LED: led::Led = led::Led::new();
static ENC: encoder::Encoder = encoder::Encoder::new();
static DRIVER: driver::DriverRef = driver::DriverRef::new();

static STEPPER: Resource<RefCell<stepper::Stepper>, C4> = Resource::new(RefCell::new(stepper::Stepper::new()));

tasks!(stm32f103xx, {
step_completed: Task {
        interrupt: Tim1UpTim10,
        priority: P4,
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
    TIM1: Peripheral {
        register_block: Tim1,
        ceiling: C4,
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
        ceiling: C4,
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
    let driver = DRIVER.materialize(&tim1, &gpioa);

    clock::setup(&rcc, &syst, &flash);

    // FIXME: ...
    LED.init(&gpioa, &rcc);

    TIMER.init(&tim4, &rcc);
    LCD.init(&gpiob, &rcc);
    ENC.init(&tim3, &gpioa, &rcc);
    ENC.set_limit(&tim3, 20);
    driver.init(&rcc);
    driver.release();

    let stepper = STEPPER.access(priority, threshold);
    stepper.borrow_mut().reset();
    stepper.borrow_mut().set_speed((4 * 200 * 16) << 8);
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

fn stepper_command<CB>(priority: &P0, threshold: &T0, cb: CB)
    where
        CB: FnOnce(core::cell::RefMut<stepper::Stepper>, &driver::Driver) {
    threshold.raise(
        &STEPPER, |threshold| {
            let gpioa = GPIOA.access(priority, threshold);
            let tim1 = TIM1.access(priority, threshold);
            let driver = DRIVER.materialize(&tim1, &gpioa);
            let stepper = STEPPER.access(priority, threshold);
            cb(stepper.borrow_mut(), &driver);
        }
    );
}

fn idle(priority: P0, threshold: T0) -> ! {
    let ipm = 10;
    let pitch = 16;
    let microsteps = 16;
    let slow_speed = (ipm * pitch * 200 * microsteps / 60) << 8;
    let fast_speed = slow_speed * 3;

    let syst = SYST.access(&priority, &threshold);
    // FIXME: ..
    let gpioa = unsafe { &(*stm32f103xx::GPIOA.get()) }; //GPIOA.access(&priority, &threshold);
    let tim1 = unsafe { & (*stm32f103xx::TIM1.get()) };
    let gpiob = GPIOB.access(&priority, &threshold);
    let tim3 = TIM3.access(&priority, &threshold);
    let lcd = LCD.materialize(&syst, &gpiob);
    let mut was_moving = false;
    let mut last_speed = 0;
    lcd.init();
    lcd.display(hd44780::DisplayMode::DisplayOn, hd44780::DisplayCursor::CursorOff, hd44780::DisplayBlink::BlinkOff);
    lcd.entry_mode(hd44780::EntryModeDirection::EntryRight, hd44780::EntryModeShift::NoShift);

    // Need to wait at least 40ms after Vcc rises to 2.7V
    // STM32 could start much earlier than that
    ::delay::ms(&syst, 50);

    loop {
        lcd.position(0, 0);

        let left = ::hw::LEFT.get(&gpioa) == 1;
        let right = ::hw::RIGHT.get(&gpioa) == 1;
        let fast = ::hw::FAST.get(&gpioa) == 1;

        let speed = if fast { fast_speed } else { slow_speed };
        if last_speed != speed {
            stepper_command(&priority, &threshold, |mut s, d| { s.set_speed(speed); });
            last_speed = speed;
        }

        match (was_moving, left, right) {
            (false, true, false) => {
                stepper_command(&priority, &threshold, |mut s, d| { s.move_to(&syst, d, -1000000000); });
                was_moving = true;
            },

            (false, false, true) => {
                stepper_command(&priority, &threshold, |mut s, d| { s.move_to(&syst, d, 1000000000); });
                was_moving = true;
            },

            (true, false, false) => {
                stepper_command(&priority, &threshold, |mut s, _| s.stop());
                // FIXME: should wait till stopped!
                was_moving = false;
            },

            _ => {}
        }

        lcd.print(if gpioa.idr.read().idr1().is_set() { "1" } else { "0" });
        lcd.print(if gpioa.idr.read().idr2().is_set() { "1" } else { "0" });
        lcd.print(if gpioa.idr.read().idr3().is_set() { "1" } else { "0" });


        lcd.print(if was_moving { "1" } else { "0" });
        lcd.print(if left { "1" } else { "0" });
        lcd.print(if right { "1" } else { "0" });


        lcd.position(0, 1);
        if ESTOP.get(&gpiob) == 0 {
            estop(&syst, &lcd);
        }
        //lcd.print(if gpioa.idr.read().idr0().is_set() { "1 " } else { "0 " });

        let cnt = ENC.current(&tim3);
        let cnt0 = cnt % 10;
        let cnt1 = (cnt / 10) % 10;
        let cnt2 = (cnt / 100) % 10;
        lcd.write((cnt2 as u8) + ('0' as u8));
        lcd.write((cnt1 as u8) + ('0' as u8));
        lcd.write((cnt0 as u8) + ('0' as u8));

        lcd.write(' ' as u8);
        let cnt = tim1.cnt.read().bits();
        let cnt0 = cnt % 10;
        let cnt1 = (cnt / 10) % 10;
        let cnt2 = (cnt / 100) % 10;
        lcd.write((cnt2 as u8) + ('0' as u8));
        lcd.write((cnt1 as u8) + ('0' as u8));
        lcd.write((cnt0 as u8) + ('0' as u8));
    }
}

fn step_completed(mut _task: Tim1UpTim10, ref priority: P4, ref threshold: T4) {
    let gpioa = GPIOA.access(priority, threshold);
    let tim1 = TIM1.access(priority, threshold);
    let driver = DRIVER.materialize(&tim1, &gpioa);
    let stepper = STEPPER.access(priority, threshold);

    if tim1.sr.read().uif().is_pending() {
        tim1.sr.modify(|_, w| w.uif().clear());
        stepper.borrow_mut().step_completed(&driver);
    }
}