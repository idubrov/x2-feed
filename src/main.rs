#![no_std]
#![no_main]
//#![deny(warnings)]

//! Stepper-motor based power feed for X2 mill.
//!
//! The following features are supported:
//!
//! 1. Power feeding in both directions.
//! 1. Two feed modes: "slow" and "fast".
//! 1. Setting feed speed via rotary encoder (both "slow" and "fast").
//! 1. Spindle tachometer via hall sensor.
//! 1. Emergency stop mode: halts stepper motor driver when emergency stop is pressed.
//! 1. Screen screen displays current spindle speed and feed speed.
//!
//! # PCB
//! See PCB (Eagle CAD) in the [pcb/](pcb/) directory.

use crate::hal::{Display, Screen};
use core::panic::PanicInfo;
use stm32_hal::gpio::Port;
use stm32f1::stm32f103::Peripherals;

mod config;
mod font;
mod hal;
mod menu;
mod settings;
mod stepper;
mod threads;

#[rtic::app(device = stm32f1::stm32f103, peripherals = true)]
mod app {
    use crate::hal::{
        clock, delay, Controls, Display, Led, QuadEncoder, RpmSensor, Screen, StepperDriverImpl,
        DRIVER_TICK_FREQUENCY,
    };
    use crate::menu::{MainMenu, MenuItem, MenuResources};
    use crate::stepper::Stepper;
    use eeprom::EEPROM;
    use stm32_hal::gpio::{Pin, Port};
    use stm32f1::stm32f103::FLASH;

    #[shared]
    struct Shared {
        stepper: Stepper<StepperDriverImpl>,
        hall: RpmSensor,
    }

    #[local]
    struct Local {
        display: Display,
        led: Led,
        encoder: QuadEncoder,
        controls: Controls,
        flash: FLASH,
        estop: Pin,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let mut core: cortex_m::Peripherals = cx.core;
        let mut peripherals: stm32f1::stm32f103::Peripherals = cx.device;

        clock::setup(&mut peripherals.RCC, &mut core.SYST, &mut peripherals.FLASH);

        // Enable peripherals
        peripherals.RCC.apb1enr.modify(|_, w| w.tim2en().enabled());
        peripherals.RCC.apb1enr.modify(|_, w| w.tim3en().enabled());
        peripherals.RCC.apb2enr.modify(|_, w| w.tim1en().enabled());
        peripherals.RCC.apb2enr.modify(|_, w| w.iopaen().enabled());
        peripherals.RCC.apb2enr.modify(|_, w| w.iopben().enabled());
        peripherals.RCC.apb2enr.modify(|_, w| w.afioen().enabled());

        let [
            // PA0 is hall encoder
            hall_pin,
            // PA1 is left
            left_btn,
            // PA2 is right
            right_btn,
            // PA3 is fast
            fast_btn,
            // PA4 is led
            led_pin,
            // PA5 is encoder button
            encoder_btn,
            // PA6 is DT
            encoder_dt_pin,
            // PA7 is CLK
            encoder_clk_pin,
            // PA8 is step
            step_pin,
            // PA9 is dir
            dir_pin,
            // PA10 is enable
            enable_pin,
            // PA11 is reset
            reset_pin,
            pa12,
            pa13,
            pa14,
            pa15,
        ] = peripherals.GPIOA.into_bitband();

        let [
            estop,
            // PB1 is RS
            rs_pin,
            pb2,
            pb3,
            pb4,
            pb5,
            pb6,
            pb7,
            pb8,
            pb9,
            // PB10 is RW
            rw_pin,
            // PB11 is E
            e_pin,
            // PB12-PB15 are DB4-DB7
            db4,
            db5,
            db6,
            db7,
        ] = peripherals.GPIOB.into_bitband();
        passivate([
            pa12, pa13, pa14, pa15, pb2, pb3, pb4, pb5, pb6, pb7, pb8, pb9,
        ]);

        estop.config().floating();

