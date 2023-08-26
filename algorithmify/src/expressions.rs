use std::collections::HashSet;

use crate::interpreter::context::Context;

pub use self::{
    block::Block,
    conditions::Condition,
    float::Float,
    functions::Function,
    functions::{FunctionArgs, FunctionName},
    integer::Integer,
    loops::Loop,
    operation::Operation,
    reference::IndexedAccessExpression,
    reference::Reference,
    statements::Statement,
};
use anyhow::anyhow;

pub mod block;
pub mod conditions;
pub mod float;
pub mod functions;
pub mod integer;
pub mod loops;
pub mod operation;
pub mod reference;
pub mod statements;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Unit,
    Vector(Vec<Expression>),
    Reference(Reference),
    IndexedAccessExpression(IndexedAccessExpression),
    Integer(Integer),
    Float(Float),
    Char(char),
    String(String),
    Bool(bool),
    Operation(Box<Operation>),
    Condition(Box<Condition>),
    Loop(Box<Loop>),
    FunctionCall(FunctionName, FunctionArgs),
    Block(Box<Block>),
}

impl Expression {
    pub fn execute(&self, context: &mut Context) -> anyhow::Result<Expression> {
        let mut result = self.clone();
        result.try_replace_references(context)?;

        match result {
            Self::IndexedAccessExpression(expression) => expression.execute(context),
            Self::Operation(operation) => operation.execute(context),
            Self::Loop(loop_instance) => loop_instance.execute(context),
            Self::Condition(condition) => condition.execute(context),
            Self::Block(block) => block.execute(context),
            _ => Ok(result),
        }
    }

    fn try_replace_references(&mut self, context: &Context) -> anyhow::Result<()> {
        if let Expression::Vector(_) | Expression::Reference(_) | Expression::Operation(_) = self {
            let references = self.get_reference_set();

            for reference in references {
                let result = context.search_reference(&reference);
                if let Some(expression) = result {
                    self.replace(&reference, expression);
                } else {
                    return Err(anyhow!("Unknown reference {}", reference));
                }
            }
        }

        Ok(())
    }

    fn get_reference_set(&self) -> HashSet<Reference> {
        let mut set = HashSet::new();
        self.add_to_reference_set(&mut set);
        set
    }

    fn add_to_reference_set(&self, set: &mut HashSet<Reference>) {
        match self {
            Self::Reference(reference) => {
                set.insert(reference.clone());
            }
            Self::Operation(operation) => operation.add_to_reference_set(set),
            _ => {}
        }
    }

    fn replace(&mut self, reference: &Reference, value: &Expression) {
        match self {
            Self::Reference(r) if r == reference => *self = value.clone(),
            Self::Operation(operation) => operation.replace(reference, value),
            _ => {}
        }
    }

    fn as_boolean(&self) -> Option<bool> {
        match self {
            Self::Bool(bool) => Some(*bool),
            _ => None,
        }
    }
}
