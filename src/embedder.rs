//! The module that holds types to embed nodes of a tree into the plane.

use crate::visualize::Visualize;
use id_tree::{NodeId, Tree};
use std::collections::HashMap;

///
/// The Embedding is the interface to drawers that need the embedding
/// to transform it to their own format.
///
pub type Embedding = Vec<PlacedTreeItem>;

///
/// The PlacedTreeItem is the embedding information for one single tree node.
/// It is used only in a collection type `Embedding`.
/// Due to private member(s) this struct can't be created outside.
///
#[derive(Debug, Clone, Default)]
pub struct PlacedTreeItem {
    /// The nodes level, root has level 0. Can be used to calculate an y coordinate for the node
    pub y_order: usize,
    /// The logical x coordinate of the node's center
    pub x_center: usize,
    /// The x-extent of the nodes text representation in logical coordinate units
    pub x_extent: usize,
    /// Internal value used to sum up the x-extent of all children of the node
    x_extent_of_children: usize,
    /// The maximum extent over the nodes text representation and the sum of all children's x-extent
    pub x_extent_children: usize,
    /// The text representation of the nodes data - created by the `Visualize` trait's implementation
    pub text: String,
    /// The *emphasize* property obtained from the `Visualize` trait
    pub is_emphasized: bool,
    /// The parent's `ord`, if there is one
    pub parent: Option<usize>,
    /// A unique number reflecting the topological post-ordering of the nodes in the tree
    pub ord: usize,
}

type EmbeddingHelperMap = HashMap<NodeId, PlacedTreeItem>;

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
        // 'x_extent', 'text', 'is_emphasized', 'x_extent_children', 'ord'
        let mut items = Self::create_initial_embedding_data(tree);

        // Set depth (y_order) on each PlacedTreeItem structure
        // After this step each item has following properties set:
        // 'x_extent', 'text', 'is_emphasized', 'x_extent_children', 'ord', 'parent', 'y_order'
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
            let text = node.data().visualize();
            let y_order = 0;
            let x_center = 0;
            let x_extent = text.len() + 1;
            let x_extent_of_children = node.children().iter().fold(0, |acc, child_node_id| {
                if let Some(placed_item) = items.get(child_node_id) {
                    acc + placed_item.x_extent_children
                } else {
                    // The `id_tree::Tree<T>::traverse_post_order_ids` used to visit the nodes
                    // should always ensure that child nodes are visited before their parent nodes
                    // are.
                    // If you encounter this panic, please report!
                    panic!("Child node should have already visited!");
                }
            });
            let x_extent_children = std::cmp::max(x_extent, x_extent_of_children);
            let is_emphasized = node.data().emphasize();
            let parent = None;

            PlacedTreeItem {
                y_order,
                x_center,
                x_extent,
                x_extent_of_children,
                x_extent_children,
                text,
                is_emphasized,
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
                let level = tree.ancestor_ids(&node_id).unwrap().count();
                let parent = tree.ancestor_ids(&node_id).unwrap().next().map(|id| items.get(id).unwrap().ord );
                let item = items.get_mut(&node_id).unwrap();
                item.y_order = level;
                item.parent = parent;
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
                                - placed_parent_item.x_extent_of_children / 2
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
                        placed_item.x_center = moving_x_center + placed_item.x_extent_children / 2;
                        moving_x_center += placed_item.x_extent_children;
                    }
                }
            }
        }

        for l in 0..tree.height() + 1 {
            x_center_layer(l, tree, items);
        }
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
