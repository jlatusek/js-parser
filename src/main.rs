use std::env;
use std::path;
use walkdir::WalkDir;

pub mod file_search;

// Why we are using here PathBuf instead of Path???
fn list_files(directory: &str) -> Vec<path::PathBuf> {
    WalkDir::new(directory)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_type().is_file()
                && e.path().extension().and_then(|ext| ext.to_str()) == Some("js")
        })
        .map(|e| e.into_path()) // owned PathBuf
        .collect()
}

fn main() {
    let paths = list_files(
        env::args()
            .skip(1)
            .next()
            .expect("Not enough arguments")
            .as_str(),
    );
    for path in paths {
        println!("{}", path.display());
    }
}
