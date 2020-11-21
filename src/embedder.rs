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
/// External API: keep stable.
///
#[derive(Debug, Clone, Default)]
pub struct PlacedTreeItem {
    /// The nodes level, root has level 0. Can be used to calculate an y coordinate for the node
    pub y_order: usize,
    /// The logical x coordinate of the node's center
    pub x_center: usize,
    /// The x-extent of the nodes text representation in logical coordinate units
    pub x_extent: usize,
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

///
/// Conversion form internal to external (i.e. public) representation of the embedding structure.
///
impl From<ItemEmbeddingData> for PlacedTreeItem {
    fn from(e: ItemEmbeddingData) -> Self {
        Self {
            y_order: e.y_order,
            x_center: e.x_center,
            x_extent: e.x_extent,
            x_extent_children: e.x_extent_children,
            text: e.text,
            is_emphasized: e.is_emphasized,
            parent: e.parent,
            ord: e.ord,
        }
    }
}

///
/// The ItemEmbeddingData is the internal embedding information for one single tree node.
///
#[derive(Debug, Clone, Default)]
struct ItemEmbeddingData {
    /// The nodes level, root has level 0. Can be used to calculate an y coordinate for the node
    y_order: usize,
    /// The logical x coordinate of the node's center
    x_center: usize,
    /// The x-extent of the nodes text representation in logical coordinate units
    x_extent: usize,
    /// Internal value used to sum up the x-extent of all children of the node
    x_extent_of_children: usize,
    /// The maximum extent over the nodes text representation and the sum of all children's x-extent
    x_extent_children: usize,
    /// The text representation of the nodes data - created by the `Visualize` trait's implementation
    text: String,
    /// The *emphasize* property obtained from the `Visualize` trait
    is_emphasized: bool,
    /// The parent's `ord`, if there is one
    parent: Option<usize>,
    /// A unique number reflecting the topological post-ordering of the nodes in the tree
    ord: usize,
    /// Internal node id - The Option type used to circumvent missing Default implementation of `NodeId`s
    /// There should normally be no None values in there.
    node_id: Option<NodeId>,
}

///
/// Internal helper data
///
struct EmbeddingHelperData(HashMap<usize, ItemEmbeddingData>, HashMap<NodeId, usize>);

impl EmbeddingHelperData {
    fn new() -> Self {
        Self(HashMap::new(), HashMap::new())
    }

    fn get_by_ord(&self, ord: usize) -> Option<&ItemEmbeddingData> {
        self.0.get(&ord)
    }

    fn get_mut_by_ord(&mut self, ord: usize) -> Option<&mut ItemEmbeddingData> {
        self.0.get_mut(&ord)
    }

    fn get_by_node_id(&self, node_id: &NodeId) -> Option<&ItemEmbeddingData> {
        self.1.get(node_id).map(|n| self.0.get(n)).flatten()
    }

    fn get_mut_by_node_id(&mut self, node_id: &NodeId) -> Option<&mut ItemEmbeddingData> {
        let ord = self.1.get(node_id).cloned();
        ord.map(move |n| self.0.get_mut(&n)).flatten()
    }

    fn insert(&mut self, ord: usize, item: ItemEmbeddingData) {
        item.node_id.as_ref().map(|n| self.1.insert(n.clone(), ord));
        self.0.insert(ord, item);
    }
}

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
        debug_assert_eq!(items.0.len(), items.1.len());

        // Set depth (y_order) on each ItemEmbeddingData structure
        // After this step each item has following properties set:
        // 'x_extent', 'text', 'is_emphasized', 'x_extent_children', 'ord', 'parent', 'y_order'
        Self::apply_y_order(tree, &mut items);

        // Finally set the property 'x_center' from leafs to root
        // After this step each item has all necessary properties set
        Self::apply_x_center(tree, &mut items);

