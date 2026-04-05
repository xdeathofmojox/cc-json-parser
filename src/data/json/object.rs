use crate::data::JsonMember;

#[derive(PartialEq, Eq, Debug)]
pub struct Object {
    pub members: Vec<JsonMember>,
}
