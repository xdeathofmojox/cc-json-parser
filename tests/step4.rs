use cc_json_parser::{
    json_valid, JsonArray, JsonData, JsonElement, JsonMember, JsonNumber, JsonObject, JsonString,
    JsonValue,
};

#[test]
fn valid_nested_empty() {
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
                        string: JsonString { string: String::from("key-n") },
                        element: JsonElement {
                            value: JsonValue::Number(JsonNumber {
                                integer: 101,
                                fraction: None,
                                exponent: None,
                            }),
                        },
                    },
                    JsonMember {
                        string: JsonString { string: String::from("key-o") },
                        element: JsonElement {
                            value: JsonValue::Object(JsonObject { members: vec![] }),
                        },
                    },
                    JsonMember {
                        string: JsonString { string: String::from("key-l") },
                        element: JsonElement {
                            value: JsonValue::Array(JsonArray { elements: vec![] }),
                        },
                    },
                ],
            }),
        },
    };
    let input = r#"
{
  "key": "value",
  "key-n": 101,
  "key-o": {},
  "key-l": []
}"#;
    let result = json_valid(&mut input.as_bytes());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected);
}

#[test]
fn valid_nested_with_values() {
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
                        string: JsonString { string: String::from("key-n") },
                        element: JsonElement {
                            value: JsonValue::Number(JsonNumber {
                                integer: 101,
                                fraction: None,
                                exponent: None,
                            }),
                        },
                    },
                    JsonMember {
                        string: JsonString { string: String::from("key-o") },
                        element: JsonElement {
                            value: JsonValue::Object(JsonObject {
                                members: vec![JsonMember {
                                    string: JsonString { string: String::from("inner key") },
                                    element: JsonElement {
                                        value: JsonValue::String(JsonString {
                                            string: String::from("inner value"),
                                        }),
                                    },
                                }],
                            }),
                        },
                    },
                    JsonMember {
                        string: JsonString { string: String::from("key-l") },
                        element: JsonElement {
                            value: JsonValue::Array(JsonArray {
                                elements: vec![JsonElement {
                                    value: JsonValue::String(JsonString {
                                        string: String::from("list value"),
                                    }),
                                }],
                            }),
                        },
                    },
                ],
            }),
        },
    };
    let input = r#"
{
  "key": "value",
  "key-n": 101,
  "key-o": {
    "inner key": "inner value"
  },
  "key-l": ["list value"]
}"#;
    let result = json_valid(&mut input.as_bytes());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected);
}

#[test]
fn invalid_single_quoted_array() {
    let input = r#"
{
  "key": "value",
  "key-n": 101,
  "key-o": {
    "inner key": "inner value"
  },
  "key-l": ['list value']
}"#;
    let result = json_valid(&mut input.as_bytes());
    assert!(result.is_err());
    assert_eq!(result.err().unwrap().to_string(), "Invalid Character");
}
