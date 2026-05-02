use cc_json_parser::{handle_file, ParseOptions};

#[test]
fn valid_nested_empty() {
    assert!(handle_file("test-data/step4/valid.json", ParseOptions::unlimited()).is_ok());
}

#[test]
fn valid_nested_with_values() {
    assert!(handle_file("test-data/step4/valid2.json", ParseOptions::unlimited()).is_ok());
}

#[test]
fn invalid_single_quoted_array() {
    let result = handle_file("test-data/step4/invalid.json", ParseOptions::unlimited());
    assert!(result.is_err());
    assert_eq!(result.err().unwrap().to_string(), "Invalid Character");
}
