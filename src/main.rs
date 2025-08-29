use std::{env, fs, path};
use tree_sitter::{InputEdit, Language, Parser, Point};

pub mod js_file;

fn parse_file(path: &path::PathBuf) {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_javascript::LANGUAGE.into())
        .expect("Error loading JavaScript grammar");
    let source = fs::read_to_string(&path).expect("Cannot read file :((");

    let tree = parser.parse(source, None).unwrap();
    let root_node = tree.root_node();
    println!("xx {}", root_node.kind());
    let mut cursor = root_node.walk();
    for child in root_node.children(&mut cursor) {
        println!(" - child: kind={} range={:?}", child.kind(), child.range());
    }
}

fn main() {
    let paths = js_file::list_files(
        env::args()
            .skip(1)
            .next()
            .expect("Not enough arguments")
            .as_str(),
    );
    for path in paths {
        println!("{}", path.display());
        parse_file(&path);
    }
}
