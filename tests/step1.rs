use cc_json_parser::{json_valid, JsonData, JsonElement, JsonObject, JsonValue};

#[test]
fn valid() {
    let expected = JsonData {
        element: JsonElement {
            value: JsonValue::Object(JsonObject { members: vec![] }),
        },
    };
    let result = json_valid(&mut "{}".as_bytes());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected);
}

#[test]
fn invalid() {
    let result = json_valid(&mut "".as_bytes());
    assert!(result.is_err());
    assert_eq!(result.err().unwrap().to_string(), "Empty Json");
}
