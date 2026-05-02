use crate::data::JsonData;
use crate::parse::ParseOptions;
use crate::validation::json_valid;
use std::fs::File;
use std::io::{BufReader, Error};

pub fn handle_file(filename: &str, options: ParseOptions) -> Result<JsonData, Error> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);
    json_valid(&mut reader, options)
}
