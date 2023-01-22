use crate::token::Token;
use std::iter::Iterator;

/// An iterator over Vec<Token> that can walk and peek in both directions
pub struct Wayfarer {
    index: usize,
    tokens: Vec<Token>,
}

impl Wayfarer {
    #[must_use]
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { index: 0, tokens }
    }

    pub fn peek(&mut self) -> Option<&<Wayfarer as Iterator>::Item> {
        self.tokens.get(self.index)
    }

    pub fn peek_prev(&mut self) -> Option<&<Wayfarer as Iterator>::Item> {
        self.tokens.get(self.index - 1)
    }

    pub fn prev(&mut self) -> Option<<Wayfarer as Iterator>::Item> {
        self.index -= 1;
        self.tokens.get(self.index).cloned()
    }

    pub fn step(&mut self, step: usize) -> Option<<Wayfarer as Iterator>::Item> {
        self.index += step;

        self.tokens.get(self.index - 1).cloned()
    }

    pub fn back(&mut self, step: usize) -> Option<<Wayfarer as Iterator>::Item> {
        self.index -= step;

        self.tokens.get(self.index - 1).cloned()
    }
}

impl Iterator for Wayfarer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        self.tokens.get(self.index - 1).cloned()
    }
}
