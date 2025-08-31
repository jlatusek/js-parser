use crate::parser::JSFunction;
use dir_scanner::list_files;
use serde_json;
use std::env;

pub mod dir_scanner;
pub mod parser;

fn main() {
    let directory = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: js_parser <directory>");
        std::process::exit(1);
    });

    let paths = list_files(directory, "js");

    // TODO: Add parallel processing of JS files
    let js_functions: Vec<JSFunction> = paths
        .into_iter()
        .flat_map(|path| parser::JSParser::new().parse_file(&path))
        .collect();

    let json = serde_json::to_string_pretty(&js_functions).unwrap_or_else(|e| {
        eprintln!("Error serializing to JSON: {}", e);
        std::process::exit(1);
    });
    println!("{}", json);
}
