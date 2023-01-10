#![no_std]
#![no_main]

/// Import external crates
extern crate panic_halt;
extern crate embedded_hal;
extern crate rp2040_hal;
extern crate ssd1306;
extern crate fugit;

/// Import my PicoPonics lib
mod _lib;

/// configure PicoPonics lib utilization
use _lib::time_keeper::time_keeper::TimeKeeper;
use _lib::time_keeper::u8_time_mapping::get_time_chars;
use _lib::gp_config::gp_config::GPConfig;

// Alias for our HAL crate
use rp2040_hal as hal;

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use hal::pac;

// Some traits we need
// use embedded_hal::digital::v2::InputPin;
use embedded_hal::digital::v2::{InputPin, OutputPin};
use rp2040_hal::clocks::Clock;

use fugit::RateExtU32;
use core::fmt::Write;
use core::char;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};


/// The linker will place this boot block at the start of our program image. We
/// need this to help the ROM bootloader get our code up and running.
/// Note: This boot block is not necessary when using a rp-hal based BSP
/// as the BSPs already perform this step.x
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
/// The function configures the RP2040 peripherals, then toggles a GPIO pin in
/// an infinite loop. If there is an LED connected to that pin, it will blink.
#[rp2040_hal::entry]
fn main() -> ! {
    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

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

    // configure a ms delay timer
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

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
    let mut gp18 = pins.gpio18.into_push_pull_output();
    let gp15 = pins.gpio15.into_pull_down_input();

    // Create the I²C drive, using the two pre-configured pins. This will fail
    // at compile time if the pins are in the wrong mode, or if this I²C
    // peripheral isn't available on these pins!
    let mut i2c = hal::I2C::i2c1(
        pac.I2C1,
        sda_pin,
        scl_pin,
        400_u32.kHz(),
        &mut pac.RESETS,
        &clocks.system_clock,
    );

    // Configure GPIO outputs
    // let mut output_pins = DigitalOutputs::new(pins);

    let mut gp18_config = GPConfig::new();

    // TODO:AW - timer should be live-configurable
    // 6 hours run time for the lights
    gp18_config.set_start_hours(7);
    gp18_config.set_start_minutes(0);
    gp18_config.set_start_seconds(0);
    gp18_config.set_end_hours(19);
    gp18_config.set_end_minutes(0);
    gp18_config.set_end_seconds(0);

    let mut time_keeper = TimeKeeper::new();


    // Prep LCD Screen interfacing
    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(
        interface,
        DisplaySize128x64,
        DisplayRotation::Rotate0,
    ).into_terminal_mode();

    // Init and clear the screen
    display.init().unwrap();
    display.clear().unwrap();

    let mut message_chars: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
    let hours = get_time_chars(0);
    let minutes = get_time_chars(0);
    let seconds = get_time_chars(0);
    message_chars[0] = hours.0 as u8;
    message_chars[1] = hours.1 as u8;
    message_chars[2] = ':' as u8;
    message_chars[3] = minutes.0 as u8;
    message_chars[4] = minutes.1 as u8;
    message_chars[5] = ':' as u8;
    message_chars[6] = seconds.0 as u8;
    message_chars[7] = seconds.1 as u8;


    for c in message_chars {
        let _ = display.write_str(unsafe { core::str::from_utf8_unchecked(&[c]) });
    }

    loop {
        // create one second delay
        delay.delay_ms(1000);

        // Timekeeper updates because it's role is to keep track of a
        // locally created system time.
        time_keeper.tick();

        // Check if the time adjust is pressed
        if gp15.is_high().unwrap() {
            time_keeper.increment_hours();
        }

        let hours = get_time_chars(time_keeper.hours);
        let minutes = get_time_chars(time_keeper.minutes);
        let seconds = get_time_chars(time_keeper.seconds);
        let time_message = [
            hours.0 as u8,
            hours.1 as u8,
            ':' as u8,
            minutes.0 as u8,
            minutes.1 as u8,
            ':' as u8,
            seconds.0 as u8,
            seconds.1 as u8,
        ];


        // Init and clear the screen
        // display.init().unwrap();
        display.clear().unwrap();

        for c in time_message {
            let _ = display.write_str(unsafe { core::str::from_utf8_unchecked(&[c]) });
        }

        let between5And10 = time_keeper.seconds > 10 && time_keeper.seconds < 15;

        let tk_hours = time_keeper.hours;
        let light_on = tk_hours >= 7 && tk_hours < 13;

        if light_on {
            gp18.set_high().unwrap();
            // output_pins._18.set_high().unwrap();
            // output_pins.pin_on(18);
            // out_pin.set_high().unwrap();
        } else {
            gp18.set_low().unwrap();
            // output_pins.pin_off(18);
            // output_pins._18.set_low().unwrap();
            // out_pin.set_low().unwrap();
        }
    }
}
