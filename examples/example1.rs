use id_tree::InsertBehavior::{AsRoot, UnderNode};
use id_tree::{Node, NodeId, Tree, TreeBuilder};
use id_tree_layout::{Layouter, Visualize};

struct MyNodeData(i32);

// You need to implement id_tree_layout::Visualize for your nodes data type.
// This way you provide basic formatting information.
impl Visualize for MyNodeData {
    fn visualize(&self) -> std::string::String {
        // We simply convert the i32 value to string here.
        self.0.to_string()
    }
    fn emphasize(&self) -> bool {
        // This simply emphasizes only the leaf nodes.
        // It only works for this example.
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

    let layouter =
        Layouter::new(&tree).with_file_path(std::path::Path::new("examples/example1.svg"));
    layouter.write().expect("Failed writing layout")
}
