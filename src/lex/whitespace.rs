use super::json::Chars;

pub fn lex_whitespace(chars: &mut Chars) -> bool {
    let mut found = false;
    while matches!(chars.peek(), Some(' ' | '\n' | '\t' | '\r')) {
        chars.next();
        found = true;
    }
    found
}
