pub struct GPConfig {
    pub start_hours: u8,
    pub start_minutes: u8,
    pub start_seconds: u8,
    pub end_hours: u8,
    pub end_minutes: u8,
    pub end_seconds: u8
}

impl GPConfig {
    #[inline]
    pub fn new() -> Self {
        GPConfig {
            start_hours: 0,
            start_minutes: 0,
            start_seconds: 0,
            end_hours: 0,
            end_minutes: 0,
            end_seconds: 0
        }
    }

    pub fn set_start_hours(&mut self, hours: u8) {
        self.start_hours = hours;
    }

    pub fn set_start_minutes(&mut self, minutes: u8) {
        self.start_minutes = minutes;
    }

    pub fn set_start_seconds(&mut self, seconds: u8) {
        self.start_seconds = seconds;
    }

    pub fn set_end_hours(&mut self, hours: u8) {
        self.end_hours = hours;
    }

    pub fn set_end_minutes(&mut self, minutes: u8) {
        self.end_minutes = minutes;
    }

    pub fn set_end_seconds(&mut self, seconds: u8) {
        self.end_seconds = seconds;
    }
}