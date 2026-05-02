use cc_json_parser::handle_file;

#[test]
fn valid() {
    assert!(handle_file("test-data/step2/valid.json").is_ok());
}

#[test]
fn valid_multiple_members() {
    assert!(handle_file("test-data/step2/valid2.json").is_ok());
}

#[test]
fn invalid_trailing_comma() {
    let result = handle_file("test-data/step2/invalid.json");
    assert!(result.is_err());
    assert_eq!(result.err().unwrap().to_string(), "Failed to parse members");
}

#[test]
fn invalid_unquoted_key() {
    let result = handle_file("test-data/step2/invalid2.json");
    assert!(result.is_err());
    assert_eq!(result.err().unwrap().to_string(), "Invalid Character");
}
