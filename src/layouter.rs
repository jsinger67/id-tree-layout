use crate::layouter_error;
use crate::layouter_error::LayouterError;
use crate::Drawer;
use crate::Embedder;
use crate::Visualize;
use id_tree::Tree;

pub type Result = layouter_error::Result<()>;

pub struct Layouter<'a, 'c, 'b, T, D>
where
    T: Visualize,
    D: Drawer,
{
    tree: &'a Tree<T>,
    drawer: Option<&'b D>,
    file_name: Option<&'c std::path::Path>,
}

impl<'a, 'c, 'b, T, D> Layouter<'a, 'c, 'b, T, D>
where
    T: Visualize,
    D: Drawer,
{
    pub fn with_tree(tree: &'a Tree<T>) -> Self {
        Self {
            tree,
            drawer: None,
            file_name: None,
        }
    }

    pub fn with_file_name(mut self, path: &'c std::path::Path) -> Self {
        self.file_name = Some(path);
        self
    }

    pub fn with_drawer(mut self, drawer: &'b D) -> Self {
        self.drawer = Some(drawer);
        self
    }

    pub fn write(&self) -> Result {
        if self.drawer.is_none() {
            Err(LayouterError::from_description(
                "No drawer set - use Layouter::with_drawer.".to_string(),
            ))
        } else if self.file_name.is_none() {
            Err(LayouterError::from_description(
                "No output file name given - use Layouter::with_file_name.".to_string(),
            ))
        } else {
            let embedding = Embedder::embed(self.tree);
            let drawer = self.drawer.unwrap() as &dyn Drawer;
            let result = drawer.draw(self.file_name.unwrap(), &embedding);
            match result {
                Err(err) => Err(LayouterError::from_ioerror(err)),
                Ok(()) => Ok(()),
            }
        }
    }
}
