use id_tree::InsertBehavior::*;
use id_tree::*;
use id_tree_layout::*;

struct MyNodeData(i32);

impl Visualize for MyNodeData {
    fn visualize(&self) -> std::string::String {
        self.0.to_string()
    }
}

#[test]
fn empty_tree() {
    let tree: Tree<MyNodeData> = TreeBuilder::new().build();

    let embedding = Embedder::embed(&tree);

    assert!(embedding.is_empty());
}

#[test]
fn tree_with_single_node() {
    let mut tree: Tree<MyNodeData> = TreeBuilder::new().with_node_capacity(1).build();
    let _ = tree.insert(Node::new(MyNodeData(0)), AsRoot).ok().unwrap();

    let embedding = Embedder::embed(&tree);

    assert_eq!(1, embedding.len());

    {
        let e = &embedding[0];
        assert_eq!("0".to_string(), e.name);
        assert_eq!(0, e.y_order);
        assert_eq!(1, e.x_center);
        assert_eq!(2, e.x_extend);
        assert_eq!(2, e.x_extend_children);
    }
}

#[test]
fn more_complex_tree() {
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

    let embedding = Embedder::embed(&tree);

    assert!(!embedding.is_empty());
    assert_eq!(5, embedding.len());

    {
        let e = &embedding[0];
        assert_eq!("0".to_string(), e.name);
        assert_eq!(0, e.y_order);
        assert_eq!(3, e.x_center);
        assert_eq!(2, e.x_extend);
        assert_eq!(6, e.x_extend_children);
    }
    {
        let e = &embedding[1];
        assert_eq!("1".to_string(), e.name);
        assert_eq!(1, e.y_order);
        assert_eq!(2, e.x_center);
        assert_eq!(2, e.x_extend);
        assert_eq!(4, e.x_extend_children);
    }
    {
        let e = &embedding[2];
        assert_eq!("2".to_string(), e.name);
        assert_eq!(1, e.y_order);
        assert_eq!(5, e.x_center);
        assert_eq!(2, e.x_extend);
        assert_eq!(2, e.x_extend_children);
    }
    {
        let e = &embedding[3];
        assert_eq!("3".to_string(), e.name);
        assert_eq!(2, e.y_order);
        assert_eq!(1, e.x_center);
        assert_eq!(2, e.x_extend);
        assert_eq!(2, e.x_extend_children);
    }
    {
        let e = &embedding[4];
        assert_eq!("4".to_string(), e.name);
        assert_eq!(2, e.y_order);
        assert_eq!(3, e.x_center);
        assert_eq!(2, e.x_extend);
        assert_eq!(2, e.x_extend_children);
    }
}

#[test]
fn moved_nodes() {
    //      0 ---------
    //     / \    \    \
    //    1   2    4    3
    let mut tree: Tree<MyNodeData> = TreeBuilder::new().with_node_capacity(5).build();

    let root_id: NodeId = tree.insert(Node::new(MyNodeData(0)), AsRoot).unwrap();
    let n1_id: NodeId = tree
        .insert(Node::new(MyNodeData(1)), UnderNode(&root_id))
        .unwrap();
    tree.insert(Node::new(MyNodeData(2)), UnderNode(&root_id))
        .unwrap();
    let n4_id = tree
        .insert(Node::new(MyNodeData(4)), UnderNode(&root_id))
        .unwrap();
    let n3_id = tree.insert(Node::new(MyNodeData(3)), UnderNode(&root_id))
        .unwrap();

    //      0
    //     / \
    //    1   2
    //   / \
    //  3   4
    let _ = tree.move_node(&n3_id, MoveBehavior::ToParent(&n1_id));
    let _ = tree.move_node(&n4_id, MoveBehavior::ToParent(&n1_id));

    let embedding = Embedder::embed(&tree);

    assert!(!embedding.is_empty());
    assert_eq!(5, embedding.len());

    {
        let e = &embedding[0];
        assert_eq!("0".to_string(), e.name);
        assert_eq!(0, e.y_order);
        assert_eq!(3, e.x_center);
        assert_eq!(2, e.x_extend);
        assert_eq!(6, e.x_extend_children);
    }
    {
        let e = &embedding[1];
        assert_eq!("1".to_string(), e.name);
        assert_eq!(1, e.y_order);
        assert_eq!(2, e.x_center);
        assert_eq!(2, e.x_extend);
        assert_eq!(4, e.x_extend_children);
    }
    {
        let e = &embedding[2];
        assert_eq!("2".to_string(), e.name);
        assert_eq!(1, e.y_order);
        assert_eq!(5, e.x_center);
        assert_eq!(2, e.x_extend);
        assert_eq!(2, e.x_extend_children);
    }
    {
        let e = &embedding[3];
        assert_eq!("4".to_string(), e.name);
        assert_eq!(2, e.y_order);
        assert_eq!(3, e.x_center);
        assert_eq!(2, e.x_extend);
        assert_eq!(2, e.x_extend_children);
    }
    {
        let e = &embedding[4];
        assert_eq!("3".to_string(), e.name);
        assert_eq!(2, e.y_order);
        assert_eq!(1, e.x_center);
        assert_eq!(2, e.x_extend);
        assert_eq!(2, e.x_extend_children);
    }
}
