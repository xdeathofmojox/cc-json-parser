use crate::data::Token;
use std::io::Error;

use super::lexer::Lexer;

pub fn lex_true(lexer: &mut Lexer) -> Result<Option<Token>, Error> {
    lex_keyword(lexer, "true", Token::True)
}

pub fn lex_false(lexer: &mut Lexer) -> Result<Option<Token>, Error> {
    lex_keyword(lexer, "false", Token::False)
}

pub fn lex_null(lexer: &mut Lexer) -> Result<Option<Token>, Error> {
    lex_keyword(lexer, "null", Token::Null)
}

fn lex_keyword(lexer: &mut Lexer, keyword: &str, token: Token) -> Result<Option<Token>, Error> {
    if lexer.peek() != keyword.chars().next() {
        return Ok(None);
    }
    for expected in keyword.chars() {
        match lexer.consume() {
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
