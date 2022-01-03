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
use stm32f1::stm32f103::Peripherals;
use stm32f1xx_hal::prelude::*;

mod config;
mod font;
mod hal;
mod menu;
mod settings;
mod stepper;
mod threads;

#[rtic::app(device = stm32f1::stm32f103, peripherals = true)]
mod app {
    use crate::hal::{delay, Controls, Display, Led, QuadEncoder, RpmSensor, Screen, StepperDriverImpl, DRIVER_TICK_FREQUENCY, EStop};
    use crate::menu::{LatheMenu, MenuItem, MenuResources, MillMenu};
    use crate::stepper::Stepper;
    use eeprom::EEPROM;
    use stm32f1::stm32f103::{FLASH, Peripherals};
    use stm32f1xx_hal::prelude::*;

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
        estop: EStop,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let mut core: cortex_m::Peripherals = cx.core;
        let peripherals: stm32f1::stm32f103::Peripherals = cx.device;

        // Enable peripherals
        peripherals.RCC.apb1enr.modify(|_, w| w.tim2en().enabled());
        peripherals.RCC.apb1enr.modify(|_, w| w.tim3en().enabled());
        peripherals.RCC.apb2enr.modify(|_, w| w.tim1en().enabled());
        peripherals.RCC.apb2enr.modify(|_, w| w.iopaen().enabled());
        peripherals.RCC.apb2enr.modify(|_, w| w.iopben().enabled());
        peripherals.RCC.apb2enr.modify(|_, w| w.afioen().enabled());

        let flash = unsafe { Peripherals::steal().FLASH };
        let mut flash = flash.constrain();
        let rcc = peripherals.RCC.constrain();
        let _clocks = rcc
          .cfgr
          .use_hse(8.mhz())
          .sysclk(72.mhz())
          .pclk1(36.mhz())
          .pclk2(72.mhz())
          .hclk(72.mhz())
          .freeze(&mut flash.acr);

        // We use our own timer which keeps syst running (since we also use it for measuring delays)
        // Note that SYST is running at the frequency of AHB/8, which is 9Mhz (72Mhz SYSCLK)
        core.SYST.enable_counter();
        core.SYST.set_reload(0x00ff_ffff);

        let mut gpioa = peripherals.GPIOA.split();
        let hall_pin = gpioa.pa0.into_pull_up_input(&mut gpioa.crl);
        let left_btn = gpioa.pa1.erase();
        let right_btn = gpioa.pa2.erase();
        let fast_btn = gpioa.pa3.erase();
        let led_pin = gpioa.pa4.into_push_pull_output(&mut gpioa.crl).erase();
        let encoder_btn = gpioa.pa5.erase();
        let encoder_dt_pin = gpioa.pa6;
        let encoder_clk_pin = gpioa.pa7;
        let step_pin = gpioa.pa8.into_alternate_open_drain(&mut gpioa.crh);
        let dir_pin = gpioa.pa9.into_open_drain_output(&mut gpioa.crh).erase();
        let enable_pin = gpioa.pa10.into_open_drain_output(&mut gpioa.crh).erase();
        let reset_pin = gpioa.pa11.into_open_drain_output(&mut gpioa.crh).erase();

        // "Passivate" unused pins (pull them down), to avoid them floating with noise.
        gpioa.pa12.into_pull_down_input(&mut gpioa.crh);
        // Used by debugger, no need to "passivate"
        //gpioa.pa13.into_pull_down_input(&mut gpioa.crh);
        //gpioa.pa14.into_pull_down_input(&mut gpioa.crh);
        //gpioa.pa15.into_pull_down_input(&mut gpioa.crh);

        let mut gpiob = peripherals.GPIOB.split();
        let rs_pin = gpiob.pb1.into_push_pull_output(&mut gpiob.crl).erase();
        let rw_pin = gpiob.pb10.into_push_pull_output(&mut gpiob.crh).erase();
        let e_pin = gpiob.pb11.into_push_pull_output(&mut gpiob.crh).erase();
        let db4 = gpiob.pb12.into_push_pull_output(&mut gpiob.crh).erase();
        let db5 = gpiob.pb13.into_push_pull_output(&mut gpiob.crh).erase();
        let db6 = gpiob.pb14.into_push_pull_output(&mut gpiob.crh).erase();
        let db7 = gpiob.pb15.into_push_pull_output(&mut gpiob.crh).erase();

