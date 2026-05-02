use crate::data::Token;
use std::collections::VecDeque;
use std::io::Error;

use super::literals::{lex_false, lex_null, lex_true};
use super::number::lex_number;
use super::punctuation::{
    lex_close_bracket, lex_close_paren, lex_colon, lex_comma, lex_open_bracket, lex_open_paren,
};
use super::string::lex_string;
use super::whitespace::lex_whitespace;

pub type Chars<'a> = std::iter::Peekable<std::str::Chars<'a>>;

pub fn lex(input: &str) -> Result<VecDeque<Token>, Error> {
    let mut chars = input.chars().peekable();
    let mut result: VecDeque<Token> = VecDeque::new();

    while chars.peek().is_some() {
        if let Some(token) = lex_open_paren(&mut chars)? {
            result.push_back(token);
        } else if let Some(token) = lex_close_paren(&mut chars)? {
            result.push_back(token);
        } else if let Some(token) = lex_open_bracket(&mut chars)? {
            result.push_back(token);
        } else if let Some(token) = lex_close_bracket(&mut chars)? {
            result.push_back(token);
        } else if let Some(token) = lex_comma(&mut chars)? {
            result.push_back(token);
        } else if let Some(token) = lex_colon(&mut chars)? {
            result.push_back(token);
        } else if let Some(token) = lex_string(&mut chars)? {
            result.push_back(token);
        } else if lex_whitespace(&mut chars) {
            // whitespace is skipped
        } else if let Some(token) = lex_number(&mut chars)? {
            result.push_back(token);
        } else if let Some(token) = lex_true(&mut chars)? {
            result.push_back(token);
        } else if let Some(token) = lex_false(&mut chars)? {
            result.push_back(token);
        } else if let Some(token) = lex_null(&mut chars)? {
            result.push_back(token);
        } else {
            return Err(Error::other("Invalid Character"));
        }
    }

    Ok(result)
}
