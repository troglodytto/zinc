pub(crate) mod identifier;
pub(crate) mod keyword;
pub(crate) mod literal;
pub(crate) mod operator;
pub(crate) mod token;

use anyhow::Result;
use identifier::Identifier;
use operator::Operator;
use regex::Regex;
use std::{
    fmt::Display,
    iter::{Filter, Peekable},
    str::Chars,
    vec::IntoIter,
};
use token::Token;

/// A tokenizer implementation that uses Simple Pattern matching as well as regular expressions
#[derive(Debug, Default)]
pub struct Tokenizer {
    pub(crate) tokens: Vec<Token>,
}

impl Tokenizer {
    /// Parse and Create tokens from a given string
    pub fn tokenize(&mut self, source: String) -> Result<()> {
        let mut chars: Peekable<Chars> = source.chars().peekable();

        while let Some(current_char) = chars.next() {
            let next_char = chars.peek().copied();

            let is_next_eq = next_char == Some('=');

            let token = match current_char {
                '(' => Token::LParen,
                ')' => Token::RParen,
                '{' => Token::LBrace,
                '}' => Token::RBrace,
                '[' => Token::LBracket,
                ']' => Token::RBracket,
                ';' => Token::SemiColon,
                ':' => Token::Colon,
                '.' => Token::Dot,
                ',' => Token::Comma,
                '\n' | '\t' | ' ' => Token::Whitespace {
                    value: current_char,
                },
                '+' => Token::Op(Operator::Add(is_next_eq)),
                '-' => {
                    if next_char == Some('>') {
                        Token::Op(Operator::Arrow)
                    } else {
                        Token::Op(Operator::Sub(is_next_eq))
                    }
                }
                '*' => Token::Op(Operator::Mul(is_next_eq)),
                '/' => {
                    if next_char == Some('/') {
                        self.parse_comment(&mut chars)?
                    } else {
                        Token::Op(Operator::Div(is_next_eq))
                    }
                }
                '%' => Token::Op(Operator::Mod(is_next_eq)),
                '&' => Token::Op(Operator::And(is_next_eq)),
                '|' => Token::Op(Operator::Or(is_next_eq)),
                '^' => Token::Op(Operator::Xor(is_next_eq)),
                '=' => Token::Op(Operator::Equal(is_next_eq)),
                '>' => Token::Op(Operator::GreaterThan(is_next_eq)),
                '<' => Token::Op(Operator::LessThan(is_next_eq)),
                '!' => Token::Op(Operator::Not(is_next_eq)),
                _ => self.parse_non_trivial_token(current_char, &mut chars)?,
            };

            if let Token::Op(op) = &token {
                chars.next_if(|_| op.is_assignment_op() || *op == Operator::Arrow);
            }

            self.tokens.push(token);
        }

        Ok(())
    }

