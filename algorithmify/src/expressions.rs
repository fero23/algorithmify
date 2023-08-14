use std::collections::HashSet;

use crate::interpreter::context::Context;

pub use self::{
    float::Float, functions::Function, integer::Integer, operation::Operation,
    reference::Reference, statements::Statement,
};
use anyhow::anyhow;

pub mod float;
pub mod functions;
pub mod integer;
pub mod operation;
pub mod reference;
pub mod statements;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Unit,
    Vector(Vec<Expression>),
    Reference(Reference),
    Integer(Integer),
    Float(Float),
    Char(char),
    String(String),
    Operation(Box<Operation>),
}

impl Expression {
    pub fn execute(&self, context: &Context) -> anyhow::Result<Expression> {
        let mut result = self.clone();
        result.try_replace_references(context)?;

        match result {
            Self::Operation(operation) => operation.execute(),
            _ => Ok(result),
        }
    }

    fn try_replace_references(&mut self, context: &Context) -> anyhow::Result<()> {
        if let Expression::Vector(_) | Expression::Reference(_) | Expression::Operation(_) = self {
            let references = self.get_reference_set();

            for reference in references {
                let result = context.search_reference(&reference);
                if let Some(expression) = result {
                    self.replace(&reference, expression);
                } else {
                    return Err(anyhow!("Unknown reference {}", reference));
                }
            }
        }

        Ok(())
    }

    fn get_reference_set(&self) -> HashSet<Reference> {
        let mut set = HashSet::new();
        self.add_to_reference_set(&mut set);
        set
    }

    fn add_to_reference_set(&self, set: &mut HashSet<Reference>) {
        match self {
            Self::Reference(reference) => {
                set.insert(reference.clone());
            }
            Self::Operation(operation) => operation.add_to_reference_set(set),
            _ => {}
        }
    }

    fn replace(&mut self, reference: &Reference, value: &Expression) {
        match self {
            Self::Reference(r) if r == reference => *self = value.clone(),
            Self::Operation(operation) => operation.replace(reference, value),
            _ => {}
        }
    }

    fn add(&self, rhs: &Expression) -> anyhow::Result<Expression> {
        match (self, rhs) {
            (Expression::Integer(lhs), Expression::Integer(rhs)) => {
                Ok(Expression::Integer(*lhs + *rhs))
            }
            (Expression::Float(lhs), Expression::Float(rhs)) => Ok(Expression::Float(*lhs + *rhs)),
            (Expression::String(lhs), Expression::String(rhs)) => {
                Ok(Expression::String(lhs.clone() + &rhs))
            }
            (Expression::String(lhs), Expression::Char(rhs)) => {
                Ok(Expression::String(lhs.clone() + &rhs.to_string()))
            }
            (Expression::Char(lhs), Expression::String(rhs)) => {
                Ok(Expression::String(lhs.to_string() + &rhs))
            }
            _ => Err(anyhow!(
                "Unsupported addition bewtween {:?} and {:?}",
                self,
                rhs
            )),
        }
    }

    fn sub(&self, rhs: &Expression) -> anyhow::Result<Expression> {
        match (self, rhs) {
            (Expression::Integer(lhs), Expression::Integer(rhs)) => {
                Ok(Expression::Integer(*lhs - *rhs))
            }
            (Expression::Float(lhs), Expression::Float(rhs)) => Ok(Expression::Float(*lhs - *rhs)),
            _ => Err(anyhow!(
                "Unsupported substraction bewtween {:?} and {:?}",
                self,
                rhs
            )),
        }
    }

    fn mul(&self, rhs: &Expression) -> anyhow::Result<Expression> {
        match (self, rhs) {
            (Expression::Integer(lhs), Expression::Integer(rhs)) => {
                Ok(Expression::Integer(*lhs * *rhs))
            }
            (Expression::Float(lhs), Expression::Float(rhs)) => Ok(Expression::Float(*lhs * *rhs)),
            _ => Err(anyhow!(
                "Unsupported multiplication bewtween {:?} and {:?}",
                self,
                rhs
            )),
        }
    }

    fn div(&self, rhs: &Expression) -> anyhow::Result<Expression> {
        match (self, rhs) {
            (Expression::Integer(lhs), Expression::Integer(rhs)) => {
                Ok(Expression::Integer(*lhs / *rhs))
            }
            (Expression::Float(lhs), Expression::Float(rhs)) => Ok(Expression::Float(*lhs / *rhs)),
            _ => Err(anyhow!(
                "Unsupported division bewtween {:?} and {:?}",
                self,
                rhs
            )),
        }
    }

    fn bitand(&self, rhs: &Expression) -> anyhow::Result<Expression> {
        match (self, rhs) {
            (Expression::Integer(lhs), Expression::Integer(rhs)) => {
                Ok(Expression::Integer(*lhs & *rhs))
            }
            _ => Err(anyhow!(
                "Unsupported bitwise AND bewtween {:?} and {:?}",
                self,
                rhs
            )),
        }
    }

    fn bitor(&self, rhs: &Expression) -> anyhow::Result<Expression> {
        match (self, rhs) {
            (Expression::Integer(lhs), Expression::Integer(rhs)) => {
                Ok(Expression::Integer(*lhs | *rhs))
            }
            _ => Err(anyhow!(
                "Unsupported bitwise OR bewtween {:?} and {:?}",
                self,
                rhs
            )),
        }
    }
}
