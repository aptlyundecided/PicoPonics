use lcd_menu::menu_node::MenuNode;
use lcd_menu::menu_nodes::output_menu::build_output_menu_node;

pub struct MenuNodeNavigator {
    pub current_node: MenuNode
}

impl MenuNodeNavigator {
    pub fn new() -> MenuNodeNavigator {
        MenuNodeNavigator {
            current_node: MenuNode::new()
        }
    }
    pub fn load_node_selection(&mut self, node_id: u16) {
        match node_id {
            10000 => {
                self.current_node = build_output_menu_node();
            }
            _=> {}
        }
    }
}