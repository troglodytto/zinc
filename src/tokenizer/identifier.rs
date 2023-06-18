use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub struct Identifier {
    pub name: String,
}

impl Identifier {
    pub fn new<S: Into<String>>(name: S) -> Identifier {
        Identifier { name: name.into() }
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
