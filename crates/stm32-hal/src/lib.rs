//! HAL layer for STM32 devices on top of STM32 device crates (`stm32f103`)
#![no_std]
#![warn(missing_docs)]
#![deny(warnings)]

/// Higher-level API for the Flash memory controller.
///
/// # Examples
///
/// Erasing flash memory page and writing some data to it:
///
/// ```rust,no_run
/// extern crate stm32f103;
/// extern crate stm32_hal;
/// use stm32f103::FLASH;
/// use stm32_hal::flash::Flash;
///
/// # pub fn main() {
/// # let flash = unsafe { &*FLASH.get() };
/// // Get flash somehow...
/// // let flash = FLASH.borrow(cs);
/// unsafe {
///     let flash = flash.unlock_guard().unwrap(); // Unlock Flash for writing
///     flash.erase_page(0x800_fc00).unwrap(); // last 1K page on a chip with 64K flash memory
///     flash.program_half_word(0x800_fc00, 0xcafe).unwrap();
///     flash.program_half_word(0x800_fc02, 0xbabe).unwrap();
/// }
/// # }
/// ```
///
pub mod flash;
