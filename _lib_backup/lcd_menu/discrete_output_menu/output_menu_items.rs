fn output_option_list(opt: u8) -> [char; 12]{
    match opt {
        0 => [' ', 'S', 'e', 'l', 'e', 'c', 't', '^', '^', '^', '^', '\n'],
        1 => [' ', 'N', 'e', 'x', 't', '^', '^', '^', '^', '^', '^', '\n'],
        2 => [' ', 'P', 'r', 'e', 'v', '^', '^', '^', '^', '^', '^', '\n'],
        _ => ['^', '^', '^', '^', '^', '^', '^', '^', '^', '^', '^', '\n'],
    }
}

fn output_selection_list(sel: u8) -> [char; 2] {
    match sel {
        0 => ['1', '0'],
        1 => ['1', '1'],
        2 => ['1', '2'],
        3 => ['1', '3'],
        4 => ['1', '4'],
        5 => ['1', '5'],
        6 => ['1', '6'],
        7 => ['1', '7'],
        8 => ['1', '8'],
        9 => ['1', '9'],
        10 => ['2', '0'],
        11 => ['2', '1'],
        12 => ['2', '2'],
        _ => ['^', '^'],
    }
}

pub struct OutputMenuOptions {
    pub option_cursor_index: u8
}

impl OutputMenuOptions {
    pub fn new() -> OutputMenuOptions {
        OutputMenuOptions {
            option_cursor_index: 0
        }
    }
    pub fn get_option_val(&self, opt: u8) -> [char; 12] {
        output_option_list(opt)
    }
    pub fn increment_option_cursor(&mut self) {
        if self.option_cursor_index < 2 {
            self.option_cursor_index += 1;
        }
    }
}

pub struct OutputMenuSelections {
    pub active_selection_index: u8
}

impl OutputMenuSelections {
    pub fn new() -> OutputMenuSelections {
        OutputMenuSelections {
            active_selection_index: 0
        }
    }
    pub fn get_selection_val(&self, opt: u8) -> [char; 2] {
        output_selection_list(opt)
    }
}

pub struct OutputMenu {
    pub options: OutputMenuOptions,
    pub selections: OutputMenuSelections,
    pub mode: u8
}

impl OutputMenu {
    pub fn new() -> OutputMenu {
        OutputMenu {
            options: OutputMenuOptions::new(),
            selections: OutputMenuSelections::new(),
            mode: 0
        }
    }
    pub fn handle_selection(&mut self) {
        match self.mode {
            0 => {
                match self.options.option_cursor_index {
                    0 => self.mode = 1000,
                    1 => self.mode = 1001,
                    2 => self.mode = 1002,
                    _ => self.mode = 0
                }
            }
            _ => {}
        }
    }
}