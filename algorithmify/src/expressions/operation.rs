use std::collections::HashSet;

use super::{reference::Reference, Expression};

#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    Add(Expression, Expression),
    Sub(Expression, Expression),
    Div(Expression, Expression),
    Mul(Expression, Expression),
    BitAnd(Expression, Expression),
    BitOr(Expression, Expression),
}

impl Operation {
    pub fn execute(&self) -> anyhow::Result<Expression> {
        Ok(match self {
            Self::Add(lhs, rhs) => lhs.try_resolve_inner().add(rhs.try_resolve_inner())?,
            Self::Sub(lhs, rhs) => lhs.try_resolve_inner().sub(rhs.try_resolve_inner())?,
            Self::Mul(lhs, rhs) => lhs.try_resolve_inner().mul(rhs.try_resolve_inner())?,
            Self::Div(lhs, rhs) => lhs.try_resolve_inner().div(rhs.try_resolve_inner())?,
            Self::BitAnd(lhs, rhs) => lhs.try_resolve_inner().bitand(rhs.try_resolve_inner())?,
            Self::BitOr(lhs, rhs) => lhs.try_resolve_inner().bitor(rhs.try_resolve_inner())?,
        })
    }

    pub fn replace(&mut self, reference: &Reference, value: &Expression) {
        match self {
            Self::Add(lhs, rhs) => {
                lhs.replace(reference, value);
                rhs.replace(reference, value);
            }
            Self::Sub(lhs, rhs) => {
                lhs.replace(reference, value);
                rhs.replace(reference, value);
            }
            Self::Mul(lhs, rhs) => {
                lhs.replace(reference, value);
                rhs.replace(reference, value);
            }
            Self::Div(lhs, rhs) => {
                lhs.replace(reference, value);
                rhs.replace(reference, value);
            }
            Self::BitAnd(lhs, rhs) => {
                lhs.replace(reference, value);
                rhs.replace(reference, value);
            }
            Self::BitOr(lhs, rhs) => {
                lhs.replace(reference, value);
                rhs.replace(reference, value);
            }
        }
    }

    pub fn add_to_reference_set(&self, set: &mut HashSet<Reference>) {
        match self {
            Self::Add(lhs, rhs) => {
                lhs.add_to_reference_set(set);
                rhs.add_to_reference_set(set);
            }
            Self::Sub(lhs, rhs) => {
                lhs.add_to_reference_set(set);
                rhs.add_to_reference_set(set);
            }
            Self::Mul(lhs, rhs) => {
                lhs.add_to_reference_set(set);
                rhs.add_to_reference_set(set);
            }
            Self::Div(lhs, rhs) => {
                lhs.add_to_reference_set(set);
                rhs.add_to_reference_set(set);
            }
            Self::BitAnd(lhs, rhs) => {
                lhs.add_to_reference_set(set);
                rhs.add_to_reference_set(set);
            }
            Self::BitOr(lhs, rhs) => {
                lhs.add_to_reference_set(set);
                rhs.add_to_reference_set(set);
            }
        }
    }
}
