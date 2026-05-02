use crate::data::{JsonNumber, Token};
use std::collections::VecDeque;
use std::io::Error;

pub fn parse_number(tokens: &mut VecDeque<Token>) -> Result<Option<JsonNumber>, Error> {
    if let Some(integer) = parse_integer(tokens)? {
        let fraction = parse_fraction(tokens)?;
        let exponent = parse_exponent(tokens)?;
        Ok(Some(JsonNumber {
            integer,
            fraction,
            exponent,
        }))
    } else {
        Ok(None)
    }
}

fn parse_integer(tokens: &mut VecDeque<Token>) -> Result<Option<i64>, Error> {
    let mut neg_sign = false;
    let mut found_nums = false;
    let mut int_value: i64 = 0;
    let mut chars_found: usize = 0;

    if let Some(&Token::SignNeg) = tokens.front() {
        tokens.pop_front();
        neg_sign = true;
    }

    while let Some(&Token::Digit(val)) = tokens.front() {
        tokens.pop_front();
        found_nums = true;
        int_value *= 10;
        int_value += val as i64;
        chars_found += 1;
    }

    if neg_sign && !found_nums {
        Err(Error::new(
            std::io::ErrorKind::InvalidData,
            "No digits following sign",
        ))
    } else if found_nums {
        if int_value.to_string().len() < chars_found {
            return Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "No leading zeros allowed",
            ));
        }
        if neg_sign {
            int_value = -int_value;
        }
        Ok(Some(int_value))
    } else {
        Ok(None)
    }
}

fn parse_fraction(tokens: &mut VecDeque<Token>) -> Result<Option<u64>, Error> {
    if let Some(&Token::FractionMarker) = tokens.front() {
        tokens.pop_front();
        let mut fraction_found = false;
        let mut fraction_value: u64 = 0;

        while let Some(&Token::Digit(val)) = tokens.front() {
            tokens.pop_front();
            fraction_value *= 10;
            fraction_value += val as u64;
            fraction_found = true;
        }

        if fraction_found {
            Ok(Some(fraction_value))
        } else {
            Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "No fraction component after fraction marker",
            ))
        }
    } else {
        Ok(None)
    }
}

fn parse_exponent(tokens: &mut VecDeque<Token>) -> Result<Option<i64>, Error> {
    if let Some(&Token::ExponentMarker) = tokens.front() {
        tokens.pop_front();
        let mut found_exponent = false;
        let mut exponent_value: i64 = 0;
        let mut found_sign = false;
        let mut neg_sign = false;

        if let Some(&Token::SignNeg) = tokens.front() {
            tokens.pop_front();
            found_sign = true;
            neg_sign = true;
        } else if let Some(&Token::SignPos) = tokens.front() {
            tokens.pop_front();
            found_sign = true;
        }

        while let Some(&Token::Digit(val)) = tokens.front() {
            tokens.pop_front();
            exponent_value *= 10;
            exponent_value += val as i64;
            found_exponent = true;
        }

        if found_sign && !found_exponent {
            Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "No digits following sign",
            ))
        } else if found_exponent {
            if neg_sign {
                exponent_value = -exponent_value;
            }
            Ok(Some(exponent_value))
        } else {
            Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "No digits following exponent",
            ))
        }
    } else {
        Ok(None)
    }
}
