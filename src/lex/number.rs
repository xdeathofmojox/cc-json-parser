use crate::data::Token;
use std::io::Error;

use super::lexer::Lexer;

pub fn lex_number(lexer: &mut Lexer) -> Result<Option<Token>, Error> {
    match lexer.peek() {
        Some('-') => {
            lexer.consume();
            Ok(Some(Token::SignNeg))
        }
        Some('+') => {
            lexer.consume();
            Ok(Some(Token::SignPos))
        }
        Some('0'..='9') => {
            let dig = lexer.consume().unwrap().to_digit(10).unwrap();
            Ok(Some(Token::Digit(dig as u8)))
        }
        Some('.') => {
            lexer.consume();
            Ok(Some(Token::FractionMarker))
        }
        Some('e' | 'E') => {
            lexer.consume();
            Ok(Some(Token::ExponentMarker))
        }
        _ => Ok(None),
    }
}
