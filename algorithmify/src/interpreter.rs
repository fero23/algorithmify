use crate::{expressions::Expression, Function};

use self::context::{Context, ContractMap};

pub mod context;

pub struct Interpreter {
    root_context: Context,
}

impl Interpreter {
    pub fn new(contracts: ContractMap) -> Self {
        Self {
            root_context: Context::new(contracts),
        }
    }

    pub fn execute_function(function: Function) -> anyhow::Result<Expression> {
        let mut interpreter = Interpreter::new(function.contracts.clone());
        function.execute(&mut interpreter.root_context, vec![])
    }

    pub fn execute_function_with_args(
        function: Function,
        expressions: Vec<Expression>,
    ) -> anyhow::Result<Expression> {
        let mut interpreter = Interpreter::new(function.contracts.clone());
        function.execute(&mut interpreter.root_context, expressions)
    }
}
