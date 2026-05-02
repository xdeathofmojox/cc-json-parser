use cc_json_parser::{
    json_valid, JsonArray, JsonData, JsonElement, JsonMember, JsonNumber, JsonObject, JsonString,
    JsonValue,
};

#[test]
fn valid_number_formats() {
    let expected = JsonData {
        element: JsonElement {
            value: JsonValue::Object(JsonObject {
                members: vec![
                    JsonMember {
                        string: JsonString { string: String::from("key1") },
                        element: JsonElement {
                            value: JsonValue::Number(JsonNumber { integer: 100, fraction: None, exponent: None }),
                        },
                    },
                    JsonMember {
                        string: JsonString { string: String::from("key2") },
                        element: JsonElement {
                            value: JsonValue::Number(JsonNumber { integer: 100, fraction: Some(0), exponent: None }),
                        },
                    },
                    JsonMember {
                        string: JsonString { string: String::from("key3") },
                        element: JsonElement {
                            value: JsonValue::Number(JsonNumber { integer: 100, fraction: Some(0), exponent: Some(10) }),
                        },
                    },
                    JsonMember {
                        string: JsonString { string: String::from("key4") },
                        element: JsonElement {
                            value: JsonValue::Number(JsonNumber { integer: 100, fraction: Some(0), exponent: Some(-10) }),
                        },
                    },
                    JsonMember {
                        string: JsonString { string: String::from("key5") },
                        element: JsonElement {
                            value: JsonValue::Number(JsonNumber { integer: 100, fraction: Some(0), exponent: Some(10) }),
                        },
                    },
                    JsonMember {
                        string: JsonString { string: String::from("key6") },
                        element: JsonElement {
                            value: JsonValue::Number(JsonNumber { integer: 999, fraction: Some(9999), exponent: Some(999) }),
                        },
                    },
                    JsonMember {
                        string: JsonString { string: String::from("key7") },
                        element: JsonElement {
                            value: JsonValue::Number(JsonNumber { integer: 0, fraction: Some(0), exponent: Some(0) }),
                        },
                    },
                    JsonMember {
                        string: JsonString { string: String::from("key8") },
                        element: JsonElement {
                            value: JsonValue::Number(JsonNumber { integer: 100, fraction: None, exponent: Some(8) }),
                        },
                    },
                    JsonMember {
                        string: JsonString { string: String::from("key9") },
                        element: JsonElement {
                            value: JsonValue::Number(JsonNumber {
                                integer: 9_223_372_036_854_775_807i64,
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
    "key1": 100,
    "key2": 100.0,
    "key3": 100.0e+10,
    "key4": 100.0e-10,
    "key5": 100.0e10,
    "key6": 999.9999e999,
    "key7": 0.0E0,
    "key8": 100e8,
    "key9": 9223372036854775807
}"#;
    let result = json_valid(&mut input.as_bytes());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected);
}

#[test]
fn valid_array_of_literals() {
    let expected = JsonData {
        element: JsonElement {
            value: JsonValue::Array(JsonArray {
                elements: vec![
                    JsonElement { value: JsonValue::True },
                    JsonElement { value: JsonValue::False },
                    JsonElement { value: JsonValue::Null },
                ],
            }),
        },
    };
    let input = r#"
[
    true,
    false,
    null
]"#;
    let result = json_valid(&mut input.as_bytes());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected);
}

#[test]
fn valid_escape_sequences() {
    let expected = JsonData {
        element: JsonElement {
            value: JsonValue::Object(JsonObject {
                members: vec![
                    JsonMember {
                        string: JsonString { string: String::from("chars") },
                        element: JsonElement {
                            value: JsonValue::String(JsonString {
                                string: String::from("\"\\/\\b\\f\n\r\t"),
                            }),
                        },
                    },
                    JsonMember {
                        string: JsonString { string: String::from("never\ngive\nup") },
                        element: JsonElement {
                            value: JsonValue::String(JsonString {
                                string: String::from("never\nsurrender"),
                            }),
                        },
                    },
                    JsonMember {
                        string: JsonString { string: String::from("hex") },
                        element: JsonElement {
                            value: JsonValue::String(JsonString {
                                string: String::from("\\u0000\\uFFFF"),
                            }),
                        },
                    },
                ],
            }),
        },
    };
    // Note: \n \r \t inside JSON strings are escape sequences the lexer processes;
    // \b \f are kept as literal \b \f; \uXXXX is kept as literal \uXXXX.
    // Regular string literal used here so \\ produces the backslashes the lexer expects.
    let input = "{\n    \"chars\": \"\\\"\\\\/\\b\\f\\n\\r\\t\",\n    \"never\\ngive\\nup\": \"never\\nsurrender\",\n    \"hex\": \"\\u0000\\uFFFF\"\n}";
    let result = json_valid(&mut input.as_bytes());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected);
}
