// Some traits we need
use embedded_hal::digital::v2::OutputPin;
use rp2040_hal::gpio::{Pin, Pins, PinId, PushPullOutput, DynPinId, DynPin};
use rp2040_hal::gpio::bank0::Gpio18;

pub struct DigitalOutputWrapper {
    pin: DynPin
}

impl DigitalOutputWrapper {
    pub fn new(pin: DynPin) -> Self {
        DigitalOutputWrapper {
            pin
        }
    }

    pub fn on(&mut self) {
        self.pin.set_high().unwrap();
    }

    pub fn off(&mut self) {
        self.pin.set_low().unwrap();
    }
}

pub struct DigitalOutputs {
    pub _18: Pin<Gpio18, PushPullOutput>
}

impl DigitalOutputs {
    #[inline]
    pub fn new(pins: &Pins) -> DigitalOutputs {
        DigitalOutputs {
            _18: pins.gpio18.into_push_pull_output()
        };
    }

    pub fn pin_on(&mut self, pin_id: u8) {
        self.change_pin_state(&pin_id, true);
    }

    pub fn pin_off(&mut self, pin_id: u8) {
        // self._18.set_low().unwrap();
        self.change_pin_state(&pin_id, false);

    }

    fn change_pin_state(&mut self, pin_id: &u8, state: bool) {
        match pin_id {
            &18 => {
                if state {
                    self._18.set_high().unwrap();
                } else {
                    self._18.set_low().unwrap();
                }
            },
            _ => {}
        }
    }

}