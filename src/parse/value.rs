use crate::data::{JsonElement, JsonValue, Token};
use std::collections::VecDeque;
use std::io::Error;

use super::array::parse_array;
use super::number::parse_number;
use super::object::parse_object;
use super::string::parse_string;

pub fn parse_element(tokens: &mut VecDeque<Token>) -> Result<Option<JsonElement>, Error> {
    parse_value(tokens).map(|v| v.map(|value| JsonElement { value }))
}

fn parse_value(tokens: &mut VecDeque<Token>) -> Result<Option<JsonValue>, Error> {
    if let Some(&Token::True) = tokens.front() {
        tokens.pop_front();
        Ok(Some(JsonValue::True))
    } else if let Some(&Token::False) = tokens.front() {
        tokens.pop_front();
        Ok(Some(JsonValue::False))
    } else if let Some(&Token::Null) = tokens.front() {
        tokens.pop_front();
        Ok(Some(JsonValue::Null))
    } else if let Some(object) = parse_object(tokens)? {
        Ok(Some(JsonValue::Object(object)))
    } else if let Some(array) = parse_array(tokens)? {
        Ok(Some(JsonValue::Array(array)))
    } else if let Some(string) = parse_string(tokens)? {
        Ok(Some(JsonValue::String(string)))
    } else if let Some(number) = parse_number(tokens)? {
        Ok(Some(JsonValue::Number(number)))
    } else {
        Ok(None)
    }
}
