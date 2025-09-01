use js_parser::dir_scanner::list_files;
use js_parser::parser::{JFunction, JParser};
use std::collections::HashSet;
use std::path::PathBuf;

#[test]
fn test_parse_all_js_files_in_test_input() {
    let test_input_dir = "tests/input";

    let expected_names_template = [
        "generateRandomString",
        "shuffleArray",
        "generateRandomData",
        "processData",
        "main",
    ];
    let mut js_files = list_files(test_input_dir, "js");

    let mut parser = JParser::new().expect("Failed to create parser");
    js_files.sort();

    for (i, file) in js_files.into_iter().enumerate() {
        let mut functions: Vec<String> = parser
            .parse_file(&file)
            .unwrap_or_else(|e| panic!("Failed to parse file {}: {}", file.display(), e))
            .iter()
            .map(|o| o.name.clone())
            .collect();

        assert_eq!(functions.len(), 5);
        let mut expected_names: Vec<String> = expected_names_template
            .into_iter()
            .map(|s| format!("{}{}", s, i + 1))
            .collect();

        functions.sort();
        expected_names.sort();

        assert_eq!(
            functions,
            expected_names,
            "Function names in {} don't match expected. Found: {:?}, Expected: {:?}",
            file.display(),
            functions,
            expected_names
        );
    }
}
