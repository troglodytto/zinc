use std::fmt::Display;

use colored::Colorize;

/// Operators used in Zinc
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Operator {
    /// Addition +
    Add(bool),

    /// Subtraction -
    Sub(bool),

    /// Multiplication *
    Mul(bool),

    /// Division /
    Div(bool),

    /// Modulus %
    Mod(bool),

    /// And &
    And(bool),

    /// Or |
    Or(bool),

    /// Xor ^
    Xor(bool),

    /// Equal =
    Equal(bool),

    /// Greater than >
    GreaterThan(bool),

    /// Less than <
    LessThan(bool),

    /// Not !
    Not(bool),

    /// Arrow ->
    Arrow,
}

impl Operator {
    pub fn is_assignment_op(&self) -> bool {
        use Operator::*;
        match self {
            Arrow => false,
            Add(is_assignment)
            | Sub(is_assignment)
            | Mul(is_assignment)
            | Div(is_assignment)
            | Mod(is_assignment)
            | And(is_assignment)
            | Or(is_assignment)
            | Xor(is_assignment)
            | Equal(is_assignment)
            | GreaterThan(is_assignment)
            | LessThan(is_assignment)
            | Not(is_assignment) => *is_assignment,
        }
    }
}

impl Display for Operator {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let operator = match self {
            Operator::Arrow => "->".to_string(),
            Operator::Add(_) => "+".to_string(),
            Operator::Sub(_) => "-".to_string(),
            Operator::Mul(_) => "*".to_string(),
            Operator::Div(_) => "/".to_string(),
            Operator::Mod(_) => "%".to_string(),
            Operator::And(_) => "&".to_string(),
            Operator::Or(_) => "|".to_string(),
            Operator::Xor(_) => "^".to_string(),
            Operator::Equal(_) => "=".to_string(),
            Operator::GreaterThan(_) => ">".to_string(),
            Operator::LessThan(_) => "<".to_string(),
            Operator::Not(_) => "!".to_string(),
        };

        let assignment_op = if self.is_assignment_op() { "=" } else { "" };

        let formatted = format!("{operator}{assignment_op}").bright_cyan();

        write!(formatter, "{formatted}",)
    }
}
