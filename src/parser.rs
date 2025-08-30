use std::fmt::format;
use std::path::PathBuf;
use std::str::FromStr;
use std::{fmt, fs, string};
use tree_sitter::{InputEdit, Language, Parser, Point};

pub struct JSParser {
    parser: Parser,
}

// TODO: difference between String and string!!!
pub struct JSFunction {
    pub identifier: String,
    pub name: String,
    pub path: PathBuf,
    pub start: Point,
    pub end: Point,
}

impl JSParser {
    pub fn new() -> Self {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_javascript::LANGUAGE.into())
            .expect("Error loading JavaScript grammar");
        Self { parser }
    }

    pub fn parse_file(&mut self, path: &PathBuf) -> Vec<JSFunction> {
        let mut js_functions: Vec<JSFunction> = Vec::new();
        let source = fs::read_to_string(&path).expect("Cannot read file :((");
        let tree = self.parser.parse(&source, None).unwrap();
        let root_node = tree.root_node();
        println!("xx {}", root_node.kind());
        let mut cursor = root_node.walk();
        for child in root_node.named_children(&mut cursor) {
            // TODO: replace string compassion with kind_id
            if child.kind() == "function_declaration" {
                let mut gname: String = String::new();
                if let Some(name) = self.extract_function_name(&child, &source) {
                    gname = name;
                }

                js_functions.push(JSFunction {
                    identifier: format!("{}:{}", path.display(), child.start_position().row + 1,),
                    name: String::from(gname),
                    // TODO: do we really need clone here?
                    path: path.clone(),
                    start: child.start_position(),
                    end: child.end_position(),
                })
            };
        }
        return js_functions;
    }

    fn extract_function_name(&self, node: &tree_sitter::Node, source: &str) -> Option<String> {
        // Get the child by its field name ("name" works in JS grammar)
        if let Some(ident) = node.child_by_field_name("name") {
            return ident
                .utf8_text(source.as_bytes())
                .ok()
                .map(|s| s.to_string());
        }
        None
    }
}
