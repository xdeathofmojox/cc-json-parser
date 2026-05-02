use crate::data::JsonData;
use crate::lex;
use crate::parse::{self, ParseOptions};
use std::io::{BufRead, Error};

pub fn json_valid<R: BufRead>(reader: &mut R, options: ParseOptions) -> Result<JsonData, Error> {
    let mut s = String::new();
    reader.read_to_string(&mut s)?;
    let tokens = lex::lex(&s)?;
    let json_data = parse::parse_with_options(tokens, options)?;
    Ok(json_data)
}