        // "Passivate" unused pins (pull them down), to avoid them floating with noise.
        gpiob.pb2.into_pull_down_input(&mut gpiob.crl);
        // Used by debugger, no need to "passivate"
        //gpiob.pb3.into_pull_down_input(&mut gpiob.crl);
        //gpiob.pb4.into_pull_down_input(&mut gpiob.crl);
        gpiob.pb5.into_pull_down_input(&mut gpiob.crl);
        gpiob.pb6.into_pull_down_input(&mut gpiob.crl);
        gpiob.pb7.into_pull_down_input(&mut gpiob.crl);
        gpiob.pb8.into_pull_down_input(&mut gpiob.crh);
        gpiob.pb9.into_pull_down_input(&mut gpiob.crh);

        // Initialize peripherals
        let driver =
            StepperDriverImpl::new(peripherals.TIM1, step_pin, dir_pin, enable_pin, reset_pin);
        let led = Led::new(led_pin);
        let screen = Screen::new(rs_pin, rw_pin, e_pin, [db4, db5, db6, db7]);
        let encoder = QuadEncoder::new(peripherals.TIM3, encoder_dt_pin, encoder_clk_pin);
        let hall = RpmSensor::new(peripherals.TIM2, hall_pin);
        let is_lathe = crate::settings::IS_LATHE.read(&peripherals.FLASH) != 0;
        let stepper = Stepper::new(DRIVER_TICK_FREQUENCY, driver, !is_lathe);
        let mut display = Display::new(screen);
        let controls = Controls::new(left_btn, right_btn, fast_btn, encoder_btn);

        // Pull-up e-stop (it's `1` when not active).
        let estop = EStop::new(gpiob.pb0.into_pull_up_input(&mut gpiob.crl).erase());

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

        let is_lathe = crate::settings::IS_LATHE.read(r.flash) != 0;
        if is_lathe {
            let mut menu = LatheMenu::new();
            loop {
                menu.run(&mut r);
            }
        } else {
            let mut menu = MillMenu::new();
            loop {
                menu.run(&mut r);
            }
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
    // Immediately disable driver outputs, just in case
    let mut gpioa = unsafe { Peripherals::steal().GPIOA }.split();
    gpioa.pa10.into_push_pull_output(&mut gpioa.crh).set_low();

    // Steal GPIOB and create another screen in an attempt to print some info
    let mut gpiob = unsafe { Peripherals::steal().GPIOB }.split();
    let rs_pin = gpiob.pb1.into_push_pull_output(&mut gpiob.crl).erase();
    let rw_pin = gpiob.pb10.into_push_pull_output(&mut gpiob.crh).erase();
    let e_pin = gpiob.pb11.into_push_pull_output(&mut gpiob.crh).erase();
    let db4 = gpiob.pb12.into_push_pull_output(&mut gpiob.crh).erase();
    let db5 = gpiob.pb13.into_push_pull_output(&mut gpiob.crh).erase();
    let db6 = gpiob.pb14.into_push_pull_output(&mut gpiob.crh).erase();
    let db7 = gpiob.pb15.into_push_pull_output(&mut gpiob.crh).erase();

    let screen = Screen::new(rs_pin, rw_pin, e_pin, [db4, db5, db6, db7]);
    let mut display = Display::new(screen);

    // Print reason on the display
    use core::fmt::Write;
    init_display(&mut display);
    display.position(0, 0);
    write!(display, "{}", info).unwrap();
    display.position(0, 1);
    if let Some(loc) = info.location() {
        let file = loc.file();
        let file = match file.rfind('/') {
            Some(pos) => &file[pos + 1..],
            None => file,
        };
        write!(
            display,
            "{}:{} {}            ",
            loc.line(),
            loc.column(),
            file
        )
        .unwrap();
    }

    loop {
        cortex_m::asm::wfi();
    }
}
