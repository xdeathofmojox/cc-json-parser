use crate::data::{JsonArray, JsonElement, Token};
use std::collections::VecDeque;
use std::io::Error;

use super::value::parse_element;

pub fn parse_array(tokens: &mut VecDeque<Token>) -> Result<Option<JsonArray>, Error> {
    if let Some(&Token::OpenBracket) = tokens.front() {
        tokens.pop_front();
    } else {
        return Ok(None);
    }

    let elements = parse_elements(tokens)?.unwrap_or_default();

    if let Some(&Token::CloseBracket) = tokens.front() {
        tokens.pop_front();
    } else {
        return Err(Error::new(
            std::io::ErrorKind::InvalidData,
            "No Closing Bracket on Array",
        ));
    }

    Ok(Some(JsonArray { elements }))
}

fn parse_elements(tokens: &mut VecDeque<Token>) -> Result<Option<Vec<JsonElement>>, Error> {
    if let Some(element) = parse_element(tokens)? {
        let mut elements = vec![element];

        while let Some(&Token::Comma) = tokens.front() {
            tokens.pop_front();
            if let Some(element) = parse_element(tokens)? {
                elements.push(element);
            } else {
                return Err(Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Failed to parse element",
                ));
            }
        }

        Ok(Some(elements))
    } else {
        Ok(None)
    }
}
