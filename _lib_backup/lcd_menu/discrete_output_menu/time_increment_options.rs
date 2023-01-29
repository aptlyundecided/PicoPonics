/// Time Increment Option List
/// ---
/// provide a set of options to show the user that let them
/// know what sort of adjustment to the schedule they are
/// about to perform.
pub fn time_increment_option_list(opt: u8) -> [char; 12] {
    match opt {
        0 => [' ', '+', '1', '5', 'M', 'i', 'n', 's', ' ', ' ', ' ', '\n'],
        1 => [' ', '-', '1', '5', 'M', 'i', 'n', 's', ' ', ' ', ' ', '\n'],
        2 => [' ', '+', '1', 'H', 'o', 'u', 'r', ' ', ' ', ' ', ' ', '\n'],
        3 => [' ', '-', '1', 'H', 'o', 'u', 'r', ' ', ' ', ' ', ' ', '\n'],
        _ => ['^', '^', '^', '^', '^', '^', '^', '^', '^', '^', '^', '^']
    }
}

/// Time Increment Options
/// ---
/// control struct used to show which increment option
/// the user has 'cursored'.
pub struct TimeIncrementOptions {
    pub option_cursor_index: u8
}


impl TimeIncrementOptions {
    pub fn new() -> TimeIncrementOptions {
        TimeIncrementOptions {
            option_cursor_index: 0
        }
    }
    pub fn increment_option_cursor_index(&mut self) {
        if self.option_cursor_index < 3 {
            self.option_cursor_index += 1;
        } else {
            self.option_cursor_index = 0;
        }
    }
    pub fn get_option_val(&mut self, i: u8) -> [char; 12] {
        time_increment_option_list(i)
    }
}