use lcd_menu::discrete_output_menu::standard_menu_options::standard_menu_option_list;

pub struct Scheduling {
    active_selection_index: u8,
    option_cursor_index: u8
}

impl Scheduling {
    pub fn new() -> Scheduling {
        Scheduling {
            active_selection_index: 0,
            option_cursor_index: 0
        }
    }
    pub fn get_option_val(&self, opt: u8) -> [char; 12] {
        standard_menu_option_list(opt)
    }

    pub fn get_active_selection_index(&self) -> u8 {
        self.active_selection_index
    }
}