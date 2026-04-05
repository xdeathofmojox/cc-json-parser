use crate::data::JsonElement;

#[derive(PartialEq, Eq, Debug)]
pub struct JsonArray {
    pub elements: Vec<JsonElement>,
}
