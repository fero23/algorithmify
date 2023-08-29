use crate::{interpreter::context::Context, interpreter::context::ContractMap};

use super::{statements::Statement, Expression};

pub type FunctionBuilder = fn() -> Function;
pub type FunctionArgs = Vec<String>;
pub type FunctionParams = Vec<Expression>;
pub type FunctionArgParamPair = (String, Expression);

pub struct Function {
    pub(crate) args: FunctionArgs,
    pub(crate) statements: Vec<Statement>,
    pub(crate) contracts: ContractMap,
}

impl Function {
    pub fn new(args: FunctionArgs, statements: Vec<Statement>, contracts: ContractMap) -> Self {
        Self {
            args,
            statements,
            contracts,
        }
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

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall {
    pub builder: FunctionBuilder,
    pub params: FunctionParams,
}

impl FunctionCall {
    pub(crate) fn execute(&self, context: &mut Context) -> anyhow::Result<Expression> {
        let args = self
            .params
            .iter()
            .map(|expression| expression.execute(context))
            .collect::<anyhow::Result<Vec<_>>>()?;

        let function = (self.builder)();
        let mut child_context = Context::new(function.contracts.clone());
        function.execute(&mut child_context, args)
    }
}
