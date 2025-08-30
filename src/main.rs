use crate::parser::JSFunction;
use serde_json;
use std::env;

pub mod js_file;
pub mod parser;

fn main() {
    let paths = js_file::list_files(
        env::args()
            .skip(1)
            .next()
            .expect("Not enough arguments")
            .as_str(),
    );
    let mut js_functions: Vec<JSFunction> = Vec::new();
    for path in paths {
        js_functions.append(&mut parser::JSParser::new().parse_file(&path));
    }

    let json = serde_json::to_string_pretty(&js_functions).expect("Error to json");
    println!("{}", json);
}
