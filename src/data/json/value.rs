use crate::data::{JsonArray, JsonNumber, JsonObject, JsonString};

#[derive(PartialEq, Eq, Debug)]
pub enum JsonValue {
    Object(JsonObject),
    Array(JsonArray),
    String(JsonString),
    Number(JsonNumber),
    True,
    False,
    Null,
}