        // Initialize peripherals
        let driver =
            StepperDriverImpl::new(peripherals.TIM1, step_pin, dir_pin, enable_pin, reset_pin);
        let led = Led::new(led_pin);
        let screen = Screen::new(rs_pin, rw_pin, e_pin, [db4, db5, db6, db7]);
        let encoder = QuadEncoder::new(peripherals.TIM3, encoder_dt_pin, encoder_clk_pin);
        let hall = RpmSensor::new(peripherals.TIM2, hall_pin);
        let stepper = Stepper::new(DRIVER_TICK_FREQUENCY, driver);
        let mut display = Display::new(screen);
        let controls = Controls::new(left_btn, right_btn, fast_btn, encoder_btn);

        // Initialize EEPROM emulation
        peripherals.FLASH.eeprom().init().unwrap();
        let flash = peripherals.FLASH;

        // LCD device init
        // Need to wait at least 40ms after Vcc rises to 2.7V
        // STM32 could start much earlier than that
        delay::ms(50);

        crate::init_display(&mut display);
        (
            Shared { stepper, hall },
            Local {
                flash,
                display,
                led,
                encoder,
                controls,
                estop,
            },
            init::Monotonics(),
        )
    }

    #[idle(local = [led, encoder, controls, display, flash, estop], shared = [stepper, hall])]
    fn idle(context: idle::Context) -> ! {
        let mut r = MenuResources {
            encoder: context.local.encoder,
            display: context.local.display,
            controls: context.local.controls,
            flash: context.local.flash,
            shared: context.shared,
            estop: context.local.estop,
            driver_freq: DRIVER_TICK_FREQUENCY,
        };

        let mut menu = MainMenu::new(&mut r);
        loop {
            menu.run(&mut r);
        }
    }

    fn passivate<const T: usize>(pins: [Pin; T]) {
        for pin in pins {
            pin.config().input().pull_up_down();
            pin.on();
        }
    }

    #[task(binds = TIM1_UP, priority = 16, shared = [stepper])]
    fn step_completed(mut ctx: step_completed::Context) {
        ctx.shared.stepper.lock(|s| s.interrupt())
    }

    #[task(binds = TIM2, priority = 1, shared = [hall, stepper])]
    fn hall_interrupt(mut ctx: hall_interrupt::Context) {
        let (captured, rpm) = ctx
            .shared
            .hall
            .lock(|h: &mut RpmSensor| (h.interrupt(), h.rpm()));
        if captured {
            // We have captured hall sensor, update thread cutting logic
            ctx.shared
                .stepper
                .lock(|s: &mut Stepper<StepperDriverImpl>| s.spindle_sync(rpm));
        }
    }
}

fn init_display(lcd: &mut Display) {
    lcd.init(lcd::FunctionLine::Line2, lcd::FunctionDots::Dots5x8);
    lcd.display(
        lcd::DisplayMode::DisplayOn,
        lcd::DisplayCursor::CursorOff,
        lcd::DisplayBlink::BlinkOff,
    );
    font::upload_characters(lcd);
    lcd.entry_mode(
        lcd::EntryModeDirection::EntryRight,
        lcd::EntryModeShift::NoShift,
    );
}

#[inline(never)]
#[panic_handler]
pub fn begin_panic_handler(info: &PanicInfo<'_>) -> ! {
    // Immediately disable driver outputs
    let gpioa = unsafe { Peripherals::steal().GPIOA };
    let [_, _, _, _, _, _, _, _, _, _, enable, _, _, _, _, _] = gpioa.into_bitband();
    enable.write(false);

    // Steal GPIOB and create another screen in an attempt to print some info
    let gpiob = unsafe { Peripherals::steal().GPIOB };
    let [_, rs_pin, _, _, _, _, _, _, _, _, rw_pin, e_pin, db4, db5, db6, db7] =
        gpiob.into_bitband();
    let screen = Screen::new(rs_pin, rw_pin, e_pin, [db4, db5, db6, db7]);
    let mut display = Display::new(screen);

    // Print reason on the display
    use core::fmt::Write;
    init_display(&mut display);
    display.position(0, 0);
    write!(display, "{}", info).unwrap();
    display.position(0, 1);
    if let Some(loc) = info.location() {
        write!(display, "{}:{}                ", loc.line(), loc.column()).unwrap();
    }

    loop {
        cortex_m::asm::wfi();
    }
}
