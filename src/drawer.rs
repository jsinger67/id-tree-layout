use crate::embedder::PlacedTreeItem;

pub type Result = std::io::Result<()>;

pub trait Drawer {
    fn draw(&self, file_name: &std::path::Path, embedding: &[PlacedTreeItem]) -> Result;
}
