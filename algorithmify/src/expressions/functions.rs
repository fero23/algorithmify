use crate::interpreter::context::Context;

use super::{statements::Statement, Expression};

pub type FunctionName = String;
pub type FunctionArgs = Vec<Expression>;

pub struct Function {
    pub(crate) statements: Vec<Statement>,
}

impl Function {
    pub fn new(statements: Vec<Statement>) -> Self {
        Self { statements }
    }

    pub fn execute(&self, context: &mut Context) -> anyhow::Result<Expression> {
        context.push_stack();

        for statement in &self.statements[0..self.statements.len() - 1] {
            statement.execute(context)?;
        }

        let result = if let Some(last_statement) = self.statements.last() {
            last_statement.execute(context)
        } else {
            Ok(Expression::Unit)
        }?;

        context.pop_stack();

        Ok(result)
    }
}
