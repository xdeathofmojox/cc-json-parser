use super::lexer::Lexer;

pub fn lex_whitespace(lexer: &mut Lexer) -> bool {
    let mut found = false;
    while matches!(lexer.peek(), Some(' ' | '\n' | '\t' | '\r')) {
        lexer.consume();
        found = true;
    }
    found
}
