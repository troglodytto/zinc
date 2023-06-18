use crate::tokenizer::{
    identifier::{self, Identifier},
    literal::{self, Literal},
    operator::{self, Operator},
    token::Token,
    BidirctionalIterator, TokenIter,
};
use anyhow::Result;

#[derive(Debug)]
enum Expression {
    Literal(Literal),

    Reference(Identifier),

    BinaryOperator {
        left: Box<Expression>,
        right: Box<Expression>,
        operator: Operator,
    },

    UnaryOperator {
        operator: Operator,
        expression: Box<Expression>,
    },

    FunctionCall {
        identifier: Identifier,
        arguments: Vec<Expression>,
    },

    Unit,
}

impl From<Identifier> for Expression {
    fn from(value: Identifier) -> Self {
        Expression::Reference(value)
    }
}

impl From<Literal> for Expression {
    fn from(value: Literal) -> Self {
        Expression::Literal(value)
    }
}

impl From<Token> for Result<Expression> {
    fn from(value: Token) -> Self {
        match value {
            Token::Identifier(identifier) => Ok(identifier.into()),
            Token::Literal(literal) => Ok(literal.into()),
            _ => Err(anyhow::anyhow!("Invalid expression {}", line!())),
        }
    }
}

#[derive(Debug)]
struct Block {
    statements: Vec<Statement>,
    value: Expression,
}

#[derive(Debug)]
enum Statement {
    Expression(Expression),

    Assignment {
        identifier: Identifier,
        value: Expression,
        mutable: bool,
    },

    UpdateAssignment {
        identifier: Identifier,
        value: Expression,
    },

    Block(Block),

    If {
        condition: Expression,
        then_block: Block,
        else_block: Option<Block>,
    },

    While {
        condition: Expression,
        block: Block,
    },
}

#[derive(Debug)]
/// Abstract Syntax Tree Generator
pub struct Wayfarer {
    index: usize,
    tokens: TokenIter,
    root_node: Block,
}

impl Wayfarer {
    /// Create a new [Wayfarer] instance
    pub fn new<T: Into<TokenIter>>(tokenizer: T) -> Wayfarer {
        Wayfarer {
            index: 0,
            root_node: Block {
                statements: vec![],
                value: Expression::Unit,
            },
            tokens: tokenizer
                .into()
                .filter(|token| !matches!(token, Token::Comment(_) | Token::Whitespace { .. })),
        }
    }

    /// Generate the AST using the given tokens
    pub fn generate(&mut self) -> Result<()> {
        while let Some(token) = self.tokens.next() {
            if let Some(Token::Op(operator)) = self.tokens.peek_next() {
                let expression = self.parse_operator(operator)?;
                self.root_node
                    .statements
                    .push(Statement::Expression(expression));
            }
        }

        dbg!(&self.root_node);

        Ok(())
    }

    fn parse_operator(&mut self, operator: Operator) -> Result<Expression> {
        self.tokens.next();
        let left = self.tokens.peek_prev();
        let right = self.tokens.peek_next();

        match (left, right) {
            (None, None) => Err(anyhow::anyhow!("Invalid expression {}", line!())),
            (None, Some(token)) | (Some(token), None) => {
                let expression: Result<Expression> = token.into();

                Ok(Expression::UnaryOperator {
                    operator,
                    expression: Box::new(expression?),
                })
            }
            (Some(left), Some(right)) => {
                let left_expression: Result<Expression> = left.into();
                let right_expression: Result<Expression> = right.into();

                Ok(Expression::BinaryOperator {
                    left: Box::new(left_expression?),
                    right: Box::new(right_expression?),
                    operator,
                })
            }
        }
    }
}
