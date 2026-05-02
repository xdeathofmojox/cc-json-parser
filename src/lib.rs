mod data;
mod lex;
mod parse;

pub use data::{
    JsonArray, JsonData, JsonElement, JsonMember, JsonNumber, JsonObject, JsonString, JsonValue,
};

use std::fs::File;
use std::io::{BufRead, BufReader, Error};

pub fn json_valid<R: BufRead>(reader: &mut R) -> Result<JsonData, Error> {
    let mut s = String::new();
    reader.read_to_string(&mut s)?;
    let mut tokens = lex::lex(&mut s.as_str())?;
    let json_data = parse::parse(&mut tokens)?;
    Ok(json_data)
}

pub fn handle_file(filename: &str) -> Result<JsonData, Error> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);
    json_valid(&mut reader)
}
