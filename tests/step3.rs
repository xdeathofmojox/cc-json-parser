use cc_json_parser::{
    json_valid, JsonData, JsonElement, JsonMember, JsonNumber, JsonObject, JsonString, JsonValue,
};

#[test]
fn valid() {
    let expected = JsonData {
        element: JsonElement {
            value: JsonValue::Object(JsonObject {
                members: vec![
                    JsonMember {
                        string: JsonString { string: String::from("key1") },
                        element: JsonElement { value: JsonValue::True },
                    },
                    JsonMember {
                        string: JsonString { string: String::from("key2") },
                        element: JsonElement { value: JsonValue::False },
                    },
                    JsonMember {
                        string: JsonString { string: String::from("key3") },
                        element: JsonElement { value: JsonValue::Null },
                    },
                    JsonMember {
                        string: JsonString { string: String::from("key4") },
                        element: JsonElement {
                            value: JsonValue::String(JsonString { string: String::from("value") }),
                        },
                    },
                    JsonMember {
                        string: JsonString { string: String::from("key5") },
                        element: JsonElement {
                            value: JsonValue::Number(JsonNumber {
                                integer: 101,
                                fraction: None,
                                exponent: None,
                            }),
                        },
                    },
                ],
            }),
        },
    };
    let input = r#"
{
  "key1": true,
  "key2": false,
  "key3": null,
  "key4": "value",
  "key5": 101
}"#;
    let result = json_valid(&mut input.as_bytes());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected);
}

#[test]
fn invalid_capitalized_bool() {
    let input = r#"
{
  "key1": true,
  "key2": False,
  "key3": null,
  "key4": "value",
  "key5": 101
}"#;
    let result = json_valid(&mut input.as_bytes());
    assert!(result.is_err());
    assert_eq!(result.err().unwrap().to_string(), "Invalid Character");
}
