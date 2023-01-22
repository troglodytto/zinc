use colored::Colorize;
use std::{fmt::Display, process};

use crate::operator::Operator;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub enum Literal {
    String(String),
    Number(u32),
    Bool(bool),
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted = match self {
            Literal::String(value) => format!("\"{value}\""),
            Literal::Number(value) => format!("{value}"),
            Literal::Bool(value) => format!("{value}"),
        };

        write!(f, "{}", formatted.bright_yellow())
    }
}

impl Literal {
    #[must_use]
    pub fn evaluate(self, other: Self, operator: &Operator) -> Self {
        match (self, other) {
            (Literal::String(first), Literal::String(second)) => match operator {
                Operator::Add => Literal::String(format!("{first}{second}")),
                Operator::EqualEqual => Literal::Bool(first == second),
                Operator::GreaterThan => Literal::Bool(first > second),
                Operator::LessThan => Literal::Bool(first < second),
                Operator::GreaterThanEqual => Literal::Bool(first >= second),
                Operator::LessThanEqual => Literal::Bool(first <= second),
                Operator::NotEqual => Literal::Bool(first != second),
                _ => {
                    println!("Mismatched types String and String for operator {operator}");
                    process::exit(-1);
                }
            },
            (Literal::String(string), Literal::Number(number)) => {
                if let Operator::Mul = operator {
                    Literal::String(string.repeat(number as usize))
                } else {
                    println!("Mismatched types String and Number for operator {operator}");
                    process::exit(-1);
                }
            }
            (Literal::Number(first), Literal::Number(second)) => match operator {
                Operator::Add => Literal::Number(first + second),
                Operator::Sub => Literal::Number(first - second),
                Operator::Mul => Literal::Number(first * second),
                Operator::Div => Literal::Number(first / second),
                Operator::Mod => Literal::Number(first % second),
                Operator::And => Literal::Number(first & second),
                Operator::Or => Literal::Number(first | second),
                Operator::GreaterThan => Literal::Bool(first > second),
                Operator::LessThan => Literal::Bool(first < second),
                Operator::GreaterThanEqual => Literal::Bool(first >= second),
                Operator::LessThanEqual => Literal::Bool(first <= second),
                Operator::EqualEqual => Literal::Bool(first == second),
                Operator::NotEqual => Literal::Bool(first != second),
                Operator::Equal => {
                    println!("Mismatched types String and String for operator {operator}");
                    process::exit(-1);
                }
            },
            (Literal::Bool(first), Literal::Bool(second)) => match operator {
                Operator::And => Literal::Bool(first && second),
                Operator::Or => Literal::Bool(first || second),
                Operator::GreaterThanEqual => Literal::Bool(first >= second),
                Operator::LessThanEqual => Literal::Bool(first <= second),
                Operator::EqualEqual => Literal::Bool(first == second),
                Operator::NotEqual => Literal::Bool(first != second),
                _ => {
                    println!("Mismatched types Bool and Bool for operator {operator}");
                    process::exit(-1);
                }
            },
            _ => {
                println!("Invalid Expression");
                process::exit(-1);
            }
        }
    }
}
