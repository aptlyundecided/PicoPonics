use lcd_menu::discrete_output_menu::output_menu_items::OutputMenu;

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





/// <><><><><><><><><><><>
/// Refactored Zone:
///
/// Menu mode selection occurs as u8.  u8 maps to a specific menu mode.
/// The mode determines what functionality occurs depending on button
/// press type or interaction type.
///
///

pub struct Menu {
    pub menu_control: MenuControl,
    pub output_menu: OutputMenu,
    pub menu_mode: u32,
    pub menu_option_val: u8,
    pub menu_selection_val: u8,
    pub menu_selection_detected: bool,
    pub interface_code: u16
}

impl Menu {
    pub fn new() -> Menu {
        Menu {
            menu_control: MenuControl::new(),
            output_menu: OutputMenu::new(),
            menu_mode: 0,
            menu_option_val: 0,
            menu_selection_val: 0,
            menu_selection_detected: false,
            interface_code: 0
        }
    }
    pub fn set_active_mode(&mut self, mode: &str) {
        match mode {
            "scheduling" => self.menu_control.output_schedule.scheduling_mode_active = true,
            "navigate" => self.menu_control.output_schedule.scheduling_mode_active = false,
            _ => self.menu_control.output_schedule.scheduling_mode_active = false
        }
    }
    pub fn handle_cursor_move(&mut self) {
        match self.menu_mode {
            0 => self.output_menu.options.increment_option_cursor(),
            _ => ()
        }
    }
    pub fn get_cursor_text(&mut self) -> [char; 12] {
        let mut x = self.output_menu.options.get_option_val(self.output_menu.options.option_cursor_index);
        x[0] = '>';
        return x;
    }
    pub fn mark_selection(&mut self) {
        self.menu_selection_detected = true;
    }
    pub fn handle_selection(&mut self) {
        match self.menu_mode {
            1000 => {
                match self.output_menu.mode {
                    _ => {}
                }
            }
            _ => {}
        }
    }
}



pub struct SubMenuOptions {
    pub option_cursor_index: u8,
    pub option_list: [u8; 12],
    pub option_list_text: [[char; 12]; 12],
    pub selection_mode: u16
}

impl SubMenuOptions {
    pub fn new() -> SubMenuOptions {
        Sub
    }
}

/// <><><>
/// More Refactor
pub struct SubMenu {
    pub mode: u16,
    pub options: SubMenuOptions
}