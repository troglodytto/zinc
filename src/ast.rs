// Function {
//    Statement { Expression }
// }

use crate::token::Token;

pub struct Expression {
    tokens: Vec<Token>,
}

impl Expression {
    fn new(tokens: Vec<Token>) -> Expression {
        Expression { tokens }
    }

    fn evaluate(&self) {}
}

pub enum Statement {
    Expression(Expression),
    Assignment(String, Expression),
}

pub struct Function {
    identifier: String,
    content: Vec<Statement>,
}

pub struct SyntaxTree {
    functions: Vec<Function>,
}

impl SyntaxTree {
    fn parse(tokens: Vec<Token>) -> Result<SyntaxTree, ()> {
        Err(())
    }
}
