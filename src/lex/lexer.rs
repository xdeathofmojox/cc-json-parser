use std::iter::Peekable;
use std::str::Chars;

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
        }
    }

    pub fn peek(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    pub fn consume(&mut self) -> Option<char> {
        self.chars.next()
    }

    pub fn consume_if(&mut self, expected: char) -> bool {
        if self.chars.peek() == Some(&expected) {
            self.chars.next();
            true
        } else {
            false
        }
    }

    pub fn is_empty(&mut self) -> bool {
        self.chars.peek().is_none()
    }
}
