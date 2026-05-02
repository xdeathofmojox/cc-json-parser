use cc_json_parser::{handle_file, JsonData, JsonElement, JsonObject, JsonValue, ParseOptions};

#[test]
fn valid() {
    let expected = JsonData {
        element: JsonElement {
            value: JsonValue::Object(JsonObject { members: vec![] }),
        },
    };
    let result = handle_file("test-data/step1/valid.json", ParseOptions::unlimited());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected);
}

#[test]
fn invalid() {
    let result = handle_file("test-data/step1/invalid.json", ParseOptions::unlimited());
    assert!(result.is_err());
    assert_eq!(result.err().unwrap().to_string(), "Empty Json");
}
