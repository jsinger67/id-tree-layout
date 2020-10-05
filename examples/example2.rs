use id_tree::InsertBehavior::{AsRoot, UnderNode};
use id_tree::{Node, NodeId, Tree, TreeBuilder};
use id_tree_layout::{Layouter, SvgDrawer, Visualize};

struct MyNodeData {
    name: &'static str,
    is_terminal: bool,
}

impl MyNodeData {
    fn terminal(name: &'static str) -> Self {
        Self {
            name,
            is_terminal: true,
        }
    }
    fn nonterminal(name: &'static str) -> Self {
        Self {
            name,
            is_terminal: false,
        }
    }
}

impl Visualize for MyNodeData {
    fn visualize(&self) -> std::string::String {
        self.name.to_string()
    }
    fn emphasize(&self) -> bool {
        self.is_terminal
    }
}

fn main() {
    let mut tree: Tree<MyNodeData> = TreeBuilder::new().with_node_capacity(5).build();

    let root_id: NodeId = tree
        .insert(Node::new(MyNodeData::nonterminal("calc")), AsRoot)
        .unwrap();

    let calc_list1_id: NodeId = tree
        .insert(
            Node::new(MyNodeData::nonterminal("calc_lst1")),
            UnderNode(&root_id),
        )
        .unwrap();

    let calc_lst1_itm1_id = tree
        .insert(
            Node::new(MyNodeData::nonterminal("calc_lst1_itm1")),
            UnderNode(&calc_list1_id),
        )
        .unwrap();

    let instruction_id = tree
        .insert(
            Node::new(MyNodeData::nonterminal("instruction")),
            UnderNode(&calc_lst1_itm1_id),
        )
        .unwrap();
    tree.insert(
        Node::new(MyNodeData::terminal(";")),
        UnderNode(&calc_lst1_itm1_id),
    )
    .unwrap();

    let assignment_id = tree
        .insert(
            Node::new(MyNodeData::nonterminal("assignment")),
            UnderNode(&instruction_id),
        )
        .unwrap();

    let assign_item_id = tree
        .insert(
            Node::new(MyNodeData::nonterminal("assign_item")),
            UnderNode(&assignment_id),
        )
        .unwrap();

    let id_id = tree
        .insert(
            Node::new(MyNodeData::nonterminal("id")),
            UnderNode(&assign_item_id),
        )
        .unwrap();

    tree.insert(Node::new(MyNodeData::terminal("c")), UnderNode(&id_id))
        .unwrap();

    let assign_op_id = tree
        .insert(
            Node::new(MyNodeData::nonterminal("assign_op")),
            UnderNode(&assign_item_id),
        )
        .unwrap();

    tree.insert(
        Node::new(MyNodeData::terminal("=")),
        UnderNode(&assign_op_id),
    )
    .unwrap();

    let logical_or_id = tree
        .insert(
            Node::new(MyNodeData::nonterminal("locigal_or")),
            UnderNode(&assignment_id),
        )
        .unwrap();

    let logical_and_id = tree
        .insert(
            Node::new(MyNodeData::nonterminal("locigal_and")),
            UnderNode(&logical_or_id),
        )
        .unwrap();

    let bitwise_or_id = tree
        .insert(
            Node::new(MyNodeData::nonterminal("bitwise_or")),
            UnderNode(&logical_and_id),
        )
        .unwrap();

    let bitwise_and_id = tree
        .insert(
            Node::new(MyNodeData::nonterminal("bitwise_and")),
            UnderNode(&bitwise_or_id),
        )
        .unwrap();

    let equality_id = tree
        .insert(
            Node::new(MyNodeData::nonterminal("equality")),
            UnderNode(&bitwise_and_id),
        )
        .unwrap();

    let relational_id = tree
        .insert(
            Node::new(MyNodeData::nonterminal("relational")),
            UnderNode(&equality_id),
        )
        .unwrap();

    let bitwise_shift_id = tree
        .insert(
            Node::new(MyNodeData::nonterminal("bitwise_shift")),
            UnderNode(&relational_id),
        )
        .unwrap();

    let sum_id = tree
        .insert(
            Node::new(MyNodeData::nonterminal("sum")),
            UnderNode(&bitwise_shift_id),
        )
        .unwrap();

    let mult_id = tree
        .insert(
            Node::new(MyNodeData::nonterminal("mult")),
            UnderNode(&sum_id),
        )
        .unwrap();

    let power_id = tree
        .insert(
            Node::new(MyNodeData::nonterminal("power")),
            UnderNode(&mult_id),
        )
        .unwrap();
    let factor_id = tree
        .insert(
            Node::new(MyNodeData::nonterminal("factor")),
            UnderNode(&power_id),
        )
        .unwrap();
    let number_id = tree
        .insert(
            Node::new(MyNodeData::nonterminal("number")),
            UnderNode(&factor_id),
        )
        .unwrap();
    tree.insert(Node::new(MyNodeData::terminal("2")), UnderNode(&number_id))
        .unwrap();

    let mult_lst1_id = tree
        .insert(
            Node::new(MyNodeData::nonterminal("mult_lst1")),
            UnderNode(&mult_id),
        )
        .unwrap();
    let mult_lst1_itm1_id = tree
        .insert(
            Node::new(MyNodeData::nonterminal("mult_lst1_itm1")),
            UnderNode(&mult_lst1_id),
        )
        .unwrap();
    let mult_item_id = tree
        .insert(
            Node::new(MyNodeData::nonterminal("mult_item")),
            UnderNode(&mult_lst1_itm1_id),
        )
        .unwrap();
    let mult_op_id = tree
        .insert(
            Node::new(MyNodeData::nonterminal("mult_op")),
            UnderNode(&mult_item_id),
        )
        .unwrap();
    tree.insert(Node::new(MyNodeData::terminal("*")), UnderNode(&mult_op_id))
        .unwrap();

    let power_id = tree
        .insert(
            Node::new(MyNodeData::nonterminal("power")),
            UnderNode(&mult_item_id),
        )
        .unwrap();
    let factor_id = tree
        .insert(
            Node::new(MyNodeData::nonterminal("factor")),
            UnderNode(&power_id),
        )
        .unwrap();
    let number_id = tree
        .insert(
            Node::new(MyNodeData::nonterminal("number")),
            UnderNode(&factor_id),
        )
        .unwrap();
    tree.insert(Node::new(MyNodeData::terminal("4")), UnderNode(&number_id))
        .unwrap();

    let sum_lst1_id = tree
        .insert(
            Node::new(MyNodeData::nonterminal("sum_lst1")),
            UnderNode(&sum_id),
        )
        .unwrap();
    let sum_lst1_itm1_id = tree
        .insert(
            Node::new(MyNodeData::nonterminal("sum_lst1_itm1")),
            UnderNode(&sum_lst1_id),
        )
        .unwrap();
    let sum_item_id = tree
        .insert(
            Node::new(MyNodeData::nonterminal("sum_item")),
            UnderNode(&sum_lst1_itm1_id),
        )
        .unwrap();
    let add_op_id = tree
        .insert(
            Node::new(MyNodeData::nonterminal("add_op")),
            UnderNode(&sum_item_id),
        )
        .unwrap();
    let plus_id = tree
        .insert(
            Node::new(MyNodeData::nonterminal("plus")),
            UnderNode(&add_op_id),
        )
        .unwrap();
    tree.insert(Node::new(MyNodeData::terminal("+")), UnderNode(&plus_id))
        .unwrap();

    let mult_id = tree
        .insert(
            Node::new(MyNodeData::nonterminal("mult")),
            UnderNode(&sum_item_id),
        )
        .unwrap();
    let power_id = tree
        .insert(
            Node::new(MyNodeData::nonterminal("power")),
            UnderNode(&mult_id),
        )
        .unwrap();
    let factor_id = tree
        .insert(
            Node::new(MyNodeData::nonterminal("factor")),
            UnderNode(&power_id),
        )
        .unwrap();
    let number_id = tree
        .insert(
            Node::new(MyNodeData::nonterminal("number")),
            UnderNode(&factor_id),
        )
        .unwrap();
    tree.insert(Node::new(MyNodeData::terminal("5")), UnderNode(&number_id))
        .unwrap();

    let drawer = SvgDrawer::new();
    let layouter = Layouter::with_tree(&tree)
        .with_drawer(&drawer)
        .with_file_name(std::path::Path::new("example2.svg"));
    layouter.write().expect("Failed writing layout")
}
