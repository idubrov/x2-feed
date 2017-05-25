#![feature(const_fn)]
#![feature(used)]
#![feature(struct_field_attributes)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate stm32f103xx;
extern crate stepgen;

#[macro_use]
extern crate cortex_m_rtfm as rtfm;

use rtfm::{Local, P0, P1, T0, T1, TMax};
use stm32f103xx::interrupt::Tim4;
use stm32f103xx::{Gpioc};

mod clock;
mod led;
mod timer;

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
    GPIOC: Peripheral {
        register_block: Gpioc,
        ceiling: C1,
    },
});

type GpiocLed = led::Led<Gpioc>;
static LED: GpiocLed = GpiocLed::new(13, false);

type Tim4Timer = timer::Timer<stm32f103xx::Tim4>;
static TIMER: Tim4Timer = Tim4Timer::new();

fn init(ref priority: P0, threshold: &TMax) {
    let rcc = RCC.access(priority, threshold);
    let syst = SYST.access(priority, threshold);
    let flash = FLASH.access(priority, threshold);
    let gpioc = GPIOC.access(priority, threshold);
    let tim4 = TIM4.access(priority, threshold);

    clock::setup(&rcc, &syst, &flash);
    LED.init(&gpioc, &rcc);
    TIMER.init(&tim4, &rcc);
}

fn idle(_priority: P0, _threshold: T0) -> ! {
    loop {
        rtfm::wfi();
    }
}

fn tick(mut task: Tim4, ref priority: P1, ref threshold: T1) {
    static STATE: Local<bool, Tim4> = Local::new(false);
    let gpioc = GPIOC.access(priority, threshold);
    let tim4 = TIM4.access(priority, threshold);

    if TIMER.check_pending(&tim4) {
        let state = STATE.borrow_mut(&mut task);
        *state = !*state;
        LED.set(&gpioc, *state);
    }
}