use js_parser::dir_scanner::*;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use tempfile::TempDir;

fn create_js_tree() -> (TempDir, Vec<PathBuf>) {
    let td = TempDir::new().expect("make temp dir");

    // Layout:
    // <tmp>/
    //   a/
    //     app.js
    //     nested/
    //       util.js
    //   b/
    //     index.js
    let a = td.path().join("a");
    let nested = a.join("nested");
    let b = td.path().join("b");

    fs::create_dir_all(&nested).unwrap();
    fs::create_dir_all(&b).unwrap();

    let files = vec![a.join("app.js"), nested.join("util.js"), b.join("index.js")];

    for path in files.iter() {
        write_js_hello(path);
    }

    (td, files)
}

fn write_js_hello(path: &Path) {
    let mut f = fs::File::create(path).expect("Cannot create file");
    f.write("function hello() { console.log('hello from file '); }".as_bytes())
        .expect("Cannot write js content");
}

#[test]
fn test_list_files() {
    let (td, mut expected_files) = create_js_tree();
    expected_files.sort();
    let mut found_files = list_files(
        td.path()
            .as_os_str()
            .to_str()
            .expect("Cannot change tmpdir to string"),
    );
    found_files.sort();
    assert!(expected_files == found_files);
}
