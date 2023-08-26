use crate::interpreter::context::Context;

use super::{statements::Statement, Expression};

pub type FunctionName = String;
pub type FunctionArgs = Vec<String>;
pub type FunctionParams = Vec<Expression>;
pub type FunctionArgParamPair = (String, Expression);

pub struct Function {
    pub(crate) args: FunctionArgs,
    pub(crate) statements: Vec<Statement>,
}

impl Function {
    pub fn new(args: FunctionArgs, statements: Vec<Statement>) -> Self {
        Self { args, statements }
    }

    pub fn execute(
        &self,
        context: &mut Context,
        args: FunctionParams,
    ) -> anyhow::Result<Expression> {
        let arg_pairs = self.args.iter().cloned().zip(args.into_iter()).collect();
        context.push_stack_from(arg_pairs);

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
