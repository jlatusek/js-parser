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
    for path in paths {
        println!("{}", path.display());
        parser::JSParser::new().parse_file(&path);
    }
}
