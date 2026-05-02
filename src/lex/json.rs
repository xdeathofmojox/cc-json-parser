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

pub fn lex(string: &mut &str) -> Result<VecDeque<Token>, Error> {
    let mut result: VecDeque<Token> = VecDeque::new();

    while !string.is_empty() {
        if let Some(token) = lex_open_paren(string)? {
            result.push_back(token);
        } else if let Some(token) = lex_close_paren(string)? {
            result.push_back(token);
        } else if let Some(token) = lex_open_bracket(string)? {
            result.push_back(token);
        } else if let Some(token) = lex_close_bracket(string)? {
            result.push_back(token);
        } else if let Some(token) = lex_comma(string)? {
            result.push_back(token);
        } else if let Some(token) = lex_colon(string)? {
            result.push_back(token);
        } else if let Some(token) = lex_string(string)? {
            result.push_back(token);
        } else if let Some(token) = lex_true(string)? {
            result.push_back(token);
        } else if let Some(token) = lex_false(string)? {
            result.push_back(token);
        } else if let Some(token) = lex_null(string)? {
            result.push_back(token);
        } else if lex_whitespace(string)?.is_some() {
            // whitespace is skipped
        } else if let Some(token) = lex_number(string)? {
            result.push_back(token);
        } else {
            return Err(Error::other("Invalid Character"));
        }
    }

    Ok(result)
}
