#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use id_tree::Tree;
use id_tree_layout::{Layouter, SvgDrawer, Visualize};

#[derive(Debug, Serialize, Deserialize)]
struct MyNodeData<'a> {
    name: &'a str,
    is_terminal: bool,
}

impl<'a> Visualize for MyNodeData<'a> {
    fn visualize(&self) -> std::string::String {
        self.name.to_string()
    }
    fn emphasize(&self) -> bool {
        self.is_terminal
    }
}

fn main() {
    let json = std::fs::read_to_string("examples/parse_tree.json")
        .unwrap_or_else(|_| panic!("Can't read file 'examples/parse_tree.json'"));
    let tree: Tree<MyNodeData> = serde_json::from_str(&json).unwrap();

    let drawer = SvgDrawer::new();
    let layouter = Layouter::with_tree(&tree)
        .with_drawer(&drawer)
        .with_file_name(std::path::Path::new("examples/example2.svg"));
    layouter.write().expect("Failed writing layout")
}
