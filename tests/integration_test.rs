use js_parser::dir_scanner::list_files;
use js_parser::parser::JParser;

#[test]
fn test_parse_all_js_files_in_test_input() {
    let test_input_dir = "tests/input/testcase_1";

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

#[test]
fn test_function_structure() {
    let test_input_dir = "tests/input/testcase_1";
    let js_files = list_files(test_input_dir, "js");

    let mut parser = JParser::new().expect("Failed to create parser");

    // Test with just the first file
    let first_file = &js_files[0];
    let functions = parser.parse_file(first_file).expect("Failed to parse file");

    for function in &functions {
        let parts: Vec<&str> = function.identifier.split("::").collect();
        assert_eq!(
            parts.len(),
            3,
            "Identifier should have 3 parts separated by '::': {}",
            function.identifier
        );

        assert_eq!(
            parts[1], function.name,
            "Second part of identifier should match function name"
        );

        assert!(
            parts[2].chars().all(|c| c.is_ascii_hexdigit()),
            "Hash part should be valid hex: {}",
            parts[2]
        );

        assert!(!function.path.is_empty(), "Path should not be empty");

        assert!(
            !function.name.is_empty(),
            "Function name should not be empty"
        );

        assert!(
            function.start.row <= function.end.row,
            "Start row should be <= end row"
        );
        if function.start.row == function.end.row {
            assert!(
                function.start.column < function.end.column,
                "If same row, start column should be < end column"
            );
        }
    }
}

#[test]
fn test_only_js_files_found() {
    let test_input_dir = "tests/input/testcase1";
    let js_files = list_files(test_input_dir, "js");

    for file_path in &js_files {
        let extension = file_path.extension().unwrap().to_string_lossy();
        assert_eq!(
            extension.to_lowercase(),
            "js",
            "Found non-JS file: {:?}",
            file_path
        );
    }
}
