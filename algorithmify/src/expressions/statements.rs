use crate::interpreter::context::Context;

use super::{
    functions::{FunctionArgs, FunctionName},
    reference::Reference,
    Expression,
};

#[derive(Clone, Debug)]
pub enum Statement {
    FunctionCall(FunctionName, FunctionArgs),
    Assignment(Reference, Expression),
}

impl Statement {
    pub(crate) fn execute(&self, context: &mut Context) -> anyhow::Result<Expression> {
        match self {
            Self::Assignment(reference, expression) => {
                let result = expression.execute(context)?;
                context.insert_into_heap(reference, result)?;
                Ok(Expression::Unit)
            }
            _ => todo!(),
        }
    }
}
