use crate::{interpreter::context::Context, Expression};

use super::Statement;

#[derive(Debug, Clone, PartialEq)]
pub enum Condition {
    If(If),
}

impl Condition {
    pub fn execute(&self, context: &mut Context) -> anyhow::Result<Expression> {
        match self {
            Self::If(condition) => condition.execute(context),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct If {
    pub statements: Vec<Statement>,
    pub else_clause: Option<Expression>,
    pub condition: Expression,
}

impl If {
    fn execute(&self, context: &mut Context) -> Result<Expression, anyhow::Error> {
        let mut result = Expression::Unit;

        if let Expression::Bool(true) = self.condition.execute(context)? {
            for statement in &self.statements {
                result = statement.execute(context)?;
            }
        } else {
            if let Some(else_clause) = &self.else_clause {
                result = else_clause.execute(context)?;
            }
        }

        Ok(result)
    }
}
