use crate::data::JsonElement;

#[derive(PartialEq, Eq, Debug)]
pub struct Array {
    pub elements: Vec<JsonElement>,
}
