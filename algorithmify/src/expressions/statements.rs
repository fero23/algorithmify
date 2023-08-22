use crate::interpreter::context::Context;

use super::{reference::Reference, Expression};

#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
    Assignment(Reference, Expression),
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
            Self::Expression(expression) => expression.execute(context),
        }
    }
}
