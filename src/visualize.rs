//! The visualize module provides the `Visualize` trait.

/// The `Visualize` trait abstracts the visual presentation of the node's data.
/// It should be implemented by the Tree<T>'s node type T.
/// Only mandatory to implement is the `visualize` function.
pub trait Visualize {
    /// Returns the string representation of the nodes data.
    fn visualize(&self) -> String;

    /// When this function returns true the drawer can emphasize the node's string representation
    /// in an implementation dependent way, i.e. it can print it bold.
    fn emphasize(&self) -> bool {
        false
    }
}
