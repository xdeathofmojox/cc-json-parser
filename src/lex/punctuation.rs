use crate::data::Token;
use std::io::Error;

use super::lexer::Lexer;

pub fn lex_open_paren(lexer: &mut Lexer) -> Result<Option<Token>, Error> {
    Ok(lexer.consume_if('{').then_some(Token::OpenParen))
}

pub fn lex_close_paren(lexer: &mut Lexer) -> Result<Option<Token>, Error> {
    Ok(lexer.consume_if('}').then_some(Token::CloseParen))
}

pub fn lex_open_bracket(lexer: &mut Lexer) -> Result<Option<Token>, Error> {
    Ok(lexer.consume_if('[').then_some(Token::OpenBracket))
}

pub fn lex_close_bracket(lexer: &mut Lexer) -> Result<Option<Token>, Error> {
    Ok(lexer.consume_if(']').then_some(Token::CloseBracket))
}

pub fn lex_comma(lexer: &mut Lexer) -> Result<Option<Token>, Error> {
    Ok(lexer.consume_if(',').then_some(Token::Comma))
}

pub fn lex_colon(lexer: &mut Lexer) -> Result<Option<Token>, Error> {
    Ok(lexer.consume_if(':').then_some(Token::Colon))
}
