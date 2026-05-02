use crate::data::Token;
use std::io::Error;

use super::json::Chars;

pub fn lex_string(chars: &mut Chars) -> Result<Option<Token>, Error> {
    if chars.peek() != Some(&'"') {
        return Ok(None);
    }
    chars.next();

    let mut s = String::new();
    loop {
        match chars.peek() {
            None => {
                return Err(Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Unterminated string",
                ));
            }
            Some(&'"') => {
                chars.next();
                break;
            }
            Some(&'\\') => {
                chars.next();
                s.push_str(&lex_escape(chars)?);
            }
            Some(&c) if c >= '\u{0020}' => {
                s.push(c);
                chars.next();
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

fn lex_escape(chars: &mut Chars) -> Result<String, Error> {
    match chars.next() {
        Some('"') => Ok(String::from("\"")),
        Some('\\') => Ok(String::from("\\")),
        Some('/') => Ok(String::from("/")),
        Some('b') => Ok(String::from("\\b")),
        Some('f') => Ok(String::from("\\f")),
        Some('n') => Ok(String::from("\n")),
        Some('r') => Ok(String::from("\r")),
        Some('t') => Ok(String::from("\t")),
        Some('u') => lex_escape_hex(chars),
        _ => Err(Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid Escape Character",
        )),
    }
}

fn lex_escape_hex(chars: &mut Chars) -> Result<String, Error> {
    let mut s = String::from("\\u");
    for _ in 0..4 {
        match chars.next() {
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
