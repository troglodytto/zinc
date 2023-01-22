use colored::Colorize;
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
    GreaterThan,
    LessThan,
    GreaterThanEqual,
    LessThanEqual,
    EqualEqual,
    Equal,
    NotEqual,
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted = match self {
            Operator::Add => "+",
            Operator::Sub => "-",
            Operator::Mul => "*",
            Operator::Div => "/",
            Operator::Mod => "%",
            Operator::And => "&&",
            Operator::Or => "||",
            Operator::GreaterThan => ">",
            Operator::LessThan => "<",
            Operator::GreaterThanEqual => ">=",
            Operator::LessThanEqual => "<=",
            Operator::NotEqual => "!=",
            Operator::EqualEqual => "==",
            Operator::Equal => "=",
        };

        write!(f, "{}", formatted.blue())
    }
}
