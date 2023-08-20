use crate::Expression;

use super::Statement;

#[derive(Debug, Clone, PartialEq)]
pub enum Loop {
    While(WhileLoop),
}

#[derive(Debug, Clone, PartialEq)]
pub struct WhileLoop {
    statements: Vec<Statement>,
    condition: Expression,
}
