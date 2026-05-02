use crate::data::{JsonString, Token};
use std::collections::VecDeque;
use std::io::Error;

pub fn parse_string(tokens: &mut VecDeque<Token>) -> Result<Option<JsonString>, Error> {
    if let Some(&Token::String(_)) = tokens.front() {
        if let Token::String(string) = tokens.pop_front().unwrap() {
            return Ok(Some(JsonString { string }));
        } else {
            return Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "String not parsed correctly",
            ));
        }
    }

    Ok(None)
}
