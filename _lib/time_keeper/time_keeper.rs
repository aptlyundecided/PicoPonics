pub struct TimeKeeper {
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8
}

impl TimeKeeper {
    #[inline]
    pub fn new() -> Self {
        TimeKeeper {
            hours: 0,
            minutes: 0,
            seconds: 0
        }
    }

    pub fn tick(&mut self) {
        if self.seconds < 59 {
            self.seconds += 1;
        } else {
            self.seconds = 0;
            if self.minutes < 59 {
                self.minutes += 1;
            } else {
                self.minutes = 0;
                if self.hours < 23 {
                    self.hours += 1;
                } else {
                    self.hours = 0;
                }
            }
        }
    }

    pub fn increment_hours(&mut self) {
        if self.hours < 23 {
            self.hours += 1;
        } else {
            self.hours = 0;
        }
    }
}