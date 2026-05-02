use crate::data::Token;
use std::io::Error;

use super::lexer::Lexer;

pub fn lex_string(lexer: &mut Lexer) -> Result<Option<Token>, Error> {
    if !lexer.consume_if('"') {
        return Ok(None);
    }

    let mut s = String::new();
    loop {
        match lexer.peek() {
            None => {
                return Err(Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Unterminated string",
                ));
            }
            Some('"') => {
                lexer.consume();
                break;
            }
            Some('\\') => {
                lexer.consume();
                s.push_str(&lex_escape(lexer)?);
            }
            Some(c) if c >= '\u{0020}' => {
                s.push(c);
                lexer.consume();
            }
            _ => {
                return Err(Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Invalid Unicode Character in String",
                ));
            }
        }
    }

    Ok(Some(Token::String(s)))
}

fn lex_escape(lexer: &mut Lexer) -> Result<String, Error> {
    match lexer.consume() {
        Some('"') => Ok(String::from("\"")),
        Some('\\') => Ok(String::from("\\")),
        Some('/') => Ok(String::from("/")),
        Some('b') => Ok(String::from("\\b")),
        Some('f') => Ok(String::from("\\f")),
        Some('n') => Ok(String::from("\n")),
        Some('r') => Ok(String::from("\r")),
        Some('t') => Ok(String::from("\t")),
        Some('u') => lex_escape_hex(lexer),
        _ => Err(Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid Escape Character",
        )),
    }
}

fn lex_escape_hex(lexer: &mut Lexer) -> Result<String, Error> {
    let mut s = String::from("\\u");
    for _ in 0..4 {
        match lexer.consume() {
            Some(c @ ('0'..='9' | 'a'..='f' | 'A'..='F')) => s.push(c),
            _ => {
                return Err(Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Invalid Escape Hex Character",
                ));
            }
        }
    }
    Ok(s)
}
