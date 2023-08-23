use std::collections::HashSet;

use crate::interpreter::context::Context;

use self::{
    block::Block,
    functions::{FunctionArgs, FunctionName},
};
pub use self::{
    conditions::Condition, float::Float, functions::Function, integer::Integer, loops::Loop,
    operation::Operation, reference::Reference, statements::Statement,
};
use anyhow::anyhow;

pub mod block;
pub mod conditions;
pub mod float;
pub mod functions;
pub mod integer;
pub mod loops;
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
    Bool(bool),
    Operation(Box<Operation>),
    Condition(Box<Condition>),
    Loop(Box<Loop>),
    FunctionCall(FunctionName, FunctionArgs),
    Block(Box<Block>),
}

impl Expression {
    pub fn execute(&self, context: &mut Context) -> anyhow::Result<Expression> {
        let mut result = self.clone();
        result.try_replace_references(context)?;

        match result {
            Self::Operation(operation) => operation.execute(),
            Self::Loop(loop_instance) => loop_instance.execute(context),
            Self::Condition(condition) => condition.execute(context),
            Self::Block(block) => block.execute(context),
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

    fn try_resolve_inner(&self) -> Expression {
        if let Expression::Operation(operation) = self {
            if let Ok(resolution) = operation.execute() {
                return resolution;
            }
        }

        self.clone()
    }

    fn as_boolean(&self) -> Option<bool> {
        match self {
            Self::Bool(bool) => Some(*bool),
            _ => None,
        }
    }

    fn add(self, rhs: Expression) -> anyhow::Result<Expression> {
        match (self, rhs) {
            (Expression::Integer(lhs), Expression::Integer(rhs)) => {
                Ok(Expression::Integer(lhs + rhs))
            }
            (Expression::Float(lhs), Expression::Float(rhs)) => Ok(Expression::Float(lhs + rhs)),
            (Expression::String(lhs), Expression::String(rhs)) => {
                Ok(Expression::String(lhs.clone() + &rhs))
            }
            (Expression::String(lhs), Expression::Char(rhs)) => {
                Ok(Expression::String(lhs.clone() + &rhs.to_string()))
            }
            (Expression::Char(lhs), Expression::String(rhs)) => {
                Ok(Expression::String(lhs.to_string() + &rhs))
            }
            (lhs, rhs) => Err(anyhow!(
                "Unsupported addition between {:?} and {:?}",
                lhs,
                rhs
            )),
        }
    }

    fn sub(self, rhs: Expression) -> anyhow::Result<Expression> {
        match (self, rhs) {
            (Expression::Integer(lhs), Expression::Integer(rhs)) => {
                Ok(Expression::Integer(lhs - rhs))
            }
            (Expression::Float(lhs), Expression::Float(rhs)) => Ok(Expression::Float(lhs - rhs)),
            (lhs, rhs) => Err(anyhow!(
                "Unsupported substraction between {:?} and {:?}",
                lhs,
                rhs
            )),
        }
    }

    fn mul(self, rhs: Expression) -> anyhow::Result<Expression> {
        match (self, rhs) {
            (Expression::Integer(lhs), Expression::Integer(rhs)) => {
                Ok(Expression::Integer(lhs * rhs))
            }
            (Expression::Float(lhs), Expression::Float(rhs)) => Ok(Expression::Float(lhs * rhs)),
            (lhs, rhs) => Err(anyhow!(
                "Unsupported multiplication between {:?} and {:?}",
                lhs,
                rhs
            )),
        }
    }

    fn div(self, rhs: Expression) -> anyhow::Result<Expression> {
        match (self, rhs) {
            (Expression::Integer(lhs), Expression::Integer(rhs)) => {
                Ok(Expression::Integer(lhs / rhs))
            }
            (Expression::Float(lhs), Expression::Float(rhs)) => Ok(Expression::Float(lhs / rhs)),
            (lhs, rhs) => Err(anyhow!(
                "Unsupported division between {:?} and {:?}",
                lhs,
                rhs
            )),
        }
    }

    fn bitand(self, rhs: Expression) -> anyhow::Result<Expression> {
        match (self, rhs) {
            (Expression::Integer(lhs), Expression::Integer(rhs)) => {
                Ok(Expression::Integer(lhs & rhs))
            }
            (lhs, rhs) => Err(anyhow!(
                "Unsupported bitwise AND between {:?} and {:?}",
                lhs,
                rhs
            )),
        }
    }

    fn bitor(self, rhs: Expression) -> anyhow::Result<Expression> {
        match (self, rhs) {
            (Expression::Integer(lhs), Expression::Integer(rhs)) => {
                Ok(Expression::Integer(lhs | rhs))
            }
            (lhs, rhs) => Err(anyhow!(
                "Unsupported bitwise OR between {:?} and {:?}",
                lhs,
                rhs
            )),
        }
    }

    fn and(self, rhs: Expression) -> anyhow::Result<Expression> {
        match (self, rhs) {
            (Expression::Bool(lhs), Expression::Bool(rhs)) => Ok(Expression::Bool(lhs && rhs)),
            (lhs, rhs) => Err(anyhow!("Unsupported AND between {:?} and {:?}", lhs, rhs)),
        }
    }

    fn or(self, rhs: Expression) -> anyhow::Result<Expression> {
        match (self, rhs) {
            (Expression::Bool(lhs), Expression::Bool(rhs)) => Ok(Expression::Bool(lhs || rhs)),
            (lhs, rhs) => Err(anyhow!("Unsupported OR between {:?} and {:?}", lhs, rhs)),
        }
    }

    fn eq(&self, rhs: &Expression) -> anyhow::Result<Expression> {
        match (self, rhs) {
            (Expression::Integer(lhs), Expression::Integer(rhs)) => {
                Ok(Expression::Bool(*lhs == *rhs))
            }
            (Expression::Float(lhs), Expression::Float(rhs)) => Ok(Expression::Bool(*lhs == *rhs)),
            (Expression::Bool(lhs), Expression::Bool(rhs)) => Ok(Expression::Bool(*lhs == *rhs)),
            (lhs, rhs) => Err(anyhow!(
                "Unsupported equals between {:?} and {:?}",
                lhs,
                rhs
            )),
        }
    }

    fn ne(&self, rhs: &Expression) -> anyhow::Result<Expression> {
        self.eq(rhs)
            .map(|result| Expression::Bool(!result.as_boolean().unwrap()))
            .map_err(|_| anyhow!("Unsupported not equals between {:?} and {:?}", self, rhs))
    }

    fn lt(&self, rhs: &Expression) -> anyhow::Result<Expression> {
        match (self, rhs) {
            (Expression::Integer(lhs), Expression::Integer(rhs)) => {
                Ok(Expression::Bool(*lhs < *rhs))
            }
            (Expression::Float(lhs), Expression::Float(rhs)) => Ok(Expression::Bool(*lhs < *rhs)),
            (Expression::Bool(lhs), Expression::Bool(rhs)) => Ok(Expression::Bool(*lhs < *rhs)),
            (lhs, rhs) => Err(anyhow!(
                "Unsupported less than between {:?} and {:?}",
                lhs,
                rhs
            )),
        }
    }

    fn lte(&self, rhs: &Expression) -> anyhow::Result<Expression> {
        self.eq(rhs)
            .and_then(|eq| {
                self.lt(rhs).map(|lt| {
                    Expression::Bool(eq.as_boolean().unwrap() || lt.as_boolean().unwrap())
                })
            })
            .map_err(|_| {
                anyhow!(
                    "Unsupported less than equals between {:?} and {:?}",
                    self,
                    rhs
                )
            })
    }

    fn gt(&self, rhs: &Expression) -> anyhow::Result<Expression> {
        self.eq(rhs)
            .and_then(|eq| {
                self.lt(rhs).map(|lt| {
                    Expression::Bool(!eq.as_boolean().unwrap() && !lt.as_boolean().unwrap())
                })
            })
            .map_err(|_| anyhow!("Unsupported greater than between {:?} and {:?}", self, rhs))
    }

    fn gte(&self, rhs: &Expression) -> anyhow::Result<Expression> {
        self.lt(rhs)
            .map(|result| Expression::Bool(!result.as_boolean().unwrap()))
            .map_err(|_| {
                anyhow!(
                    "Unsupported greater than equals between {:?} and {:?}",
                    self,
                    rhs
                )
            })
    }
}
