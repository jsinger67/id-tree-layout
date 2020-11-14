//! The module that holds types to embed nodes of a tree into the plane.

use crate::visualize::Visualize;
use id_tree::{NodeId, Tree};
use std::collections::hash_map::DefaultHasher;
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};

///
/// The Embedding is the interface to drawers that need the embedding
/// to transform it to their own format.
///
pub type Embedding = Vec<PlacedTreeItem>;

///
/// The PlacedTreeItem is the embedding information for one single tree node.
/// It is used only in a collection type `Embedding`.
///
#[derive(Debug, Clone, Default)]
pub struct PlacedTreeItem {
    pub y_order: usize,
    pub x_center: usize,
    pub x_extend: usize,
    x_extend_of_children: usize,
    pub x_extend_children: usize,
    pub name: String,
    pub is_emphasized: bool,
    pub id: u64,
    pub parent: Option<u64>,
    pub ord: usize,
}

type EmbeddingHelperMap = BTreeMap<NodeId, PlacedTreeItem>;

///
/// The Embedder type provides a single public method `embed` to arrange nodes of a tree into the
/// plane.
///
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
    ///
    /// This method creates an embedding of the nodes of the given tree in the plane.
    ///
    /// # Panics
    ///
    /// The method should not panic. If you encounter a panic this should be originated from
    /// bugs in coding. Please report such panics.
    ///
    /// # Complexity
    ///
    /// The algorithm is of complexity class O(n).
    ///
    pub fn embed(tree: &Tree<T>) -> Embedding {
        // Insert all tree items with their indices
        // After this step each item has following properties set:
        // 'x_extend', 'name', 'is_emphasized', 'x_extend_children', 'id', 'parent'
        let mut items = Self::create_initial_embedding_data(tree);

        // Set depth (y_order) on each PlacedTreeItem structure
        // After this step each item has following properties set:
        // 'x_extend', 'name', 'is_emphasized', 'x_extend_children', 'id', 'parent', 'y_order'
        Self::apply_y_order(tree, &mut items);

        // Finally set the property 'x_center' from leafs to root
        // After this step each item has all necessary properties set
        Self::apply_x_center(tree, &mut items);

        // Transfer result
        Self::transfer_result(items)
    }

    fn create_initial_embedding_data(tree: &Tree<T>) -> EmbeddingHelperMap {
        fn create_from_node<T: Visualize>(
            node_id: &NodeId,
            ord: usize,
            tree: &Tree<T>,
            items: &EmbeddingHelperMap,
        ) -> PlacedTreeItem {
            let node = tree.get(node_id).unwrap();
            let name = node.data().visualize();
            let y_order = 0;
            let x_center = 0;
            let x_extend = name.len() + 1;
            let x_extend_of_children = node.children().iter().fold(0, |acc, child_node_id| {
                if let Some(placed_item) = items.get(child_node_id) {
                    acc + placed_item.x_extend_children
                } else {
                    // The `id_tree::Tree<T>::traverse_post_order_ids` used to visit the nodes
                    // should always ensure that child nodes are visited before their parent nodes
                    // are.
                    // If you encounter this panic, please report!
                    panic!("Child node should have already visited!");
                }
            });
            let x_extend_children = std::cmp::max(x_extend, x_extend_of_children);
            let is_emphasized = node.data().emphasize();
            let id = Embedder::<T>::get_node_id_hash(node_id);
            let parent = node.parent().map(|p| Embedder::<T>::get_node_id_hash(p));

            PlacedTreeItem {
                y_order,
                x_center,
                x_extend,
                x_extend_of_children,
                x_extend_children,
                name,
                is_emphasized,
                id,
                parent,
                ord,
            }
        }

        let mut items = EmbeddingHelperMap::new();

        if let Some(root_node_id) = tree.root_node_id() {
            for (ord, node_id) in tree
                .traverse_post_order_ids(root_node_id)
                .unwrap()
                .enumerate()
            {
                let new_item = create_from_node(&node_id, ord, tree, &items);
                let _ = items.insert(node_id, new_item);
            }
        }

        items
    }

    fn apply_y_order<'a>(tree: &Tree<T>, items: &'a mut EmbeddingHelperMap) {
        if let Some(root_node_id) = tree.root_node_id() {
            for node_id in tree.traverse_pre_order_ids(root_node_id).unwrap() {
                let item = items.get_mut(&node_id).unwrap();
                item.y_order = tree.ancestor_ids(&node_id).unwrap().count();
            }
        };
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
                .collect::<Vec<Option<&NodeId>>>();

            for p in parents_in_layer {
                let mut nodes_in_layer_per_parent = node_ids_in_layer
                    .iter()
                    .filter_map(|node_id| {
                        if tree.get(node_id).unwrap().parent() == p {
                            Some(node_id.clone())
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<NodeId>>();
                nodes_in_layer_per_parent.sort_by_key(|n| items.get(n).unwrap().ord);

                let mut moving_x_center = {
                    if let Some(parent_node_id) = p {
                        if let Some(placed_parent_item) = items.get(&parent_node_id) {
                            // We start half way left from the parents x center
                            placed_parent_item.x_center
                                - placed_parent_item.x_extend_of_children / 2
                        } else {
                            // This really should not happen, because the parent_node_id was
                            // previously retrieved from the tree itself. And the tree is not
                            // touched at all.
                            panic!("Some item expected here!")
                        }
                    } else {
                        // `None` means we are in layer 0
                        debug_assert_eq!(layer, 0);
                        // and we should have only one root
                        debug_assert_eq!(node_ids_in_layer.len(), 1);
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

    /// To not being forced to convey `NodeId`'s out of the tree we simply use their hash value as
    /// an appropriate, unique identifier. The `NodeId` is `Hash`.
    fn get_node_id_hash(node_id: &NodeId) -> u64 {
        let mut hasher = DefaultHasher::new();
        node_id.clone().hash(&mut hasher);
        hasher.finish()
    }

    /// Transforming the internal `EmbeddingHelperMap` to the external representation `Embedding`.
    /// The `items` parameter is hereby consumed.
    fn transfer_result(items: EmbeddingHelperMap) -> Embedding {
        let mut embedding_result = Embedding::with_capacity(items.len());
        for (_, e) in items {
            embedding_result.push(e);
        }
        embedding_result
    }
}
