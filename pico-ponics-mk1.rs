#![no_std]
#![no_main]

/// Import external crates
extern crate panic_halt;
extern crate embedded_hal;
extern crate rp2040_hal;
extern crate ssd1306;
extern crate fugit;

/// Import the PicoPonics lib
extern crate _lib;

/// configure PicoPonics lib utilization
use _lib::time_keeper::time_keeper::TimeKeeper;
use _lib::time_keeper::u8_time_mapping::get_time_chars;
use _lib::gp_config::gp_config::GPConfig;
use _lib::state_machine::state::{PicoState};
use _lib::lcd_menu::menu_nodes::menu_node_navigator::MenuNodeNavigator;

// Alias for the HAL crate
use rp2040_hal as hal;

// prep for RP2040 pac and pins accessing
use hal::pac;
use hal::gpio::Pins;

// Some traits we need use embedded_hal::digital::v2::InputPin;
use embedded_hal::digital::v2::{InputPin, OutputPin};
use rp2040_hal::clocks::Clock;

// eh?
use fugit::RateExtU32;
use core::fmt::Write;

// More PicoPonics specific imports
use _lib::state_machine::interface_utils::lcd_menu_interpreter::create_schedule_update::create_schedule_update;
use _lib::state_machine::state_update_actions::pin_schedule_updates::PinScheduleUpdate;
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

    // Create state machine for representing the state of the PicoPonics system
    let mut pico_state: PicoState = PicoState::new();

    // Create menu reference variable for user interface handling
    // let mut menu: Menu = Menu::new();
    let mut menu_nav = MenuNodeNavigator::new();
    menu_nav.load_node_selection(0);

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
    /// Only four inputs because I want more availability for comms channels.  I want
    /// inputs to come from external devices that I'm tinkering with later on.
    let gp6 = pins.gpio6.into_pull_down_input();
    let gp7 = pins.gpio7.into_pull_down_input();
    let gp8 = pins.gpio8.into_pull_down_input();
    let gp9 = pins.gpio9.into_pull_down_input();

    // TODO:AW change to output pin

    /// Configure Outputs [gp10 - gp21]
    /// ---
    /// more digital outputs than inputs because first version will be more of an
    /// advanced timer.  Advanced inputs will come from i2C comms from another Pico.
    let mut gp10 = pins.gpio10.into_push_pull_output();
    let mut gp11 = pins.gpio11.into_push_pull_output();
    // let mut gp15 = pins.gpio15.into_push_pull_output();

    let mut gp18 = pins.gpio18.into_push_pull_output();
    let mut gp25 = pins.gpio25.into_push_pull_output();
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


    // --- TESTING --- //
    pico_state.output_pins.pin10.schedule.start_time.hours = 0;
    pico_state.output_pins.pin10.schedule.start_time.minutes = 0;
    pico_state.output_pins.pin10.schedule.start_time.seconds = 0;
    pico_state.output_pins.pin10.schedule.end_time.hours = 0;
    pico_state.output_pins.pin10.schedule.end_time.minutes = 0;
    pico_state.output_pins.pin10.schedule.end_time.seconds = 0;

    pico_state.output_pins.pin18.schedule.start_time.hours = 7;
    pico_state.output_pins.pin18.schedule.start_time.minutes = 0;
    pico_state.output_pins.pin18.schedule.start_time.seconds = 0;
    pico_state.output_pins.pin18.schedule.end_time.hours = 14;
    pico_state.output_pins.pin18.schedule.end_time.minutes = 0;
    pico_state.output_pins.pin18.schedule.end_time.seconds = 0;


    /*
        BEGIN PROGRAM LOOP / SCAN CYCLE
        ---
        ^V^V^V^V^V^V^V^V^V^V^V^V^V^V^V^V^V^V^V
     */
    loop {


        // TODO|AW: This is a hack to get the time keeper working
        // Rudimentary Time Keeping
        time_keeper.loop_handler();

        pico_state.output_pins.pins_schedule_compare(
            time_keeper.produce_time_in_seconds()
        );

        // LOOP SYNC TOOLING -- flash onboard LED on ~1hz
        // This is so I can ensure that the board 'tick'
        // rate is ~1s.  Time sync proper is coming in
        // future iterations.
        if time_keeper.hz {
            gp25.set_high().unwrap();
        } else {
            gp25.set_low().unwrap();
        }


        // map Input Status to PicoState
        pico_state.enter_button = gp6.is_high().unwrap();
        pico_state.cursor_move_button = gp7.is_high().unwrap();
        pico_state.hour_index_button = gp8.is_high().unwrap();

        // map comms data onto pico state
        // TODO|AW: !


        // Mark menu for eval on next loop
        if pico_state.enter_button {
            menu_nav.make_selection();
            pico_state.queue_lcd_update();
        } else if pico_state.cursor_move_button {
            menu_nav.cursor_next();
            pico_state.queue_lcd_update();
        } else if pico_state.hour_index_button {
            time_keeper.increment_hours();
            pico_state.queue_lcd_update();
        }

        // TODO|AW: create an LCD Screen abstraction module for handling this
        if pico_state.lcd_update_q {
            // CLEAR!
            display.clear().unwrap();

            // LCD MENU CONTROL --> Menu Title / Name
            match menu_nav.current_node.node_id {
                0 => {
                    let _ = display.write_str("Main Menu>>\n\n");
                }
                1000 => {
                    let _ = display.write_str("Sys Time\n\n");
                    let time: [char; 8] = time_keeper.produce_time_char_array();

                    for ch in time {
                        let c = ch as u8;
                        let _ = display.write_str(
                            unsafe { core::str::from_utf8_unchecked(&[c]) }
                        );
                    }

                    let _ = display.write_str("\n");

                }
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
                11130 => {
                    let _ = display.write_str("Schedule>>\n\n");

                    let mut time: [[char; 8]; 2] = [
                        ['0', '0', ':', '0', '0', ':', '0', '0'],
                        ['0', '0', ':', '0', '0', ':', '0', '0']
                    ];
                    // TODO|AW: CLEAN UP
                    // TODO|AW: The LCD abstraction becomes more necessary here as the code gets more complex
                    match menu_nav.meta_data.selected_pin {
                        Some(pin) => {
                            match pin {
                                10 => {
                                    let _ = display.write_str("<Pin 10>\n\n");
                                    time = pico_state.output_pins.pin10.get_output_schedule_as_chars();
                                }
                                11 => {
                                    let _ = display.write_str("<Pin 11>\n\n");
                                    time = pico_state.output_pins.pin11.get_output_schedule_as_chars();
                                }
                                12 => {
                                    let _ = display.write_str("<Pin 12>\n\n");
                                    time = pico_state.output_pins.pin12.get_output_schedule_as_chars();
                                }
                                13 => {
                                    let _ = display.write_str("<Pin 13>\n\n");
                                    time = pico_state.output_pins.pin13.get_output_schedule_as_chars();

                                }
                                18 => {
                                    let _ = display.write_str("<Pin 18>\n\n");
                                    time = pico_state.output_pins.pin13.get_output_schedule_as_chars();
                                }
                                _ => {
                                    let _ = display.write_str("Pin ??\n\n");
                                    time = [['_'; 8]; 2];
                                }
                            }
                        }
                        _ => {
                            let _ = display.write_str("__:__:__\n\n");
                        }
                    }

                    // TODO|AW: ADD this to the LCD Abstraction
                    let __ = display.write_str("s:");
                    for ch in time[0 as usize] {
                        let _ = display.write_str(
                            unsafe { core::str::from_utf8_unchecked(&[ch as u8]) }
                        );
                    }
                    let nl0 = display.write_str("\n");
                    let ___ = display.write_str(" e:");
                    for ch in time[1 as usize] {
                        let _ = display.write_str(
                            unsafe { core::str::from_utf8_unchecked(&[ch as u8]) }
                        );
                    }
                    let nl = display.write_str("\n");

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

            // TODO|AW: do not put any delays in the loop.  This breaks time-keeping.
            // Prevent LCD Update Spam
            delay.delay_ms(100);

            // Dequeue LCD Update
            pico_state.lcd_update_complete();
        }

        // Move UI data from the LCD menu into the Application State
        if menu_nav.update_action_queued {

            if menu_nav.meta_data.is_system_time_adjustment {
                match menu_nav.meta_data.time_adjustment {
                    Some(time) => {
                        match time {
                            0 => {
                                time_keeper.increment_hours();
                            },
                            2 => {
                                time_keeper.increment_minutes();
                            },
                            _ => {}
                        }
                    }
                    _ => {}
                }
            } else {
                // TODO|AW: pico_state needs to be able to determine update type.
                // TODO|AW: how to determine which type of update to create?
                let update: PinScheduleUpdate = create_schedule_update(&menu_nav);
                pico_state.handle_pin_schedule_update(update);
                menu_nav.meta_data.time_adjustment = None;
            }


            // clean
            menu_nav.meta_data.is_system_time_adjustment = false;
        }

        // TODO|AW: Abstract this into a HAL Context Manager
        // Evaluate Pico State; handle cyclical updates
        if time_keeper.hz1 {

            // Output Pin 10
            if pico_state.output_pins.pin10.active {
                gp10.set_high().unwrap();
            } else {
                gp10.set_low().unwrap();
            }

            // Output Pin 11
            if pico_state.output_pins.pin11.active {
                gp11.set_high().unwrap();
            } else {
                gp11.set_low().unwrap();
            }

            // Output Pin 18
            if pico_state.output_pins.pin18.active {
                gp18.set_high().unwrap();
            } else {
                gp18.set_low().unwrap();
            }
        }
    }
}
