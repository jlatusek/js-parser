use js_parser::parser::JParser;
use std::path::PathBuf;

#[test]
fn test_identical_files_produce_the_same_identifier() {
    let mut parser = JParser::new().expect("Failed to create parser");

    let file_path = PathBuf::from("tests/input/testcase_1/a/a/file1.js");

    let mut functions1 = parser.parse_file(&file_path).expect("Failed to parse file");
    let mut functions2 = parser.parse_file(&file_path).expect("Failed to parse file");

    assert_eq!(
        functions1.len(),
        functions2.len(),
        "Both files should have the same number of functions"
    );

    functions1.sort_by(|a, b| a.identifier.cmp(&b.identifier));
    functions2.sort_by(|a, b| a.identifier.cmp(&b.identifier));

    for (f1, f2) in functions1.iter().zip(functions2.iter()) {
        assert_eq!(f1.identifier, f2.identifier, "Identifier are not the same");
    }
}
