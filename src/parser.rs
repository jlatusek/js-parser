use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tree_sitter::Parser;

pub struct JSParser {
    parser: Parser,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SerPoint {
    pub row: usize,
    pub column: usize,
}

impl From<tree_sitter::Point> for SerPoint {
    fn from(p: tree_sitter::Point) -> Self {
        SerPoint {
            row: p.row,
            column: p.column,
        }
    }
}

// TODO: difference between String and string!!!
#[derive(Serialize, Deserialize, Debug)]
pub struct JSFunction {
    pub identifier: String,
    pub name: String,
    pub path: String,
    pub start: SerPoint,
    pub end: SerPoint,
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
                    path: path.display().to_string(),
                    start: child.start_position().into(),
                    end: child.end_position().into(),
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
