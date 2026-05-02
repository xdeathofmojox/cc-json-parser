mod data;
mod file_handler;
mod lex;
mod parse;
mod validation;

pub use data::{
    JsonArray, JsonData, JsonElement, JsonMember, JsonNumber, JsonObject, JsonString, JsonValue,
};
pub use file_handler::handle_file;
pub use validation::json_valid;
