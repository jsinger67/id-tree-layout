#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use id_tree::Tree;
use id_tree_layout::{Layouter, Visualize};

#[derive(Serialize, Deserialize)]
struct MyNodeData<'a> {
    name: &'a str,
    is_terminal: bool,
}

// You need to implement id_tree_layout::Visualize for your nodes data type.
// This way you provide basic formatting information.
impl<'a> Visualize for MyNodeData<'a> {
    fn visualize(&self) -> std::string::String {
        // We simply convert the name to an owned string here.
        self.name.to_string()
    }
    fn emphasize(&self) -> bool {
        // This simply emphasizes only to leaf nodes,
        // i.e. the terminals of the parse tree.
        self.is_terminal
    }
}

fn main() {
    // Read the tree from the json export.
    let json = std::fs::read_to_string("examples/parse_tree.json")
        .unwrap_or_else(|_| panic!("Can't read file 'examples/parse_tree.json'"));
    let tree: Tree<MyNodeData> = serde_json::from_str(&json).unwrap();

    Layouter::new(&tree)
        .with_file_name(std::path::Path::new("examples/example2.svg"))
        .write()
        .expect("Failed writing layout")
}
