use lcd_menu::menu_node::MenuNode;

/// Build Output Menu Node
/// ---
/// create node information to give menu navigation abilities
/// plus data structure reference capabilities.
pub fn build_output_menu_node() -> MenuNode {
    let mut output_menu_node: MenuNode = MenuNode::new();
    output_menu_node.node_id = 10000;
    output_menu_node.select_ref = 11000;
    output_menu_node.deselect_ref = 0;
    output_menu_node.next_ref = 20000;
    output_menu_node.prev_ref = 0;
    output_menu_node.option_range = 3;
    output_menu_node.selection_range = 12;
    return output_menu_node;
}