use anyhow::anyhow;
use std::fmt::Display;

use crate::{interpreter::context::Context, Expression};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Reference {
    Variable(String),
    IndexedAccess(String, usize),
}

impl Reference {
    pub(crate) fn execute(&self, context: &mut Context) -> Result<Expression, anyhow::Error> {
        if let Some(expression) = context.search_reference(self) {
            Ok(expression.clone())
        } else {
            return Err(anyhow!("Unknown reference {}", self));
        }
    }
}

impl Display for Reference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Reference::Variable(variable) => write!(f, "{}", variable),
            Reference::IndexedAccess(variable, index) => write!(f, "{}[{}]", variable, index),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct IndexedAccessExpression {
    pub variable: String,
    pub index: Box<Expression>,
}

impl IndexedAccessExpression {
    pub(crate) fn execute(&self, context: &mut Context) -> anyhow::Result<Expression> {
        let index = self.index.execute(context)?;
        if let Expression::Integer(index) = index {
            let reference = Expression::Reference(Reference::IndexedAccess(
                self.variable.clone(),
                index.as_usize(),
            ));

            reference.execute(context)
        } else {
            Err(anyhow!(
                "{:?} does not resolve to a valid index expression",
                index
            ))
        }
    }

    pub(crate) fn to_reference(&self, context: &mut Context) -> anyhow::Result<Expression> {
        let index = self.index.execute(context)?;
        if let Expression::Integer(index) = index {
            Ok(Expression::Reference(Reference::IndexedAccess(
                self.variable.clone(),
                index.as_usize(),
            )))
        } else {
            Err(anyhow!(
                "{:?} does not resolve to a valid index expression",
                index
            ))
        }
    }
}
