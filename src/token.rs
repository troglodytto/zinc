use colored::Colorize;
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Literal {
    String(String),
    Number(u32),
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted = match self {
            Literal::String(value) => format!("\"{value}\""),
            Literal::Number(value) => format!("{}", value),
        };

        write!(f, "{}", formatted.bright_yellow())
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
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
            Token::Whitespace { value } => format!("{}", value),
            Token::Identifier(identifier) => format!("{}", identifier.red()),
            Token::Literal(literal) => format!("{literal}"),
            Token::Keyword(keyword) => format!("{keyword}"),
        };

        write!(f, "{}", formatted)
    }
}
