use std::collections::hash_map::DefaultHasher;
use crate::visualize::Visualize;
use id_tree::{NodeId, Tree};
use std::collections::{BTreeMap, BTreeSet};
use std::hash::{Hash, Hasher};

pub type Embedding = Vec<PlacedTreeItem>;

#[derive(Debug, Clone, Default)]
pub struct PlacedTreeItem {
    pub y_order: usize,
    pub x_center: usize,
    pub x_extend: usize,
    x_extend_of_children: usize,
    pub x_extend_children: usize,
    pub name: String,
    pub is_empasized: bool,
    pub id: u64,
    pub parent: Option<u64>,
}

impl PlacedTreeItem {}

type EmbeddingHelperMap = BTreeMap<NodeId, PlacedTreeItem>;

pub struct Embedder<T>
where
    T: Visualize,
{
    _1: std::marker::PhantomData<T>,
}

impl<T> Embedder<T>
where
    T: Visualize,
{
    pub fn embed(tree: &Tree<T>) -> Embedding {
        // Insert all tree items with their indices
        // After this step each item has following properties set: 'x_extend', 'name', 'is_empasized', 'x_extend_children', 'id', 'parent'
        let mut items = Self::create_initial_embedding_data(tree);

        // Set depth (y_order) on each PlacedTreeItem structure
        // After this step each item has following properties set: 'x_extend', 'name', 'is_empasized', 'x_extend_children', 'id', 'parent', 'parent_index', 'y_order'
        Self::apply_y_order(tree, &mut items);

        // Finally set the property 'x_center' from leafs to root
        // After this step each item has all necessary properties set
        Self::apply_x_center(tree, &mut items);

        // Transfer result
        Self::transfer_result(items)
    }

    fn create_initial_embedding_data<'a>(tree: &'a Tree<T>) -> EmbeddingHelperMap {
        fn create_from_node<T: Visualize>(
            node_id: &NodeId,
            tree: &Tree<T>,
            items: &EmbeddingHelperMap,
        ) -> PlacedTreeItem {
            let node = tree.get(node_id).unwrap();
            let name = node.data().visualize();
            let xe = name.len() + 1;
            let xec = node.children().iter().fold(0, |acc, child_node_id| {
                if let Some(placed_item) = items.get(child_node_id) {
                    acc + placed_item.x_extend_children
                } else {
                    panic!("Child node should already exist!");
                }
            });
            let id = Embedder::<T>::get_node_id_hash(node_id);
            let parent = node.parent().map(|p| Embedder::<T>::get_node_id_hash(p));
            PlacedTreeItem {
                y_order: 0,
                x_center: 0,
                x_extend: xe,
                x_extend_of_children: xec,
                x_extend_children: std::cmp::max(xe, xec),
                name,
                is_empasized: node.data().emphasize(),
                id,
                parent
            }
        }

        let mut items = EmbeddingHelperMap::new();

        for node_id in tree
            .traverse_pre_order_ids(tree.root_node_id().unwrap())
            .unwrap()
        {
            let new_item = create_from_node(&node_id, tree, &items);
            let _ = items.insert(node_id.clone(), new_item);
        }

        items
    }

    fn apply_y_order<'a>(tree: &Tree<T>, items: &'a mut EmbeddingHelperMap) {
        for node_id in tree
            .traverse_pre_order_ids(tree.root_node_id().unwrap())
            .unwrap()
        {
            let item = items.get_mut(&node_id).unwrap();
            item.y_order = tree.ancestor_ids(&node_id).unwrap().count();
        }
    }

    fn apply_x_center(tree: &Tree<T>, items: &mut EmbeddingHelperMap) {
        fn x_center_layer<T>(layer: usize, tree: &Tree<T>, items: &mut EmbeddingHelperMap) {
            let node_ids_in_layer = items.iter().fold(Vec::new(), |mut acc, (node_id, item)| {
                if item.y_order == layer {
                    acc.push(node_id.clone())
                }
                acc
            });

            let parents_in_layer = node_ids_in_layer
                .iter()
                .map(|node_id| tree.get(node_id).unwrap().parent())
                .collect::<BTreeSet<Option<&NodeId>>>();

            for p in parents_in_layer {
                let nodes_in_layer_per_parent = node_ids_in_layer
                    .iter()
                    .filter_map(|node_id| {
                        if tree.get(node_id).unwrap().parent() == p {
                            Some(node_id.clone())
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<NodeId>>();

                let mut moving_x_center = {
                    if let Some(parent_node_id) = p {
                        if let Some(placed_parent_item) = items.get(&parent_node_id) {
                            // We start half way left from the parents x center
                            placed_parent_item.x_center
                                - placed_parent_item.x_extend_of_children / 2
                        } else {
                            panic!("Some item expected here!")
                        }
                    } else {
                        // None means we are in layer 0
                        assert_eq!(layer, 0);
                        // and we should have only one root
                        assert_eq!(node_ids_in_layer.len(), 1);
                        // We start all the way left
                        0
                    }
                };
                for node_id in nodes_in_layer_per_parent {
                    if let Some(placed_item) = items.get_mut(&node_id) {
                        placed_item.x_center = moving_x_center + placed_item.x_extend_children / 2;
                        moving_x_center += placed_item.x_extend_children;
                    }
                }
            }
        }

        for l in 0..tree.height() + 1 {
            x_center_layer(l, tree, items);
        }
    }

    fn get_node_id_hash(node_id: &NodeId) -> u64 {
        let mut hasher = DefaultHasher::new();
        node_id.clone().hash(&mut hasher);
        hasher.finish()
    }

    fn transfer_result(items: EmbeddingHelperMap) -> Embedding {
        let mut embedding_result = Embedding::with_capacity(items.len());
        for (_, e) in items {
            embedding_result.push(e);
        }
        embedding_result
    }
}
