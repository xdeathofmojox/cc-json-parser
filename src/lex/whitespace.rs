use crate::data::Token;
use std::io::Error;

pub fn lex_whitespace(string: &mut &str) -> Result<Option<Token>, Error> {
    let mut found_whitespace = false;
    while string.starts_with(' ')
        || string.starts_with('\n')
        || string.starts_with('\t')
        || string.starts_with('\r')
    {
        *string = &string[1..];
        found_whitespace = true;
    }

    if found_whitespace {
        return Ok(Some(Token::Whitespace));
    }
    Ok(None)
}
