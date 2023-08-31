use std::collections::HashMap;

use anyhow::anyhow;

use crate::expressions::{
    functions::FunctionArgParamPair, loops::Contract, reference::Reference, Expression,
};

pub type ContractMap = HashMap<String, Contract>;

pub struct Context {
    stack: Vec<HashMap<String, Expression>>,
    contracts: ContractMap,
}

impl Context {
    pub(crate) fn new(contracts: ContractMap) -> Self {
        Self {
            stack: Vec::new(),
            contracts,
        }
    }

    fn search_expression(&self, key: &str) -> Option<&Expression> {
        for map in self.stack.iter().rev() {
            if let Some(expression) = map.get(key) {
                return Some(expression);
            }
        }
        None
    }

    fn search_expression_mut(&mut self, key: &str) -> Option<&mut Expression> {
        for map in self.stack.iter_mut().rev() {
            if let Some(expression) = map.get_mut(key) {
                return Some(expression);
            }
        }
        None
    }

    pub(crate) fn search_reference(&self, reference: &Reference) -> Option<&Expression> {
        match reference {
            Reference::Variable(variable) => self.search_expression(variable),
            Reference::IndexedAccess(variable, index) => {
                self.search_expression(variable)
                    .and_then(|expression| match expression {
                        Expression::Vector(vector) => Some(&vector[*index]),
                        _ => None,
                    })
            }
        }
    }

    pub(crate) fn push_stack(&mut self) {
        self.stack.push(HashMap::new());
    }

    pub(crate) fn push_stack_from(&mut self, args: Vec<FunctionArgParamPair>) {
        let mut map = HashMap::new();
        for (key, value) in args {
            map.insert(key, value);
        }
        self.stack.push(map);
    }

    pub(crate) fn pop_stack(&mut self) {
        self.stack.pop();
    }

    pub(crate) fn insert_or_update_in_heap(
        &mut self,
        reference: &Reference,
        expression: Expression,
    ) -> anyhow::Result<()> {
        match reference {
            Reference::Variable(variable) => {
                if let Some(existing_expression) = self.search_expression_mut(variable) {
                    *existing_expression = expression;
                } else {
                    self.stack
                        .last_mut()
                        .unwrap()
                        .insert(variable.clone(), expression);
                }
            }
            Reference::IndexedAccess(variable, index) => {
                if let Some(vector_expression) = self.search_expression_mut(variable) {
                    match vector_expression {
                        Expression::Vector(vector) => vector[*index] = expression.clone(),
                        _ => return Err(anyhow!("{} is not a vector", variable)),
                    }
                } else {
                    return Err(anyhow!("{} not found", variable));
                }
            }
        }

        Ok(())
    }

    pub(crate) fn insert_into_heap(
        &mut self,
        reference: &Reference,
        expression: Expression,
    ) -> anyhow::Result<()> {
        match reference {
            Reference::Variable(variable) => {
                self.stack
                    .last_mut()
                    .unwrap()
                    .insert(variable.clone(), expression);
            }
            _ => {
                return Err(anyhow!("Cannot insert into {:?}", reference));
            }
        }

        Ok(())
    }

    pub(crate) fn get_contract(&self, tag: Option<&String>) -> Contract {
        if let Some(contract) = tag.and_then(|tag| self.contracts.get(tag)) {
            contract.clone()
        } else {
            Default::default()
        }
    }
}
