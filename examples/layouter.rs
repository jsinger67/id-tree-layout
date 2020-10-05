use id_tree::InsertBehavior::{AsRoot, UnderNode};
use id_tree::*;
use id_tree_layout::Layouter;
use id_tree_layout::SvgDrawer;
use id_tree_layout::Visualize;

struct MyNodeData(i32);

impl Visualize for MyNodeData {
    fn visualize(&self) -> std::string::String {
        self.0.to_string()
    }
    fn emphasize(&self) -> bool {
        self.0 > 1
    }
}

fn main() {
    //      0
    //     / \
    //    1   2
    //   / \
    //  3   4
    let mut tree: Tree<MyNodeData> = TreeBuilder::new().with_node_capacity(5).build();

    let root_id: NodeId = tree.insert(Node::new(MyNodeData(0)), AsRoot).unwrap();
    let child_id: NodeId = tree
        .insert(Node::new(MyNodeData(1)), UnderNode(&root_id))
        .unwrap();
    tree.insert(Node::new(MyNodeData(2)), UnderNode(&root_id))
        .unwrap();
    tree.insert(Node::new(MyNodeData(3)), UnderNode(&child_id))
        .unwrap();
    tree.insert(Node::new(MyNodeData(4)), UnderNode(&child_id))
        .unwrap();

    let drawer = SvgDrawer::new();
    let layouter = Layouter::with_tree(&tree)
        .with_drawer(&drawer)
        .with_file_name(std::path::Path::new("example1.svg"));
    layouter.write().expect("Failed writing layout")
}
