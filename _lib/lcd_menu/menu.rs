pub struct OutputSchedule {
    pub pins: [u8; 13],
    pub scheduling_mode_active: bool
}

impl OutputSchedule {
    pub fn new() -> OutputSchedule {
        OutputSchedule {
            pins: [10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22],
            scheduling_mode_active: false
        }
    }
}

pub struct MenuControl {
    pub output_schedule: OutputSchedule
}

impl MenuControl {
    pub fn new() -> MenuControl {
        MenuControl {
            output_schedule: OutputSchedule::new()
        }
    }
}

pub struct Menu {
    pub menu_control: MenuControl
}

impl Menu {
    pub fn new() -> Menu {
        Menu {
            menu_control: MenuControl::new()
        }
    }
    pub fn set_active_mode(&mut self, mode: &str) {
        match mode {
            "scheduling" => self.menu_control.output_schedule.scheduling_mode_active = true,
            "navigate" => self.menu_control.output_schedule.scheduling_mode_active = false,
            _ => self.menu_control.output_schedule.scheduling_mode_active = false
        }
    }
}
