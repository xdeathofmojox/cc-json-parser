use crate::data::JsonMember;

#[derive(PartialEq, Eq, Debug)]
pub struct JsonObject {
    pub members: Vec<JsonMember>,
}
