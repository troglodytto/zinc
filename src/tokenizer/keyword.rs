use colored::Colorize;
use std::fmt::Display;

/// Keywords used in Zinc
#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
    /// Return
    Return,

    /// If
    If,

    /// Else
    Else,

    /// Func
    Func,

    /// Let
    Let,

    /// Mut
    Mut,

    /// While
    While,

    /// For
    For,

    /// Lambda
    Lambda,

    /// Class
    Class,
}

impl Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted = match self {
            Keyword::Return => "ret",
            Keyword::If => "if",
            Keyword::Else => "else",
            Keyword::Func => "func",
            Keyword::Let => "let",
            Keyword::Mut => "mut",
            Keyword::While => "while",
            Keyword::For => "for",
            Keyword::Lambda => "lambda",
            Keyword::Class => "class",
        };

        write!(f, "{}", formatted.purple())
    }
}
