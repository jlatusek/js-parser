use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::PathBuf;
use thiserror::Error;
use tree_sitter::Parser;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Failed to read file: {0}")]
    FileRead(#[from] std::io::Error),
    #[error("Failed to parse JavaScript: {0}")]
    TreeSitter(String),
    #[error("Failed to set language")]
    LanguageSetup,
    #[error("Language field '{0}' not found")]
    FieldNotFound(String),
}

pub type Result<T> = std::result::Result<T, ParseError>;

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

#[derive(Serialize, Deserialize, Debug)]
pub struct JFunction {
    pub identifier: String,
    pub name: String,
    pub path: String,
    pub start: SerPoint,
    pub end: SerPoint,
}

pub struct JParser {
    parser: Parser,
    function_kind_id: u16,
    name_field_id: u16,
    hasher: Sha256,
}

impl JParser {
    pub fn new() -> Result<Self> {
        let mut parser = Parser::new();
        let language = &tree_sitter_javascript::LANGUAGE.into();
        parser
            .set_language(&language)
            .map_err(|_| ParseError::LanguageSetup)?;

        let function_kind_id = language.id_for_node_kind("function_declaration", true);
        let name_field_id = language
            .field_id_for_name("name")
            .ok_or_else(|| ParseError::FieldNotFound("name".to_string()))?;

        Ok(Self {
            parser,
            function_kind_id,
            name_field_id: name_field_id.into(),
            hasher: Sha256::new(),
        })
    }

    pub fn parse_file(&mut self, path: &PathBuf) -> Result<Vec<JFunction>> {
        let source = fs::read_to_string(&path)?;

        let tree = self
            .parser
            .parse(&source, None)
            .ok_or_else(|| ParseError::TreeSitter("Error during parsing a tree".to_string()))?;
        let root_node = tree.root_node();

        let mut cursor = root_node.walk();

        let mut js_functions = Vec::new();

        for child in root_node.named_children(&mut cursor) {
            if child.kind_id() == self.function_kind_id {
                if let Some(function) = self.create_js_function(&child, &source, path) {
                    js_functions.push(function);
                }
            }
        }

        Ok(js_functions)
    }

    fn create_js_function(
        &mut self,
        node: &tree_sitter::Node,
        source: &str,
        path: &PathBuf,
    ) -> Option<JFunction> {
        let name = self.get_function_name(node, source)?;

        let hash = self.generate_source_hash(node, &source)?;
        let identifier = format!("{}::{}::{}", path.display(), name, hash);

        Some(JFunction {
            identifier,
            name,
            path: path.display().to_string(),
            start: node.start_position().into(),
            end: node.end_position().into(),
        })
    }

    fn generate_source_hash(&mut self, node: &tree_sitter::Node, source: &str) -> Option<String> {
        let function_content = node.utf8_text(source.as_bytes()).ok()?;

        // Reset and reuse the existing hasher
        self.hasher.reset();
        self.hasher.update(function_content.as_bytes());
        let hash_result = self.hasher.finalize_reset();
        Some(format!("{:x}", hash_result).to_string())
    }

    fn get_function_name(&self, node: &tree_sitter::Node, source: &str) -> Option<String> {
        if let Some(ident) = node.child_by_field_id(self.name_field_id) {
            return ident
                .utf8_text(source.as_bytes())
                .ok()
                .map(|s| s.to_string());
        }
        None
    }
}
