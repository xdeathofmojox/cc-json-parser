use crate::data::Token;
use std::collections::VecDeque;
use std::io::Error;

pub struct Parser {
    tokens: VecDeque<Token>,
    depth: usize,
    max_depth: Option<usize>,
}

impl Parser {
    pub fn new(tokens: VecDeque<Token>, max_depth: Option<usize>) -> Self {
        Self { tokens, depth: 0, max_depth }
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

    pub fn enter_depth(&mut self) -> Result<(), Error> {
        self.depth += 1;
        if let Some(max) = self.max_depth {
            if self.depth > max {
                return Err(Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("Exceeded maximum nesting depth of {}", max),
                ));
            }
        }
        Ok(())
    }

    pub fn exit_depth(&mut self) {
        self.depth = self.depth.saturating_sub(1);
    }
}
