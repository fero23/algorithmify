use crate::{interpreter::context::Context, Expression};
use anyhow::anyhow;

#[derive(Clone, Debug, PartialEq)]
pub struct MethodCall {
    pub expression: Box<Expression>,
    pub method: String,
    pub args: Vec<Expression>,
}

impl MethodCall {
    pub(crate) fn execute(&self, context: &mut Context) -> anyhow::Result<Expression> {
        let expression = self.expression.execute(context)?;
        execute_method(expression, &self.method, &self.args)
    }
}

fn execute_method(
    expression: Expression,
    method: &str,
    args: &Vec<Expression>,
) -> anyhow::Result<Expression> {
    match (&expression, method, args) {
        (Expression::Vector(vec), "len", _) => Ok(vec.len().into()),
        _ => Err(anyhow!(
            "Invalid method '{}' for value '{:?}'",
            method,
            expression
        )),
    }
}
