use crate::data::Token;
use std::io::Error;

pub fn lex_string(string: &mut &str) -> Result<Option<Token>, Error> {
    if string.starts_with('"') {
        *string = &string[1..];
        let mut new_string = String::new();
        while !string.starts_with('"') {
            if string.starts_with('\\') {
                *string = &string[1..];
                new_string.push_str(lex_escape(string)?.as_str());
            } else {
                let new_char = string.chars().next().unwrap();
                match new_char {
                    '\u{0020}'..='\u{10FFFF}' => {
                        new_string.push(new_char);
                        *string = &string[1..];
                    }
                    _ => {
                        return Err(Error::new(
                            std::io::ErrorKind::InvalidData,
                            "Invalid Unicode Character in String",
                        ));
                    }
                }
            }
        }
        *string = &string[1..];
        return Ok(Some(Token::String(new_string)));
    }
    Ok(None)
}

fn lex_escape(string: &mut &str) -> Result<String, Error> {
    if string.starts_with('"') {
        *string = &string[1..];
        Ok(String::from("\""))
    } else if string.starts_with('\\') {
        *string = &string[1..];
        Ok(String::from("\\"))
    } else if string.starts_with('/') {
        *string = &string[1..];
        Ok(String::from("/"))
    } else if string.starts_with('b') {
        *string = &string[1..];
        Ok(String::from("\\b"))
    } else if string.starts_with('f') {
        *string = &string[1..];
        Ok(String::from("\\f"))
    } else if string.starts_with('n') {
        *string = &string[1..];
        Ok(String::from("\n"))
    } else if string.starts_with('r') {
        *string = &string[1..];
        Ok(String::from("\r"))
    } else if string.starts_with('t') {
        *string = &string[1..];
        Ok(String::from("\t"))
    } else if string.starts_with('u') {
        *string = &string[1..];
        Ok(lex_escape_hex(string)?)
    } else {
        Err(Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid Escape Character",
        ))
    }
}

fn lex_escape_hex(string: &mut &str) -> Result<String, Error> {
    if string.len() < 4 {
        Err(Error::new(
            std::io::ErrorKind::InvalidData,
            "Not enough hex characters in escape",
        ))
    } else {
        let mut new_string = String::from("\\u");
        let mut chars = string.chars();
        for _ in 0..4 {
            let new_char = chars.next().unwrap();
            match new_char {
                '0'..='9' | 'a'..='f' | 'A'..='F' => {
                    new_string.push(new_char);
                }
                _ => {
                    return Err(Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Invalid Escape Hex Character",
                    ));
                }
            }
        }
        *string = &string[4..];
        Ok(new_string)
    }
}
