use colored::Colorize;
use std::fmt::Display;

use crate::{keyword::Keyword, literal::Literal, operator::Operator};

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Op(Operator),
    LParen,
    RParen,
    LBrace,
    RBrace,
    Dot,
    Comma,
    Whitespace { value: char },
    Identifier(String),
    Literal(Literal),
    Keyword(Keyword),
    Comment(String),
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted = match self {
            Token::Op(operator) => format!("{operator}"),
            Token::LParen => format!("{}", "(".cyan()),
            Token::RParen => format!("{}", ")".cyan()),
            Token::LBrace => format!("{}", "{".cyan()),
            Token::RBrace => format!("{}", "}".cyan()),
            Token::Dot => format!("{}", ".".cyan()),
            Token::Comma => format!("{}", ",".cyan()),
            Token::Comment(comment) => {
                format!("{}{}", ";".bright_black(), comment.bright_black())
            }
            Token::Whitespace { value } => format!("{value}"),
            Token::Identifier(identifier) => format!("{}", identifier.red()),
            Token::Literal(literal) => format!("{literal}"),
            Token::Keyword(keyword) => format!("{keyword}"),
        };

        write!(f, "{formatted}")
    }
}
