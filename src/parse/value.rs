use crate::data::{JsonElement, JsonValue, Token};
use std::io::Error;

use super::array::parse_array;
use super::number::parse_number;
use super::object::parse_object;
use super::parser::Parser;
use super::string::parse_string;

pub fn parse_element(parser: &mut Parser) -> Result<Option<JsonElement>, Error> {
    parse_value(parser).map(|v| v.map(|value| JsonElement { value }))
}

fn parse_value(parser: &mut Parser) -> Result<Option<JsonValue>, Error> {
    if let Some(&Token::True) = parser.peek() {
        parser.consume();
        Ok(Some(JsonValue::True))
    } else if let Some(&Token::False) = parser.peek() {
        parser.consume();
        Ok(Some(JsonValue::False))
    } else if let Some(&Token::Null) = parser.peek() {
        parser.consume();
        Ok(Some(JsonValue::Null))
    } else if let Some(object) = parse_object(parser)? {
        Ok(Some(JsonValue::Object(object)))
    } else if let Some(array) = parse_array(parser)? {
        Ok(Some(JsonValue::Array(array)))
    } else if let Some(string) = parse_string(parser)? {
        Ok(Some(JsonValue::String(string)))
    } else if let Some(number) = parse_number(parser)? {
        Ok(Some(JsonValue::Number(number)))
    } else {
        Ok(None)
    }
}
