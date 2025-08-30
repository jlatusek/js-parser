use std::path::PathBuf;
use walkdir::WalkDir;

// Why we are using here PathBuf instead of Path???
pub fn list_files(directory: &str) -> Vec<PathBuf> {
    WalkDir::new(directory)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_type().is_file()
                && e.path().extension().and_then(|ext| ext.to_str()) == Some("js")
        })
        .map(|e| e.into_path())
        .collect()
}
