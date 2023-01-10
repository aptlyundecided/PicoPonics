//! # I²C Example
//!
//! This application demonstrates how to talk to I²C devices with an RP2040.
//!
//! It may need to be adapted to your particular board layout and/or pin assignment.
//!
//! See the `Cargo.toml` file for Copyright and license details.

#![no_std]
#![no_main]

// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
extern crate panic_halt;
extern crate embedded_hal;
extern crate rp2040_hal;
extern crate fugit;
extern crate ssd1306;

// Some traits we need
// use embedded_hal::blocking::i2c::Write;
use fugit::RateExtU32;

// Alias for our HAL crate
use rp2040_hal as hal;

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use hal::pac;
use core::fmt::Write;
// use alloc::string::ToString;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

/// The linker will place this boot block at the start of our program image. We
/// need this to help the ROM bootloader get our code up and running.
/// Note: This boot block is not necessary when using a rp-hal based BSP
/// as the BSPs already perform this step.
#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

/// External high-speed crystal on the Raspberry Pi Pico board is 12 MHz. Adjust
/// if your board has a different frequency
const XTAL_FREQ_HZ: u32 = 12_000_000u32;

/// Entry point to our bare-metal application.
///
/// The `#[rp2040_hal::entry]` macro ensures the Cortex-M start-up code calls this function
/// as soon as all global variables and the spinlock are initialised.
///
/// The function configures the RP2040 peripherals, then performs a single I²C
/// write to a fixed address.
#[rp2040_hal::entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    let clocks = hal::clocks::init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
        .ok()
        .unwrap();

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins to their default state
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Configure two pins as being I²C, not GPIO
    let scl_pin = pins.gpio3.into_mode::<hal::gpio::FunctionI2C>();
    let sda_pin = pins.gpio2.into_mode::<hal::gpio::FunctionI2C>();
    // let not_an_scl_pin = pins.gpio20.into_mode::<hal::gpio::FunctionI2C>();

    // Create the I²C drive, using the two pre-configured pins. This will fail
    // at compile time if the pins are in the wrong mode, or if this I²C
    // peripheral isn't available on these pins!
    let mut i2c = hal::I2C::i2c1(
        pac.I2C1,
        sda_pin,
        scl_pin, // Try `not_an_scl_pin` here
        400.kHz(),
        &mut pac.RESETS,
        &clocks.system_clock,
    );

    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(
        interface,
        DisplaySize128x64,
        DisplayRotation::Rotate0,
    ).into_terminal_mode();
    display.init().unwrap();
    display.clear().unwrap();

    // display.init().unwrap();
    // display.clear().unwrap();

    // load an array of u8's with the message 'Hello, Hannah!'

    let message = [
        'H' as u8,
        'e' as u8,
        'l' as u8,
        'l' as u8,
        'o' as u8,
        ',' as u8,
        ' ' as u8,
        'H' as u8,
        'a' as u8,
        'n' as u8,
        'n' as u8,
        'a' as u8,
        'h' as u8,
        '!' as u8,
    ];

    // let message = "Hello, Hannah!".to_string().into_bytes();

// Spam some characters to the display
//     for c in 97..123 {
//         let _ = display.write_str(unsafe { core::str::from_utf8_unchecked(&[c]) });
//     }
//     for c in 65..91 {
//         let _ = display.write_str(unsafe { core::str::from_utf8_unchecked(&[c]) });
//     }
    for c in message {
        let _ = display.write_str(unsafe { core::str::from_utf8_unchecked(&[c]) });
    }


// The `write!()` macro is also supported
//     write!(display, "Hello, {}", "world");

    loop {
        // cortex_m::asm::wfi();
    }
}

// End of file