        // Transfer result
        Self::transfer_result(items)
    }

    fn create_initial_embedding_data(tree: &Tree<T>) -> EmbeddingHelperData {
        fn create_from_node<T: Visualize>(
            node_id: &NodeId,
            ord: usize,
            tree: &Tree<T>,
            items: &EmbeddingHelperData,
        ) -> ItemEmbeddingData {
            let node = tree.get(node_id).unwrap();
            let text = node.data().visualize();
            let y_order = 0;
            let x_center = 0;
            let x_extent = text.len() + 1;
            let x_extent_of_children = node.children().iter().fold(0, |acc, child_node_id| {
                if let Some(placed_item) = items.get_by_node_id(child_node_id) {
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
            let node_id = Some(node_id.clone());

            ItemEmbeddingData {
                y_order,
                x_center,
                x_extent,
                x_extent_of_children,
                x_extent_children,
                text,
                is_emphasized,
                parent,
                ord,
                node_id,
            }
        }

        let mut items = EmbeddingHelperData::new();

        if let Some(root_node_id) = tree.root_node_id() {
            for (ord, node_id) in tree
                .traverse_post_order_ids(root_node_id)
                .unwrap()
                .enumerate()
            {
                let new_item = create_from_node(&node_id, ord, tree, &items);
                let _ = items.insert(ord, new_item);
            }
        }

        items
    }

    fn apply_y_order<'a>(tree: &Tree<T>, items: &'a mut EmbeddingHelperData) {
        if let Some(root_node_id) = tree.root_node_id() {
            for node_id in tree.traverse_pre_order_ids(root_node_id).unwrap() {
                let level = tree.ancestor_ids(&node_id).unwrap().count();
                let parent = tree
                    .ancestor_ids(&node_id)
                    .unwrap()
                    .next()
                    .map(|id| items.get_by_node_id(id).unwrap().ord);
                let item = items.get_mut_by_node_id(&node_id).unwrap();
                item.y_order = level;
                item.parent = parent;
            }
        };
    }

    fn apply_x_center(tree: &Tree<T>, items: &mut EmbeddingHelperData) {
        fn x_center_layer(layer: usize, items: &mut EmbeddingHelperData) {
            let node_ids_in_layer = items.0.iter().fold(Vec::new(), |mut acc, (ord, item)| {
                if item.y_order == layer {
                    acc.push(*ord)
                }
                acc
            });

            let parents_in_layer = node_ids_in_layer
                .iter()
                .map(|ord| items.get_by_ord(*ord).unwrap().parent)
                .collect::<Vec<Option<usize>>>();

            for p in parents_in_layer {
                let mut nodes_in_layer_per_parent = node_ids_in_layer
                    .iter()
                    .filter_map(|ord| {
                        if items.get_by_ord(*ord).unwrap().parent == p {
                            Some(*ord)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<usize>>();
                nodes_in_layer_per_parent.sort_by_key(|n| items.get_by_ord(*n).unwrap().ord);

                let mut moving_x_center = {
                    if let Some(parent_ord) = p {
                        if let Some(placed_parent_item) = items.get_by_ord(parent_ord) {
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
                for ord in nodes_in_layer_per_parent {
                    if let Some(placed_item) = items.get_mut_by_ord(ord) {
                        placed_item.x_center = moving_x_center + placed_item.x_extent_children / 2;
                        moving_x_center += placed_item.x_extent_children;
                    }
                }
            }
        }

        for l in 0..tree.height() + 1 {
            x_center_layer(l, items);
        }
    }

    /// Transforming the internal `EmbeddingHelperMap` to the external representation `Embedding`.
    /// The `items` parameter is hereby consumed.
    fn transfer_result(items: EmbeddingHelperData) -> Embedding {
        let len = items.0.len();
        items
            .0
            .into_iter()
            .fold(Embedding::with_capacity(len), |mut acc, e| {
                acc.push(e.1.into());
                acc
            })
    }
}
