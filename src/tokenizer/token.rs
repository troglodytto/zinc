use super::{identifier::Identifier, keyword::Keyword, literal::Literal, operator::Operator};
use std::fmt::Display;

/// Token Kinds
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    /// Parenthesis (
    LParen,

    /// Parenthesis )
    RParen,

    /// Curly Braces {
    LBrace,

    /// Curly Braces }
    RBrace,

    /// Square Braces [
    LBracket,

    /// Square Braces ]
    RBracket,

    /// Semi Colon ;
    SemiColon,

    /// Colon :
    Colon,

    /// Dot .
    Dot,

    /// Comma ,
    Comma,

    /// Whitespace \n, \t, ' '
    Whitespace {
        /// Value of the specified whitespace character
        value: char,
    },

    /// Operator
    Op(Operator),

    /// Identifier for variables, functions, classes, etc..
    Identifier(Identifier),

    /// Literal Values
    Literal(Literal),

    /// Keywords see [Keyword]
    Keyword(Keyword),

    /// Comments
    Comment(String),
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Token::LParen => "(".to_string(),
                Token::RParen => ")".to_string(),
                Token::LBrace => "{".to_string(),
                Token::RBrace => "}".to_string(),
                Token::LBracket => "[".to_string(),
                Token::RBracket => "]".to_string(),
                Token::Dot => ".".to_string(),
                Token::Comma => ",".to_string(),
                Token::Whitespace { value } => format!("{value}"),
                Token::Op(operator) => format!("{operator}"),
                Token::Identifier(identifier) => identifier.to_string(),
                Token::Literal(literal) => literal.to_string(),
                Token::Keyword(keyword) => format!("{keyword}"),
                Token::Comment(comment) => format!("//{comment}"),
                Token::SemiColon => ";".to_string(),
                Token::Colon => ":".to_string(),
            }
        )
    }
}
