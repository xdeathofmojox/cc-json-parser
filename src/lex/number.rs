use crate::data::Token;
use std::io::Error;

use super::json::Chars;

pub fn lex_number(chars: &mut Chars) -> Result<Option<Token>, Error> {
    match chars.peek() {
        Some('-') => {
            chars.next();
            Ok(Some(Token::SignNeg))
        }
        Some('+') => {
            chars.next();
            Ok(Some(Token::SignPos))
        }
        Some('0'..='9') => {
            let dig = chars.next().unwrap().to_digit(10).unwrap();
            Ok(Some(Token::Digit(dig as u8)))
        }
        Some('.') => {
            chars.next();
            Ok(Some(Token::FractionMarker))
        }
        Some('e' | 'E') => {
            chars.next();
            Ok(Some(Token::ExponentMarker))
        }
        _ => Ok(None),
    }
}
