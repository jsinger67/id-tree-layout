//! The module with the `Drawer` trait.
use crate::embedder::PlacedTreeItem;

///
/// The `Drawer`'s result type is the `std::io::Result` with `Unit` as success type.
///
pub type Result = std::io::Result<()>;

///
/// By implementing this trait anyone can provide his own drawer, for instance one that draws onto
/// a bitmap, if he don't want to use the `SvgDrawer` used by the crate by default.
///
pub trait Drawer {
    fn draw(&self, file_name: &std::path::Path, embedding: &[PlacedTreeItem]) -> Result;
}
