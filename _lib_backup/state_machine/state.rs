
pub struct PicoState {
    pub enter_button: bool,
    pub cursor_move_button: bool,
    pub dio1: bool,
    pub dio2: bool,
    pub dio3: bool
}

impl PicoState {
    pub fn new() -> Self {
        PicoState {
            enter_button: false,
            cursor_move_button: false,
            dio1: false,
            dio2: false,
            dio3: false
        }
    }
}