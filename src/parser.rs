use std::fs;
use std::path::PathBuf;
use tree_sitter::{InputEdit, Language, Parser, Point};

pub struct JSParser {
    parser: Parser,
}

impl JSParser {
    pub fn new() -> Self {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_javascript::LANGUAGE.into())
            .expect("Error loading JavaScript grammar");
        Self { parser }
    }

    pub fn parse_file(&mut self, path: &PathBuf) {
        let source = fs::read_to_string(&path).expect("Cannot read file :((");
        let tree = self.parser.parse(source, None).unwrap();
        let root_node = tree.root_node();
        println!("xx {}", root_node.kind());
        let mut cursor = root_node.walk();
        for child in root_node.children(&mut cursor) {
            println!(" - child: kind={} range={:?}", child.kind(), child.range());
        }
    }
}
