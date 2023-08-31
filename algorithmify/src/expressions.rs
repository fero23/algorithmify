use crate::interpreter::context::Context;

pub use self::{
    block::Block,
    conditions::Condition,
    float::Float,
    functions::{Function, FunctionBuilder, FunctionCall, FunctionParams},
    integer::Integer,
    loops::Loop,
    method_call::MethodCall,
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
pub mod method_call;
pub mod operation;
pub mod reference;
pub mod statements;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Unit,
    Break,
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
    MethodCall(MethodCall),
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
            Self::MethodCall(method_call) => method_call.execute(context),
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

impl From<i32> for Expression {
    fn from(value: i32) -> Self {
        Expression::Integer(Integer::I32(value))
    }
}

impl From<i64> for Expression {
    fn from(value: i64) -> Self {
        Expression::Integer(Integer::I64(value))
    }
}

impl From<usize> for Expression {
    fn from(value: usize) -> Self {
        Expression::Integer(Integer::Usize(value))
    }
}

impl<T: Into<Expression>> From<Vec<T>> for Expression {
    fn from(iterator: Vec<T>) -> Self {
        let vector = iterator
            .into_iter()
            .map(|expression| expression.into())
            .collect();

        Expression::Vector(vector)
    }
}
