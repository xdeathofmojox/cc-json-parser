use crate::data::Token;
use std::io::Error;

pub fn lex_open_paren(string: &mut &str) -> Result<Option<Token>, Error> {
    if string.starts_with('{') {
        *string = &string[1..];
        return Ok(Some(Token::OpenParen));
    }
    Ok(None)
}

pub fn lex_close_paren(string: &mut &str) -> Result<Option<Token>, Error> {
    if string.starts_with('}') {
        *string = &string[1..];
        return Ok(Some(Token::CloseParen));
    }
    Ok(None)
}

pub fn lex_open_bracket(string: &mut &str) -> Result<Option<Token>, Error> {
    if string.starts_with('[') {
        *string = &string[1..];
        return Ok(Some(Token::OpenBracket));
    }
    Ok(None)
}

pub fn lex_close_bracket(string: &mut &str) -> Result<Option<Token>, Error> {
    if string.starts_with(']') {
        *string = &string[1..];
        return Ok(Some(Token::CloseBracket));
    }
    Ok(None)
}

pub fn lex_comma(string: &mut &str) -> Result<Option<Token>, Error> {
    if string.starts_with(',') {
        *string = &string[1..];
        return Ok(Some(Token::Comma));
    }
    Ok(None)
}

pub fn lex_colon(string: &mut &str) -> Result<Option<Token>, Error> {
    if string.starts_with(':') {
        *string = &string[1..];
        return Ok(Some(Token::Colon));
    }
    Ok(None)
}
