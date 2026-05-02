use crate::data::{JsonData, Token};
use std::collections::VecDeque;
use std::io::Error;

use super::options::ParseOptions;
use super::parser::Parser;
use super::value::parse_element;

pub fn parse_with_options(
    tokens: VecDeque<Token>,
    options: ParseOptions,
) -> Result<JsonData, Error> {
    let mut parser = Parser::new(tokens, options.max_depth);

    if parser.is_empty() {
        return Err(Error::new(std::io::ErrorKind::InvalidData, "Empty Json"));
    }

    if let Some(element) = parse_element(&mut parser)? {
        if parser.is_empty() {
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
