use anyhow::anyhow;

use crate::{interpreter::context::Context, Expression};

use super::{Reference, Statement};

#[derive(Debug, Clone, PartialEq)]
pub enum Loop {
    While(WhileLoop),
    RangedForLoop(RangedForLoop),
}

impl Loop {
    pub fn execute(&self, context: &mut Context) -> anyhow::Result<Expression> {
        match self {
            Self::While(while_loop) => while_loop.execute(context),
            Self::RangedForLoop(for_loop) => for_loop.execute(context),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WhileLoop {
    pub statements: Vec<Statement>,
    pub condition: Expression,
}
impl WhileLoop {
    fn execute(&self, context: &mut Context) -> Result<Expression, anyhow::Error> {
        let mut result = Expression::Unit;

        while let Expression::Bool(true) = self.condition.execute(context)? {
            for statement in &self.statements {
                result = statement.execute(context)?;
            }
        }

        Ok(result)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RangedForLoop {
    pub statements: Vec<Statement>,
    pub variable: Reference,
    pub start: Expression,
    pub end: Expression,
}
impl RangedForLoop {
    fn execute(&self, context: &mut Context) -> Result<Expression, anyhow::Error> {
        let mut result = Expression::Unit;

        let previous_variable_value = context.search_reference(&self.variable).cloned();

        let start = self.start.execute(context)?;
        let end = self.end.execute(context)?;

        let (start, end) =
            if let (Expression::Integer(start), Expression::Integer(end)) = (&start, &end) {
                (start.as_usize(), end.as_usize())
            } else {
                return Err(anyhow!("Invalid range from '{:?}' to '{:?}'", start, end));
            };

        for i in start..end {
            context.insert_into_heap(&self.variable, Expression::Integer(i.into()))?;
            for statement in &self.statements {
                result = statement.execute(context)?;
            }
        }

        if let Some(previous_variable_value) = previous_variable_value {
            context.insert_into_heap(&self.variable, previous_variable_value)?;
        }

        Ok(result)
    }
}
