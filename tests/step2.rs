use cc_json_parser::{json_valid, JsonData, JsonElement, JsonMember, JsonObject, JsonString, JsonValue};

#[test]
fn valid() {
    let expected = JsonData {
        element: JsonElement {
            value: JsonValue::Object(JsonObject {
                members: vec![JsonMember {
                    string: JsonString { string: String::from("key") },
                    element: JsonElement {
                        value: JsonValue::String(JsonString { string: String::from("value") }),
                    },
                }],
            }),
        },
    };
    let result = json_valid(&mut r#"{"key": "value"}"#.as_bytes());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected);
}

#[test]
fn valid_multiple_members() {
    let expected = JsonData {
        element: JsonElement {
            value: JsonValue::Object(JsonObject {
                members: vec![
                    JsonMember {
                        string: JsonString { string: String::from("key") },
                        element: JsonElement {
                            value: JsonValue::String(JsonString { string: String::from("value") }),
                        },
                    },
                    JsonMember {
                        string: JsonString { string: String::from("key2") },
                        element: JsonElement {
                            value: JsonValue::String(JsonString { string: String::from("value") }),
                        },
                    },
                ],
            }),
        },
    };
    let input = r#"
{
  "key": "value",
  "key2": "value"
}"#;
    let result = json_valid(&mut input.as_bytes());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected);
}

#[test]
fn invalid_trailing_comma() {
    let result = json_valid(&mut r#"{"key": "value",}"#.as_bytes());
    assert!(result.is_err());
    assert_eq!(result.err().unwrap().to_string(), "Failed to parse members");
}

#[test]
fn invalid_unquoted_key() {
    let input = r#"
{
  "key": "value",
  key2: "value"
}"#;
    let result = json_valid(&mut input.as_bytes());
    assert!(result.is_err());
    assert_eq!(result.err().unwrap().to_string(), "Invalid Character");
}