    fn parse_non_trivial_token(&self, unknown: char, chars: &mut Peekable<Chars>) -> Result<Token> {
        let rest = format!("{unknown}{}", chars.clone().collect::<String>());
        let string_regex = Regex::new("\"(.*?)\"")?;
        let lang_object_regex = Regex::new(r#"[A-Za-z0-9-.]+"#)?;

        if unknown == '"' {
            if let Some(string_match) = string_regex
                .captures(&rest)
                .and_then(|capture| capture.get(1))
            {
                let _ = chars.nth(string_match.len());

                return Ok(Token::Literal(literal::Literal::String(
                    string_match.as_str().to_owned(),
                )));
            }
        }

        if let Some(lang_object_match) = lang_object_regex
            .captures(&rest)
            .and_then(|capture| capture.get(0))
        {
            let length = lang_object_match.len();

            for _ in 0..length - 1 {
                chars.next();
            }

            return Ok(match lang_object_match.as_str() {
                "true" => Token::Literal(literal::Literal::Bool(true)),
                "false" => Token::Literal(literal::Literal::Bool(false)),
                "if" => Token::Keyword(keyword::Keyword::If),
                "else" => Token::Keyword(keyword::Keyword::Else),
                "func" => Token::Keyword(keyword::Keyword::Func),
                "let" => Token::Keyword(keyword::Keyword::Let),
                "mut" => Token::Keyword(keyword::Keyword::Mut),
                "ret" => Token::Keyword(keyword::Keyword::Return),
                "while" => Token::Keyword(keyword::Keyword::While),
                "for" => Token::Keyword(keyword::Keyword::For),
                "lambda" => Token::Keyword(keyword::Keyword::Lambda),
                "class" => Token::Keyword(keyword::Keyword::Class),
                lang_object => self.parse_lang_object(lang_object)?,
            });
        }

        Err(anyhow::anyhow!("Invalid token '{unknown}'",))
    }

    fn parse_lang_object(&self, lang_object: &str) -> Result<Token> {
        let identifier_regex = Regex::new(r#"^[a-zA-Z_][A-Za-z0-9_]*$"#)?;
        let integer_regex = Regex::new(r#"^[0-9-]*$"#)?;
        let float_regex = Regex::new(r#"^[0-9-.]*$"#)?;

        if integer_regex.is_match(lang_object) {
            return Ok(Token::Literal(literal::Literal::Integer(
                lang_object.parse()?,
            )));
        }

        if float_regex.is_match(lang_object) {
            return Ok(Token::Literal(literal::Literal::Float(
                lang_object.parse()?,
            )));
        }

        if identifier_regex.is_match(lang_object) {
            return Ok(Token::Identifier(Identifier::new(lang_object)));
        }

        Err(anyhow::anyhow!("Invalid token '{lang_object}'",))
    }

    fn parse_comment(&self, chars: &mut Peekable<Chars>) -> Result<Token> {
        let mut comment = String::new();

        let _ = chars.next(); // ? Remove the extra '/'

        while let Some(character) = chars.next_if(|peeked| *peeked != '\n') {
            comment.push(character);
        }

        Ok(Token::Comment(comment))
    }
}

/// Creates a bidirectional iterator over the tokens
pub trait BidirctionalIterator {
    /// The Type returned by each iteration
    type Item: Clone;

    /// Get the current element
    fn current(&mut self) -> Option<Self::Item>;

    /// Get the Next Element if available
    fn next(&mut self) -> Option<Self::Item>;

    /// Get the Previous Element if available
    fn prev(&mut self) -> Option<Self::Item>;

    /// Get the next element if available, without incrementing the iterator
    fn peek_next(&mut self) -> Option<Self::Item>;

    /// Get the previous element if available, without incrementing the iterator
    fn peek_prev(&mut self) -> Option<Self::Item>;

    /// Filter out the elements that do not return true for the given predicate
    fn filter<P>(self, predicate: P) -> Self
    where
        Self: Sized,
        P: FnMut(&Self::Item) -> bool;
}

/// Creates a bidirectional iterator over the tokens
#[derive(Debug, Clone)]
pub struct TokenIter {
    index: usize,
    tokens: Vec<Token>,
}

impl BidirctionalIterator for TokenIter {
    type Item = Token;

    fn current(&mut self) -> Option<Self::Item> {
        self.tokens.get(self.index).cloned()
    }

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.tokens.len() {
            return None;
        }

        let value = self.tokens.get(self.index).cloned();

        self.index += 1;

        value
    }

    fn prev(&mut self) -> Option<Self::Item> {
        if self.index == 0 {
            return None;
        }

        self.index -= 1;

        let value = self.tokens.get(self.index).cloned();

        value
    }

    fn peek_next(&mut self) -> Option<Self::Item> {
        self.tokens.get(self.index + 1).cloned()
    }

    fn peek_prev(&mut self) -> Option<Self::Item> {
        self.tokens.get(self.index - 1).cloned()
    }

    fn filter<P>(self, mut predicate: P) -> Self
    where
        Self: Sized,
        P: FnMut(&Self::Item) -> bool,
    {
        return TokenIter {
            index: 0,
            tokens: self
                .tokens
                .iter()
                .filter(|token| predicate(token))
                .cloned()
                .collect(),
        };
    }
}

impl From<Tokenizer> for TokenIter {
    fn from(Tokenizer { tokens }: Tokenizer) -> Self {
        TokenIter { index: 0, tokens }
    }
}

impl Display for Tokenizer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for token in &self.tokens {
            write!(f, "{}", token)?;
        }

        Ok(())
    }
}
