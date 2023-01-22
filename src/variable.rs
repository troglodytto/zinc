use crate::literal::Literal;

#[derive(Debug, Hash)]
pub struct Variable {
    pub name: String,
    pub value: Literal,
}

#[derive(Debug)]
pub struct Context {
    variables: Vec<Variable>,
}
