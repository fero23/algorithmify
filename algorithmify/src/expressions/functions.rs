use crate::interpreter::context::Context;

use super::{statements::Statement, Expression};

pub type FunctionName = String;
pub type FunctionArgs = Vec<Expression>;

pub struct Function {
    pub(crate) statements: Vec<Statement>,
    pub(crate) return_expression: Expression,
}

impl Function {
    pub fn new(statements: Vec<Statement>, return_expression: Expression) -> Self {
        Self {
            statements,
            return_expression,
        }
    }

    pub fn execute(&self, context: &mut Context) -> anyhow::Result<Expression> {
        for statement in &self.statements {
            statement.execute(context)?;
        }
        self.return_expression.execute(context)
    }
}
