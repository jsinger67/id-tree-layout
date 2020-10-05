//! The Visualize trait is used for the realization of the embedding of a Tree<T>.
//! It should be implemented by the Trees node type T.

pub trait Visualize {
    /// Returns the string representation of the nodes data.
    fn visualize(&self) -> String;

    /// When this function returns true the drawer can emphasize the node's string representation in
    /// an implementation dependend way, i.e. it can print it bold.
    fn emphasize(&self) -> bool;
}
