# PicoPonics

A configurable plant habitat automation system that is scalable, tiny,
and low power.  The goal is when the project matures that it will be configurable enough
that it can control anything from a single plant, all the way up to
a commercial operation via distributed control.


## Latest Version Description (POC)
A quick bullet point run-down of the intended function of the current system.

### Pins
- GP18 is switched on and off on a schedule (hard coded from 7AM to 1PM)
- GP15 is set up to be an input for adjusting the time
- GP2 is an SDA pin for I2C comms to a ssd1306 OLED display
- GP3 is an SCL pin for I2C comms to a ssd1306 OLED display

### Function Description
- use a tiny ssd1306 OLED to display system state
  - The display shows what the controller thinks the current time is
- Wire a 5VDC button to GP15 and hold it down to adjust the time (normally open button)
- The time is a 24-hour clock that loops around after 23:59:59


## Getting Started
1. Plug your pico in and ensure it's detected as a USB device by your machine.
2. cargo run --bin pico-ponics-mk1 
3. ??
4. ??
5. ??
6. ??
7. Wire up your pico-ponics controller to your field devices
8. Profit!


## Roadmap
- latest:: POC ((see above description))
- v0.1.0
- v0.2.0
- v0.3.0
- v0.4.0
- v0.5.0
- v0.6.0
- v0.7.0
- v1.0.0


### v0.1.0 (Self Contained Series Mk1)
- [ ] stl files for 3D printing enclosures
- [ ] wiring diagrams
- [ ] configurable schedule for output pins
- [ ] 2-button control for navigating menus
  - set current time
  - set schedule per pin
- [ ] generic setup utilizing all available pins for single-device control
- [ ] improve timekeeping accuracy (should not lose more than 1 second per hour)
- [ ] Design Mk1 PCB for holding buttons, screen, LED, and other components
- [ ] make PCB sketch files available in repo


### v0.2.0 (Deep Embed Series Mk1)
- [ ] configure pins for external comms to other picos
- [ ] pico 'remote control' can configure habitat controlling pico
- [ ] pico remove PCB sketch files available in repo
- [ ] pico remote stl files for 3D print available in repo


### v0.3.0 (Distributed Series Mk1)
- [ ] configure pico to accept control via I2C
- [ ] Wired short range control to multiple picos
- [ ] use lorawan to send control signals to pico
- [ ] use lorawan to receive control signals from controller pico

### v0.4.0 (Self Contained Series Mk2){{DC Discrete Series
- [ ] create modules for self-contained version to extend its capability
- [ ] DC discrete output module
- [ ] DC discrete input module

### v0.5.0 (Self Contained Series Mk3){{AC Discrete Series}}
- [ ] AC discrete control output module
- [ ] AC discrete control input module

### v0.6.0 (Self Contained Series Mk4){{DC Analog Series}}
- [ ] Analog input module
- [ ] Analog output module

### v0.7.0 (Join Self Contained Modules to Distributed)
- [ ] Pluggable Ethernet network module
- [ ] Pluggable WiFi network module
- [ ] Pluggable LoRaWAN network module

### v1.0.0 (Complete Version 1)
- [ ] configurable via remote contro pico, or via web interface
- [ ] built-in distributed control features
- [ ] comparable functionality to commercial systems
- [ ] still small and cheap enough to provide habitat for a single plant

## Contributing
No real protocol for this at the moment.  Will create it if needed.