use crate::{literal::Literal, operator::Operator, variable::Variable};

#[derive(Debug)]
pub enum Expression {
    Simple {
        operator: Operator,
        operands: (Literal, Literal),
    },
    Literal(Literal),
    Var(Variable),
}

impl Expression {
    fn evaluate(self) -> Literal {
        match self {
            Expression::Simple { operator, operands } => operands.0.evaluate(operands.1, &operator),
            Expression::Literal(value) => value,
            Expression::Var(variable) => variable.value,
        }
    }
}
