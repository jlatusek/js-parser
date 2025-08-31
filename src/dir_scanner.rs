use std::path::{Path, PathBuf};
use walkdir::WalkDir;

// Why we are using here PathBuf instead of Path???
pub fn list_files<P: AsRef<Path>>(directory: P, extension: &str) -> Vec<PathBuf> {
    WalkDir::new(directory)
        .into_iter()
        .filter_map(|e| match e {
            Ok(entry) => Some(entry),
            Err(err) => {
                eprintln!("Error accessing directory entry: {}", err);
                None
            }
        })
        .filter(|entry| {
            entry.file_type().is_file()
                && entry
                    .path()
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .map_or(false, |ext| ext.eq_ignore_ascii_case(extension))
        })
        .map(|e| e.into_path())
        .collect()
}
