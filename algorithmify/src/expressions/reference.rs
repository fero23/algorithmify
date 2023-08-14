use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Reference {
    Variable(String),
    IndexedAccess(String, usize),
}

impl Display for Reference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Reference::Variable(variable) => write!(f, "{}", variable),
            Reference::IndexedAccess(vector, index) => write!(f, "{}[{}]", vector, index),
        }
    }
}
