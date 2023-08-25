use crate::{interpreter::context::Context, Expression};

use super::Statement;

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub statements: Vec<Statement>,
}

impl Block {
    pub(crate) fn execute(&self, context: &mut Context) -> Result<Expression, anyhow::Error> {
        let mut result = Expression::Unit;

        context.push_stack();

        for statement in &self.statements {
            result = statement.execute(context)?;
        }

        context.pop_stack();

        Ok(result)
    }
}
