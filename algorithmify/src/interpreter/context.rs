use std::collections::HashMap;

use anyhow::anyhow;

use crate::expressions::{reference::Reference, Expression};

pub struct Context {
    heap: HashMap<String, Expression>,
}

impl Context {
    pub(crate) fn search_reference(&self, reference: &Reference) -> Option<&Expression> {
        match reference {
            Reference::Variable(variable) => self.heap.get(variable),
            Reference::IndexedAccess(variable, index) => {
                self.heap
                    .get(variable)
                    .and_then(|expression| match expression {
                        Expression::Vector(vector) => Some(&vector[*index]),
                        _ => None,
                    })
            }
        }
    }

    pub(crate) fn insert_into_heap(
        &mut self,
        reference: &Reference,
        expression: Expression,
    ) -> anyhow::Result<()> {
        match reference {
            Reference::Variable(variable) => {
                self.heap.insert(variable.clone(), expression);
            }
            Reference::IndexedAccess(variable, index) => {
                if let Some(vector_expression) = self.heap.get_mut(variable) {
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
}
