use crate::data::Token;
use std::collections::VecDeque;
use std::io::Error;

pub struct Parser {
    tokens: VecDeque<Token>,
}

impl Parser {
    pub fn new(tokens: VecDeque<Token>) -> Self {
        Self { tokens }
    }

    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.front()
    }

    pub fn consume(&mut self) -> Option<Token> {
        self.tokens.pop_front()
    }

    pub fn expect(&mut self, expected: &Token) -> Result<(), Error> {
        match self.tokens.front() {
            Some(t) if std::mem::discriminant(t) == std::mem::discriminant(expected) => {
                self.tokens.pop_front();
                Ok(())
            }
            _ => Err(Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Expected {:?}", expected),
            )),
        }
    }
}
