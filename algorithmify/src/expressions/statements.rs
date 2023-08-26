use super::{reference::Reference, Expression, IndexedAccessExpression};
use crate::interpreter::context::Context;
use anyhow::anyhow;

#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
    Assignment(Reference, Expression),
    IndexedAssigment(IndexedAccessExpression, Expression),
    Expression(Expression),
}

impl Statement {
    pub(crate) fn execute(&self, context: &mut Context) -> anyhow::Result<Expression> {
        match self {
            Self::Assignment(reference, expression) => {
                let result = expression.execute(context)?;
                context.insert_into_heap(reference, result)?;
                Ok(Expression::Unit)
            }
            Self::IndexedAssigment(key, value) => match key.to_reference(context) {
                Ok(Expression::Reference(reference)) => {
                    let result = value.execute(context)?;
                    context.insert_into_heap(&reference, result)?;
                    Ok(Expression::Unit)
                }
                Ok(_) => Err(anyhow!("Cannot assign to expression {:?}", key)),
                err @ Err(_) => err,
            },
            Self::Expression(expression) => expression.execute(context),
        }
    }
}
