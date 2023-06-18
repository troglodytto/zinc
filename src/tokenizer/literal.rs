use colored::Colorize;
use std::fmt::Display;

/// Represents a literal value
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Literal {
    /// String Literal value
    String(String),

    /// Integer Literal value
    Integer(i32),

    /// Float Literal value
    Float(f64),

    /// Boolean Literal value
    Bool(bool),
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted = match self {
            Literal::String(value) => format!("\"{value}\""),
            Literal::Integer(value) => format!("{value}"),
            Literal::Bool(value) => format!("{value}"),
            Literal::Float(value) => format!("{value}"),
        };

        write!(f, "{}", formatted.bright_yellow())
    }
}
