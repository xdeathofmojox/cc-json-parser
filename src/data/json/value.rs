use crate::data::{JsonArray, JsonNumber, JsonObject, JsonString};

#[derive(PartialEq, Eq, Debug)]
pub enum Value {
    Object(JsonObject),
    Array(JsonArray),
    String(JsonString),
    Number(JsonNumber),
    True,
    False,
    Null,
}
