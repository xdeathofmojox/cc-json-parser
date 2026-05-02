use crate::data::Token;
use std::io::Error;

use super::json::Chars;

pub fn lex_open_paren(chars: &mut Chars) -> Result<Option<Token>, Error> {
    if chars.peek() == Some(&'{') {
        chars.next();
        return Ok(Some(Token::OpenParen));
    }
    Ok(None)
}

pub fn lex_close_paren(chars: &mut Chars) -> Result<Option<Token>, Error> {
    if chars.peek() == Some(&'}') {
        chars.next();
        return Ok(Some(Token::CloseParen));
    }
    Ok(None)
}

pub fn lex_open_bracket(chars: &mut Chars) -> Result<Option<Token>, Error> {
    if chars.peek() == Some(&'[') {
        chars.next();
        return Ok(Some(Token::OpenBracket));
    }
    Ok(None)
}

pub fn lex_close_bracket(chars: &mut Chars) -> Result<Option<Token>, Error> {
    if chars.peek() == Some(&']') {
        chars.next();
        return Ok(Some(Token::CloseBracket));
    }
    Ok(None)
}

pub fn lex_comma(chars: &mut Chars) -> Result<Option<Token>, Error> {
    if chars.peek() == Some(&',') {
        chars.next();
        return Ok(Some(Token::Comma));
    }
    Ok(None)
}

pub fn lex_colon(chars: &mut Chars) -> Result<Option<Token>, Error> {
    if chars.peek() == Some(&':') {
        chars.next();
        return Ok(Some(Token::Colon));
    }
    Ok(None)
}
