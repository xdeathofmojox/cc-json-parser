use crate::data::{JsonData, Token};
use std::collections::VecDeque;
use std::io::Error;

use super::value::parse_element;

pub fn parse(tokens: &mut VecDeque<Token>) -> Result<JsonData, Error> {
    if tokens.is_empty() {
        return Err(Error::new(std::io::ErrorKind::InvalidData, "Empty Json"));
    }

    if let Some(element) = parse_element(tokens)? {
        if tokens.is_empty() {
            Ok(JsonData { element })
        } else {
            Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid Json: Additional Data Left Over",
            ))
        }
    } else {
        Err(Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid Json: No Element",
        ))
    }
}
