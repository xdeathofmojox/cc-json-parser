use cc_json_parser::{handle_file, ParseOptions};

#[test]
fn valid() {
    assert!(handle_file("test-data/step3/valid.json", ParseOptions::unlimited()).is_ok());
}

#[test]
fn invalid_capitalized_bool() {
    let result = handle_file("test-data/step3/invalid.json", ParseOptions::unlimited());
    assert!(result.is_err());
    assert_eq!(result.err().unwrap().to_string(), "Invalid Character");
}
