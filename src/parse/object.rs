use crate::data::{JsonMember, JsonObject, Token};
use std::io::Error;

use super::parser::Parser;
use super::string::parse_string;
use super::value::parse_element;

pub fn parse_object(parser: &mut Parser) -> Result<Option<JsonObject>, Error> {
    if let Some(&Token::OpenParen) = parser.peek() {
        parser.consume();
    } else {
        return Ok(None);
    }

    let members = parse_members(parser)?.unwrap_or_default();

    parser.expect(&Token::CloseParen).map_err(|_| {
        Error::new(
            std::io::ErrorKind::InvalidData,
            "No Closing Paren on Object",
        )
    })?;

    Ok(Some(JsonObject { members }))
}

fn parse_members(parser: &mut Parser) -> Result<Option<Vec<JsonMember>>, Error> {
    let Some(member) = parse_member(parser)? else {
        return Ok(None);
    };

    let mut members = vec![member];

    while let Some(&Token::Comma) = parser.peek() {
        parser.consume();
        if let Some(member) = parse_member(parser)? {
            members.push(member);
        } else {
            return Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "Failed to parse members",
            ));
        }
    }

    Ok(Some(members))
}

fn parse_member(parser: &mut Parser) -> Result<Option<JsonMember>, Error> {
    let Some(string) = parse_string(parser)? else {
        return Ok(None);
    };

    parser
        .expect(&Token::Colon)
        .map_err(|_| Error::new(std::io::ErrorKind::InvalidData, "No colon in member"))?;

    if let Some(element) = parse_element(parser)? {
        Ok(Some(JsonMember { string, element }))
    } else {
        Err(Error::new(
            std::io::ErrorKind::InvalidData,
            "No element for string",
        ))
    }
}
