#![no_std]
#![no_main]

/// Import external crates
extern crate panic_halt;
extern crate embedded_hal;
extern crate rp2040_hal;
extern crate ssd1306;
extern crate fugit;

/// Import my PicoPonics lib
// mod _lib;
// mod _lib;
// mod pico_ponics_lib;
// extern crate tlib;
extern crate _lib;

/// configure PicoPonics lib utilization
use _lib::time_keeper::time_keeper::TimeKeeper;
use _lib::time_keeper::u8_time_mapping::get_time_chars;
use _lib::gp_config::gp_config::GPConfig;
use _lib::state_machine::state::{PicoState};
// use _lib::lcd_menu::menu::Menu;
use _lib::lcd_menu::menu_nodes::menu_node_navigator::MenuNodeNavigator;

// Alias for our HAL crate
use rp2040_hal as hal;

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use hal::pac;
use hal::gpio::Pins;

// Some traits we need
// use embedded_hal::digital::v2::InputPin;
use embedded_hal::digital::v2::{InputPin, OutputPin};
use rp2040_hal::clocks::Clock;

use fugit::RateExtU32;
use core::fmt::Write;
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
const XTAL_FREQ_HZ: u32 = 12_500_000u32;


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

    // Create state machine for representing the state of the PicoPonics system
    let mut pico_state: PicoState = PicoState::new();

    // Create menu reference variable for user interface handling
    // let mut menu: Menu = Menu::new();
    let mut menu_nav = MenuNodeNavigator::new();
    menu_nav.load_node_selection(10000);
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
    let pins: Pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Configure two pins as being I²C, not GPIO
    let scl_pin = pins.gpio3.into_mode::<hal::gpio::FunctionI2C>();
    let sda_pin = pins.gpio2.into_mode::<hal::gpio::FunctionI2C>();

    /// Configure Inputs [gp6 - gp9]
    /// ---
    /// Only four Dinputs because I want more availability for comms channels.  I want
    /// inputs to come from external devices that I'm tinkering with later on.
    let gp6 = pins.gpio6.into_pull_down_input();
    let gp7 = pins.gpio7.into_pull_down_input();
    let gp8 = pins.gpio8.into_pull_down_input();
    let gp9 = pins.gpio9.into_pull_down_input();

    // TODO:AW change to output pin
    let gp15 = pins.gpio15.into_pull_down_input();

    /// Configure Outputs [gp10 - gp21]
    /// ---
    /// more digital outputs than inputs because first version will be more of an
    /// advanced timer.  Advanced inputs will come from i2C comms from another Pico.
    let mut gp18 = pins.gpio18.into_push_pull_output();

    // Create the I²C drive, using the two pre-configured pins. This will fail
    // at compile time if the pins are in the wrong mode, or if this I²C
    // peripheral isn't available on these pins!
    let i2c = hal::I2C::i2c1(
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

    // 6 hours run time for the lights
    gp18_config.set_start_time(7, 0, 0);
    gp18_config.set_end_time(14, 0, 0);

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

    loop {
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
        // display.clear().unwrap();

        // for c in time_message {
        //     let _ = display.write_str(unsafe { core::str::from_utf8_unchecked(&[c]) });
        // }

        // map Input Status to PicoState
        pico_state.enter_button = gp6.is_high().unwrap();
        pico_state.cursor_move_button = gp7.is_high().unwrap();

        // map comms data onto pico state
        // TODO|AW: !

        // map output status onto PicoState


        // Mark menu for eval on next loop
        if pico_state.enter_button {
            // TODO|AW: do enter button stuff
            // menu.mark_selection();
            menu_nav.make_selection();
            pico_state.queue_lcd_update();
        } else if pico_state.cursor_move_button {
            // TODO|AW: do cursor move button stuff
            // menu.handle_cursor_move();
            menu_nav.cursor_next();
            pico_state.queue_lcd_update();
        }


        // TODO|AW: create an LCD Screen abstraction
        if pico_state.lcd_update_q {
            // CLEAR!
            display.clear().unwrap();

            // LCD MENU CONTROL --> Menu Title / Name
            match menu_nav.current_node.node_id {
                10000 => {
                    let _ = display.write_str("Select Pin>>\n\n");
                }
                11000 => {
                    let _ = display.write_str("Pin-Config>>\n\n");
                }
                11100 => {
                    let _ = display.write_str("Scheduling>>\n\n");
                }
                11110 => {
                    let _ = display.write_str("Start-Time>>\n\n");
                }
                11120 => {
                    let _ = display.write_str("End-Time>>\n\n");
                }
                _ => {
                    let _ = display.write_str("Farts!>>\n\n");
                }
            }

            // LCD MENU CONTROL --> Current Selection
            let selection = menu_nav.get_current_selection();
            for ch in selection {
                if ch == '^' { break; }

                let c = ch as u8;
                let _ = display.write_str(
                    unsafe { core::str::from_utf8_unchecked(&[c]) }
                );
            }

            // LCD MENU CONTROL --> Options
            for option in menu_nav.get_options_list() {
                for ch in option {
                    if ch == '^' { break; }

                    let c = ch as u8;
                    let _ = display.write_str(
                        unsafe { core::str::from_utf8_unchecked(&[c]) }
                    );
                }
            }

            delay.delay_ms(100);
        }



        // TODO|AW: Refactor everything below here.
        // create one second delay
        // delay.delay_ms(1000);

        // Timekeeper updates because it's role is to keep track of a
        // locally created system time.
        // time_keeper.tick();

        // Check if the time adjust is pressed
        if gp15.is_high().unwrap() {
            time_keeper.increment_hours();
        }

        let tk_hours = time_keeper.hours;

        // GP18 Output handling
        let gp18_after_start_time = tk_hours >= gp18_config.start_hours;
        let gp18_before_end_time: bool = tk_hours < gp18_config.end_hours;
        let gp18_on: bool = gp18_after_start_time && gp18_before_end_time;

        if gp18_on {
            gp18.set_high().unwrap();
        } else {
            gp18.set_low().unwrap();
        }


        // Dequeue LCD Update
        pico_state.lcd_update_complete();
    }
}
