use std::{
    io::{Error, ErrorKind},
    process,
};

use crate::{
    expression::Expression, keyword::Keyword, token::Token, variable::Context, wayfarer::Wayfarer,
};

#[derive(Debug)]
pub enum Statement {
    Complex(Vec<Statement>),
    Assignment(String, Expression),
    Print(Expression),
    If {
        condition: Expression,
        on_true: Vec<Statement>,
        on_false: Option<Vec<Statement>>,
    },
}

pub struct Function {
    identifier: String,
    content: Vec<Statement>,
    context: Vec<Context>,
}

impl Function {
    // fn evaluate(&self) {}
}

trait AstParser {
    fn parse(tokens: Vec<Token>) -> Result<AbstractSyntaxTree, ()>;
}

pub struct AbstractSyntaxTree {
    functions: Vec<Function>,
}

impl AbstractSyntaxTree {
    /// # Errors
    /// # Panics
    /// This is a top level statement
    /// The rest of the parsing will be done on an atomic level
    /// Global Context will parse functions
    /// Functions will parse statements,
    /// Statements will parse expressions and so on
    pub fn parse(wayfarer: &mut Wayfarer) -> Result<AbstractSyntaxTree, Error> {
        let mut paren_stack: Vec<Token> = Vec::new();

        let functions = Vec::new();

        for token in wayfarer {
            match token {
                Token::LParen => paren_stack.push(token),
                Token::RParen if paren_stack.last() == Some(&Token::LParen) => {
                    paren_stack.pop();
                }
                Token::Keyword(Keyword::Func) => {
                    todo!()
                }
                Token::RParen if paren_stack.last() == Some(&Token::RParen) => {
                    println!("Invalid token");
                    return Err(Error::from(ErrorKind::InvalidInput));
                }
                _ => {
                    println!("Invalid token");
                    return Err(Error::from(ErrorKind::InvalidInput));
                }
            };
        }

        Ok(AbstractSyntaxTree { functions })
    }
}
