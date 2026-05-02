use crate::data::{JsonString, Token};
use std::io::Error;

use super::parser::Parser;

pub fn parse_string(parser: &mut Parser) -> Result<Option<JsonString>, Error> {
    if let Some(&Token::String(_)) = parser.peek() {
        if let Some(Token::String(string)) = parser.consume() {
            return Ok(Some(JsonString { string }));
        }
    }
    Ok(None)
}
