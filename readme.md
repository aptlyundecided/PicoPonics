# PicoPonics

A configurable plant habitat automation system that is scalable, tiny,
and low power.

under construction -- please ignore my mess


## Current Version Description
A quick bullet point run-down of the intended function of the system.

### Pins
- GP18 is switched on and off on a schedule (hard coded from 7AM to 1PM)
- GP15 is set up to be an input for adjusting the time
- GP2 is an SDA pin for I2C comms to a ssd1306 OLED display
- GP3 is an SCL pin for I2C comms to a ssd1306 OLED display

### Functionality
- The display shows what the controller things the current time is
- Wire a 5VDC button to GP15 and hold it down to adjust the time (normally open button)
- The time is a 24-hour clock that loops around at 23:59


## README STILL UNDER CONSTRUCTION
- pushing this up now so that something exists

## Getting Started
1. plug your pico in and ensure it's detected as a USB device by your machine.
2. cargo run --bin pico-ponics-mk1
3 - 6: ???
7. wire up your pico-ponics controller to your field devices
8. Profit!


## Roadmap
- current
- v0.1.0
- v0.2.0
- ?


### Current Version
Proof of concept only.  I currently use this to control a 100W LED grow lamp
for my tomato plants.  