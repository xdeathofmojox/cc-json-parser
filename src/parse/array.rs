use crate::data::{JsonArray, JsonElement, Token};
use std::io::Error;

use super::parser::Parser;
use super::value::parse_element;

pub fn parse_array(parser: &mut Parser) -> Result<Option<JsonArray>, Error> {
    if let Some(&Token::OpenBracket) = parser.peek() {
        parser.consume();
    } else {
        return Ok(None);
    }

    let elements = parse_elements(parser)?.unwrap_or_default();

    parser.expect(&Token::CloseBracket).map_err(|_| {
        Error::new(
            std::io::ErrorKind::InvalidData,
            "No Closing Bracket on Array",
        )
    })?;

    Ok(Some(JsonArray { elements }))
}

fn parse_elements(parser: &mut Parser) -> Result<Option<Vec<JsonElement>>, Error> {
    let Some(element) = parse_element(parser)? else {
        return Ok(None);
    };

    let mut elements = vec![element];

    while let Some(&Token::Comma) = parser.peek() {
        parser.consume();
        if let Some(element) = parse_element(parser)? {
            elements.push(element);
        } else {
            return Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "Failed to parse element",
            ));
        }
    }

    Ok(Some(elements))
}
