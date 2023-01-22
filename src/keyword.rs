use colored::Colorize;
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
    Return,
    If,
    Else,
    Func,
    Set,
    Print,
}

impl Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted = match self {
            Keyword::Return => "ret",
            Keyword::If => "if",
            Keyword::Else => "else",
            Keyword::Func => "func",
            Keyword::Set => "set",
            Keyword::Print => "print",
        };

        write!(f, "{}", formatted.purple())
    }
}
