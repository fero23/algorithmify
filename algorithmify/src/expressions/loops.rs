use anyhow::anyhow;

use crate::{interpreter::context::Context, Expression};

use super::{FunctionBuilder, Reference, Statement};

#[derive(Debug, Clone, PartialEq)]
pub enum Loop {
    While(WhileLoop),
    RangedFor(RangedForLoop),
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Contract {
    pub pre_condition: Option<(String, FunctionBuilder)>,
    pub maintenance_condition: Option<(String, FunctionBuilder)>,
    pub post_condition: Option<(String, FunctionBuilder)>,
}

impl Contract {
    pub(crate) fn validate_pre_condition(&self, context: &mut Context) -> anyhow::Result<()> {
        if let Some((name, pre_condition)) = self.pre_condition.as_ref() {
            return match pre_condition().execute(context, Default::default())? {
                Expression::Bool(true) => Ok(()),
                Expression::Bool(false) => Err(anyhow!("Pre-condition '{}' failed", name)),
                other => Err(anyhow!(
                    "Expected boolean, got '{:?}' when validating '{}'",
                    other,
                    name
                )),
            };
        } else {
            Ok(())
        }
    }

    pub(crate) fn validate_maintenance_condition(
        &self,
        context: &mut Context,
    ) -> anyhow::Result<()> {
        if let Some((name, maintenance_condition)) = self.maintenance_condition.as_ref() {
            return match maintenance_condition().execute(context, Default::default())? {
                Expression::Bool(true) => Ok(()),
                Expression::Bool(false) => Err(anyhow!("Maintenance condition '{}' failed", name)),
                other => Err(anyhow!(
                    "Expected boolean, got '{:?}' when validating '{}'",
                    other,
                    name
                )),
            };
        } else {
            Ok(())
        }
    }

    pub(crate) fn validate_post_condition(&self, context: &mut Context) -> anyhow::Result<()> {
        if let Some((name, post_condition)) = self.post_condition.as_ref() {
            return match post_condition().execute(context, Default::default())? {
                Expression::Bool(true) => Ok(()),
                Expression::Bool(false) => Err(anyhow!("Post-condition '{}' failed", name)),
                other => Err(anyhow!(
                    "Expected boolean, got '{:?}' when validating '{}'",
                    other,
                    name
                )),
            };
        } else {
            Ok(())
        }
    }
}

impl Loop {
    pub fn execute(&self, context: &mut Context) -> anyhow::Result<Expression> {
        match self {
            Self::While(while_loop) => while_loop.execute(context),
            Self::RangedFor(for_loop) => for_loop.execute(context),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WhileLoop {
    pub tag: Option<String>,
    pub statements: Vec<Statement>,
    pub condition: Expression,
}

impl WhileLoop {
    fn execute(&self, context: &mut Context) -> Result<Expression, anyhow::Error> {
        let mut result = Expression::Unit;

        let contract = context.get_contract(self.tag.as_ref());

        contract.validate_pre_condition(context)?;

        while let Expression::Bool(true) = self.condition.execute(context)? {
            context.push_stack();

            for statement in &self.statements {
                result = statement.execute(context)?;
            }

            context.pop_stack();

            contract.validate_maintenance_condition(context)?;
        }

        contract.validate_post_condition(context)?;

        Ok(result)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RangedForLoop {
    pub tag: Option<String>,
    pub statements: Vec<Statement>,
    pub variable: Reference,
    pub start: Expression,
    pub end: Expression,
}

impl RangedForLoop {
    fn execute(&self, context: &mut Context) -> Result<Expression, anyhow::Error> {
        let mut result = Expression::Unit;

        let contract = context.get_contract(self.tag.as_ref());

        context.push_stack();

        let previous_variable_value = context.search_reference(&self.variable).cloned();

        let start = self.start.execute(context)?;
        let end = self.end.execute(context)?;

        let (start, end) =
            if let (Expression::Integer(start), Expression::Integer(end)) = (&start, &end) {
                (start.as_usize(), end.as_usize())
            } else {
                return Err(anyhow!("Invalid range from '{:?}' to '{:?}'", start, end));
            };

        contract.validate_pre_condition(context)?;

        for i in start..end {
            context.push_stack();

            context.insert_into_heap(&self.variable, i.into())?;
            for statement in &self.statements {
                result = statement.execute(context)?;
            }

            context.pop_stack();

            contract.validate_maintenance_condition(context)?;
        }

        if let Some(previous_variable_value) = previous_variable_value {
            context.insert_into_heap(&self.variable, previous_variable_value)?;
        }

        context.pop_stack();

        contract.validate_post_condition(context)?;

        Ok(result)
    }
}
