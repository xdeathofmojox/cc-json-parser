use crate::data::{JsonElement, JsonString};

#[derive(PartialEq, Eq, Debug)]
pub struct Member {
    pub string: JsonString,
    pub element: JsonElement,
}
