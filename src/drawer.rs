use crate::embedder::PlacedTreeItem;

pub type Result = std::io::Result<()>;

/// By implementing this trait anyone can provide his own drawer, i.e. one that draws onto a bitmap.
/// This crate provides the SvgDrawer as a simple and ready-to-use implementation of this trait.
pub trait Drawer {
    fn draw(&self, file_name: &std::path::Path, embedding: &[PlacedTreeItem]) -> Result;
}
