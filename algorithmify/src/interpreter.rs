use crate::{expressions::Expression, Function};

use self::context::Context;

pub mod context;

pub struct Interpreter {
    context: Context,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            context: Context::new(),
        }
    }

    pub fn execute_function(function: Function) -> anyhow::Result<Expression> {
        let mut interpreter = Interpreter::new();
        function.execute(&mut interpreter.context, vec![])
    }
}
