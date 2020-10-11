//! The module with the **Public API that is highly encouraged to be used**.
use crate::layouter_error;
use crate::{Drawer, Embedder, LayouterError, SvgDrawer, Visualize};
use id_tree::Tree;

///
/// The Result type that is uses within the public API `Layouter`.
///
pub type Result = layouter_error::Result<()>;

///
/// The Layouter type provides a simple builder mechanism with a fluent API.
///
pub struct Layouter<'a, 'b, 'c, T>
where
    T: Visualize,
{
    tree: &'a Tree<T>,
    drawer: Option<&'b dyn Drawer>,
    file_name: Option<&'c std::path::Path>,
}

impl<'a, 'b, 'c, T> Layouter<'a, 'b, 'c, T>
where
    T: Visualize,
{
    ///
    /// Creates a new Layouter with the required tree.
    ///
    /// ```
    /// use id_tree_layout::{Layouter, Visualize};
    /// use id_tree::{Tree, TreeBuilder};
    ///
    /// struct MyNodeData(i32);
    ///
    /// impl Visualize for MyNodeData {
    ///     fn visualize(&self) -> std::string::String { self.0.to_string() }
    ///     fn emphasize(&self) -> bool { false }
    /// }
    ///
    ///
    /// let tree: Tree<MyNodeData> = TreeBuilder::new().build();
    /// let layouter = Layouter::new(&tree);
    /// ```
    ///
    pub fn new(tree: &'a Tree<T>) -> Self {
        Self {
            tree,
            drawer: None,
            file_name: None,
        }
    }

    ///
    /// Sets the path of the output file on the layouter.
    ///
    /// ```
    /// use id_tree_layout::{Layouter, Visualize};
    /// use id_tree::{Tree, TreeBuilder};
    /// use std::path::Path;
    ///
    /// struct MyNodeData(i32);
    ///
    /// impl Visualize for MyNodeData {
    ///     fn visualize(&self) -> std::string::String { self.0.to_string() }
    ///     fn emphasize(&self) -> bool { false }
    /// }
    ///
    ///
    /// let tree: Tree<MyNodeData> = TreeBuilder::new().build();
    /// let layouter = Layouter::new(&tree)
    ///     .with_file_path(Path::new("test.svg"));
    /// ```
    ///
    pub fn with_file_path(self, path: &'c std::path::Path) -> Self {
        Self {
            tree: self.tree,
            file_name: Some(path),
            drawer: self.drawer,
        }
    }

    ///
    /// Sets a different drawer when you don't want to use the default svg-drawer.
    /// If this method is not called the crate's own svg-drawer is used.
    ///
    /// ```
    /// use id_tree_layout::{Drawer, Layouter, PlacedTreeItem, Visualize};
    /// use id_tree_layout::drawer::Result;
    /// use id_tree::{Tree, TreeBuilder};
    /// use std::path::Path;
    ///
    /// struct NilDrawer;
    /// impl Drawer for NilDrawer {
    ///     fn draw(&self, _file_name: &Path, _embedding: &[PlacedTreeItem]) -> Result {
    ///         Ok(())
    ///     }
    /// }
    ///
    /// struct MyNodeData(i32);
    ///
    /// impl Visualize for MyNodeData {
    ///     fn visualize(&self) -> std::string::String { self.0.to_string() }
    ///     fn emphasize(&self) -> bool { false }
    /// }
    ///
    ///
    /// let tree: Tree<MyNodeData> = TreeBuilder::new().build();
    /// let drawer = NilDrawer;
    /// let layouter = Layouter::new(&tree)
    ///     .with_drawer(&drawer)
    ///     .with_file_path(Path::new("test.svg"));
    /// ```
    ///
    pub fn with_drawer(self, drawer: &'b dyn Drawer) -> Self {
        Self {
            tree: self.tree,
            file_name: self.file_name,
            drawer: Some(drawer),
        }
    }

    ///
    /// When the layouter instance is fully configured this method invokes the necessary embedding
    /// functionality and uses the drawer which writes the result to the output file in its own
    /// output format.
    ///
    /// ```
    /// use id_tree_layout::{Layouter, Visualize};
    /// use id_tree::{Tree, TreeBuilder};
    /// use std::path::Path;
    ///
    /// struct MyNodeData(i32);
    ///
    /// impl Visualize for MyNodeData {
    ///     fn visualize(&self) -> std::string::String { self.0.to_string() }
    ///     fn emphasize(&self) -> bool { false }
    /// }
    ///
    ///
    /// let tree: Tree<MyNodeData> = TreeBuilder::new().build();
    /// Layouter::new(&tree)
    ///     .with_file_path(Path::new("test.svg"))
    ///     .write().expect("Failed writing layout")
    /// ```
    ///
    pub fn write(&self) -> Result {
        if self.file_name.is_none() {
            Err(LayouterError::from_description(
                "No output file name given - use Layouter::with_file_path.".to_string(),
            ))
        } else {
            let embedding = Embedder::embed(self.tree);
            let default_drawer = SvgDrawer::new();
            let drawer = self.drawer.unwrap_or(&default_drawer);
            drawer
                .draw(self.file_name.unwrap(), &embedding)
                .map_err(LayouterError::from_io_error)
        }
    }
}
