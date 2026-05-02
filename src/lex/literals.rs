use crate::data::Token;
use std::io::Error;

use super::json::Chars;

pub fn lex_true(chars: &mut Chars) -> Result<Option<Token>, Error> {
    lex_keyword(chars, "true", Token::True)
}

pub fn lex_false(chars: &mut Chars) -> Result<Option<Token>, Error> {
    lex_keyword(chars, "false", Token::False)
}

pub fn lex_null(chars: &mut Chars) -> Result<Option<Token>, Error> {
    lex_keyword(chars, "null", Token::Null)
}

fn lex_keyword(chars: &mut Chars, keyword: &str, token: Token) -> Result<Option<Token>, Error> {
    if chars.peek() != keyword.chars().next().as_ref() {
        return Ok(None);
    }
    for expected in keyword.chars() {
        match chars.next() {
            Some(c) if c == expected => {}
            _ => {
                return Err(Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Invalid Character",
                ));
            }
        }
    }
    Ok(Some(token))
}
