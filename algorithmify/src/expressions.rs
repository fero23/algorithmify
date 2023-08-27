use crate::interpreter::context::Context;

pub use self::{
    block::Block,
    conditions::Condition,
    float::Float,
    functions::{Function, FunctionBuilder, FunctionCall, FunctionParams},
    integer::Integer,
    loops::Loop,
    operation::Operation,
    reference::IndexedAccessExpression,
    reference::Reference,
    statements::Statement,
};

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
    FunctionCall(FunctionCall),
    Block(Box<Block>),
}

impl Expression {
    pub fn execute(&self, context: &mut Context) -> anyhow::Result<Expression> {
        match self {
            Self::Reference(reference) => reference.execute(context),
            Self::IndexedAccessExpression(expression) => expression.execute(context),
            Self::Operation(operation) => operation.execute(context),
            Self::Loop(loop_instance) => loop_instance.execute(context),
            Self::Condition(condition) => condition.execute(context),
            Self::Block(block) => block.execute(context),
            Self::FunctionCall(function_call) => function_call.execute(context),
            _ => Ok(self.clone()),
        }
    }

    fn as_boolean(&self) -> Option<bool> {
        match self {
            Self::Bool(bool) => Some(*bool),
            _ => None,
        }
    }
}
