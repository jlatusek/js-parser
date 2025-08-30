use std::env;

use crate::parser::JSFunction;

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
        // println!("{}", path.display());
        js_functions.append(&mut parser::JSParser::new().parse_file(&path));
    }

    for function in js_functions {
        println!("{} name={}", function.identifier, function.name);
    }
}
