use crate::data::Token;
use std::io::Error;

pub fn lex_number(string: &mut &str) -> Result<Option<Token>, Error> {
    let char = string.chars().next();
    match char {
        Some('-') => {
            *string = &string[1..];
            Ok(Some(Token::SignNeg))
        }
        Some('+') => {
            *string = &string[1..];
            Ok(Some(Token::SignPos))
        }
        Some('0'..='9') => {
            *string = &string[1..];
            let dig = char.unwrap().to_digit(10).unwrap();
            Ok(Some(Token::Digit(dig as u8)))
        }
        Some('.') => {
            *string = &string[1..];
            Ok(Some(Token::FractionMarker))
        }
        Some('e') | Some('E') => {
            *string = &string[1..];
            Ok(Some(Token::ExponentMarker))
        }
        _ => Ok(None),
    }
}
