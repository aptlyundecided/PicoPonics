pub struct MenuNode {
    pub node_id: u16,
    pub next_ref: u16,
    pub prev_ref: u16,
    pub select_ref: u16,
    pub deselect_ref: u16,
    pub option_index: u8,
    pub selection_index: u8,
    pub selection_range: u8,
    pub option_range: u8
}

impl MenuNode {
    pub fn new() -> MenuNode {
        MenuNode {
            node_id: 0,
            next_ref: 0,
            prev_ref: 0,
            select_ref: 0,
            deselect_ref: 0,
            option_index: 0,
            selection_index: 0,
            option_range: 0,
            selection_range: 0
        }
    }
}