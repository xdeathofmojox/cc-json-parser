use crate::data::{JsonMember, JsonObject, Token};
use std::collections::VecDeque;
use std::io::Error;

use super::string::parse_string;
use super::value::parse_element;

pub fn parse_object(tokens: &mut VecDeque<Token>) -> Result<Option<JsonObject>, Error> {
    if let Some(&Token::OpenParen) = tokens.front() {
        tokens.pop_front();
    } else {
        return Ok(None);
    }

    let members = parse_members(tokens)?.unwrap_or_default();

    if let Some(&Token::CloseParen) = tokens.front() {
        tokens.pop_front();
    } else {
        return Err(Error::new(
            std::io::ErrorKind::InvalidData,
            "No Closing Paren on Object",
        ));
    }

    Ok(Some(JsonObject { members }))
}

fn parse_members(tokens: &mut VecDeque<Token>) -> Result<Option<Vec<JsonMember>>, Error> {
    if let Some(member) = parse_member(tokens)? {
        let mut members = vec![member];

        while let Some(&Token::Comma) = tokens.front() {
            tokens.pop_front();
            if let Some(member) = parse_member(tokens)? {
                members.push(member);
            } else {
                return Err(Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Failed to parse members",
                ));
            }
        }

        Ok(Some(members))
    } else {
        Ok(None)
    }
}

fn parse_member(tokens: &mut VecDeque<Token>) -> Result<Option<JsonMember>, Error> {
    let Some(string) = parse_string(tokens)? else {
        return Ok(None);
    };

    if let Some(&Token::Colon) = tokens.front() {
        tokens.pop_front();
    } else {
        return Err(Error::new(
            std::io::ErrorKind::InvalidData,
            "No colon in member",
        ));
    }

    if let Some(element) = parse_element(tokens)? {
        Ok(Some(JsonMember { string, element }))
    } else {
        Err(Error::new(
            std::io::ErrorKind::InvalidData,
            "No element for string",
        ))
    }
}
