use crate::data::Token;
use std::io::Error;

pub fn lex_true(string: &mut &str) -> Result<Option<Token>, Error> {
    if string.starts_with("true") {
        *string = &string[4..];
        return Ok(Some(Token::True));
    }
    Ok(None)
}

pub fn lex_false(string: &mut &str) -> Result<Option<Token>, Error> {
    if string.starts_with("false") {
        *string = &string[5..];
        return Ok(Some(Token::False));
    }
    Ok(None)
}

pub fn lex_null(string: &mut &str) -> Result<Option<Token>, Error> {
    if string.starts_with("null") {
        *string = &string[4..];
        return Ok(Some(Token::Null));
    }
    Ok(None)
}
