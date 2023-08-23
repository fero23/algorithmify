use std::collections::HashSet;

use super::{reference::Reference, Expression};
use anyhow::anyhow;

#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    Add(Expression, Expression),
    Sub(Expression, Expression),
    Div(Expression, Expression),
    Mul(Expression, Expression),
    BitAnd(Expression, Expression),
    BitOr(Expression, Expression),
    And(Expression, Expression),
    Or(Expression, Expression),
    Eq(Expression, Expression),
    Ne(Expression, Expression),
    Lt(Expression, Expression),
    Lte(Expression, Expression),
    Gt(Expression, Expression),
    Gte(Expression, Expression),
}

impl Operation {
    pub fn execute(&self) -> anyhow::Result<Expression> {
        Ok(match self {
            Self::Add(lhs, rhs) => add(lhs.try_resolve_inner(), rhs.try_resolve_inner())?,
            Self::Sub(lhs, rhs) => sub(lhs.try_resolve_inner(), rhs.try_resolve_inner())?,
            Self::Mul(lhs, rhs) => mul(lhs.try_resolve_inner(), rhs.try_resolve_inner())?,
            Self::Div(lhs, rhs) => div(lhs.try_resolve_inner(), rhs.try_resolve_inner())?,
            Self::BitAnd(lhs, rhs) => bitand(lhs.try_resolve_inner(), rhs.try_resolve_inner())?,
            Self::BitOr(lhs, rhs) => bitor(lhs.try_resolve_inner(), rhs.try_resolve_inner())?,
            Self::And(lhs, rhs) => and(lhs.try_resolve_inner(), rhs.try_resolve_inner())?,
            Self::Or(lhs, rhs) => or(lhs.try_resolve_inner(), rhs.try_resolve_inner())?,
            Self::Eq(lhs, rhs) => eq(&lhs.try_resolve_inner(), &rhs.try_resolve_inner())?,
            Self::Ne(lhs, rhs) => ne(&lhs.try_resolve_inner(), &rhs.try_resolve_inner())?,
            Self::Lt(lhs, rhs) => lt(&lhs.try_resolve_inner(), &rhs.try_resolve_inner())?,
            Self::Lte(lhs, rhs) => lte(&lhs.try_resolve_inner(), &rhs.try_resolve_inner())?,
            Self::Gt(lhs, rhs) => gt(&lhs.try_resolve_inner(), &rhs.try_resolve_inner())?,
            Self::Gte(lhs, rhs) => gte(&lhs.try_resolve_inner(), &rhs.try_resolve_inner())?,
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
            Self::And(lhs, rhs) => {
                lhs.replace(reference, value);
                rhs.replace(reference, value);
            }
            Self::Or(lhs, rhs) => {
                lhs.replace(reference, value);
                rhs.replace(reference, value);
            }
            Self::Eq(lhs, rhs) => {
                lhs.replace(reference, value);
                rhs.replace(reference, value);
            }
            Self::Ne(lhs, rhs) => {
                lhs.replace(reference, value);
                rhs.replace(reference, value);
            }
            Self::Lt(lhs, rhs) => {
                lhs.replace(reference, value);
                rhs.replace(reference, value);
            }
            Self::Lte(lhs, rhs) => {
                lhs.replace(reference, value);
                rhs.replace(reference, value);
            }
            Self::Gt(lhs, rhs) => {
                lhs.replace(reference, value);
                rhs.replace(reference, value);
            }
            Self::Gte(lhs, rhs) => {
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
            Self::And(lhs, rhs) => {
                lhs.add_to_reference_set(set);
                rhs.add_to_reference_set(set);
            }
            Self::Or(lhs, rhs) => {
                lhs.add_to_reference_set(set);
                rhs.add_to_reference_set(set);
            }
            Self::Eq(lhs, rhs) => {
                lhs.add_to_reference_set(set);
                rhs.add_to_reference_set(set);
            }
            Self::Ne(lhs, rhs) => {
                lhs.add_to_reference_set(set);
                rhs.add_to_reference_set(set);
            }
            Self::Lt(lhs, rhs) => {
                lhs.add_to_reference_set(set);
                rhs.add_to_reference_set(set);
            }
            Self::Lte(lhs, rhs) => {
                lhs.add_to_reference_set(set);
                rhs.add_to_reference_set(set);
            }
            Self::Gt(lhs, rhs) => {
                lhs.add_to_reference_set(set);
                rhs.add_to_reference_set(set);
            }
            Self::Gte(lhs, rhs) => {
                lhs.add_to_reference_set(set);
                rhs.add_to_reference_set(set);
            }
        }
    }
}

fn add(lhs: Expression, rhs: Expression) -> anyhow::Result<Expression> {
    match (lhs, rhs) {
        (Expression::Integer(lhs), Expression::Integer(rhs)) => Ok(Expression::Integer(lhs + rhs)),
        (Expression::Float(lhs), Expression::Float(rhs)) => Ok(Expression::Float(lhs + rhs)),
        (Expression::String(lhs), Expression::String(rhs)) => {
            Ok(Expression::String(lhs.clone() + &rhs))
        }
        (Expression::String(lhs), Expression::Char(rhs)) => {
            Ok(Expression::String(lhs.clone() + &rhs.to_string()))
        }
        (Expression::Char(lhs), Expression::String(rhs)) => {
            Ok(Expression::String(lhs.to_string() + &rhs))
        }
        (lhs, rhs) => Err(anyhow!(
            "Unsupported addition between {:?} and {:?}",
            lhs,
            rhs
        )),
    }
}

fn sub(lhs: Expression, rhs: Expression) -> anyhow::Result<Expression> {
    match (lhs, rhs) {
        (Expression::Integer(lhs), Expression::Integer(rhs)) => Ok(Expression::Integer(lhs - rhs)),
        (Expression::Float(lhs), Expression::Float(rhs)) => Ok(Expression::Float(lhs - rhs)),
        (lhs, rhs) => Err(anyhow!(
            "Unsupported substraction between {:?} and {:?}",
            lhs,
            rhs
        )),
    }
}

fn mul(lhs: Expression, rhs: Expression) -> anyhow::Result<Expression> {
    match (lhs, rhs) {
        (Expression::Integer(lhs), Expression::Integer(rhs)) => Ok(Expression::Integer(lhs * rhs)),
        (Expression::Float(lhs), Expression::Float(rhs)) => Ok(Expression::Float(lhs * rhs)),
        (lhs, rhs) => Err(anyhow!(
            "Unsupported multiplication between {:?} and {:?}",
            lhs,
            rhs
        )),
    }
}

fn div(lhs: Expression, rhs: Expression) -> anyhow::Result<Expression> {
    match (lhs, rhs) {
        (Expression::Integer(lhs), Expression::Integer(rhs)) => Ok(Expression::Integer(lhs / rhs)),
        (Expression::Float(lhs), Expression::Float(rhs)) => Ok(Expression::Float(lhs / rhs)),
        (lhs, rhs) => Err(anyhow!(
            "Unsupported division between {:?} and {:?}",
            lhs,
            rhs
        )),
    }
}

fn bitand(lhs: Expression, rhs: Expression) -> anyhow::Result<Expression> {
    match (lhs, rhs) {
        (Expression::Integer(lhs), Expression::Integer(rhs)) => Ok(Expression::Integer(lhs & rhs)),
        (lhs, rhs) => Err(anyhow!(
            "Unsupported bitwise AND between {:?} and {:?}",
            lhs,
            rhs
        )),
    }
}

fn bitor(lhs: Expression, rhs: Expression) -> anyhow::Result<Expression> {
    match (lhs, rhs) {
        (Expression::Integer(lhs), Expression::Integer(rhs)) => Ok(Expression::Integer(lhs | rhs)),
        (lhs, rhs) => Err(anyhow!(
            "Unsupported bitwise OR between {:?} and {:?}",
            lhs,
            rhs
        )),
    }
}

fn and(lhs: Expression, rhs: Expression) -> anyhow::Result<Expression> {
    match (lhs, rhs) {
        (Expression::Bool(lhs), Expression::Bool(rhs)) => Ok(Expression::Bool(lhs && rhs)),
        (lhs, rhs) => Err(anyhow!("Unsupported AND between {:?} and {:?}", lhs, rhs)),
    }
}

fn or(lhs: Expression, rhs: Expression) -> anyhow::Result<Expression> {
    match (lhs, rhs) {
        (Expression::Bool(lhs), Expression::Bool(rhs)) => Ok(Expression::Bool(lhs || rhs)),
        (lhs, rhs) => Err(anyhow!("Unsupported OR between {:?} and {:?}", lhs, rhs)),
    }
}

fn eq(lhs: &Expression, rhs: &Expression) -> anyhow::Result<Expression> {
    match (lhs, rhs) {
        (Expression::Integer(lhs), Expression::Integer(rhs)) => Ok(Expression::Bool(*lhs == *rhs)),
        (Expression::Float(lhs), Expression::Float(rhs)) => Ok(Expression::Bool(*lhs == *rhs)),
        (Expression::Bool(lhs), Expression::Bool(rhs)) => Ok(Expression::Bool(*lhs == *rhs)),
        (lhs, rhs) => Err(anyhow!(
            "Unsupported equals between {:?} and {:?}",
            lhs,
            rhs
        )),
    }
}

fn ne(lhs: &Expression, rhs: &Expression) -> anyhow::Result<Expression> {
    eq(lhs, rhs)
        .map(|result| Expression::Bool(!result.as_boolean().unwrap()))
        .map_err(|_| anyhow!("Unsupported not equals between {:?} and {:?}", lhs, rhs))
}

fn lt(lhs: &Expression, rhs: &Expression) -> anyhow::Result<Expression> {
    match (lhs, rhs) {
        (Expression::Integer(lhs), Expression::Integer(rhs)) => Ok(Expression::Bool(*lhs < *rhs)),
        (Expression::Float(lhs), Expression::Float(rhs)) => Ok(Expression::Bool(*lhs < *rhs)),
        (Expression::Bool(lhs), Expression::Bool(rhs)) => Ok(Expression::Bool(*lhs < *rhs)),
        (lhs, rhs) => Err(anyhow!(
            "Unsupported less than between {:?} and {:?}",
            lhs,
            rhs
        )),
    }
}

fn lte(lhs: &Expression, rhs: &Expression) -> anyhow::Result<Expression> {
    eq(lhs, rhs)
        .and_then(|eq| {
            lt(lhs, rhs)
                .map(|lt| Expression::Bool(eq.as_boolean().unwrap() || lt.as_boolean().unwrap()))
        })
        .map_err(|_| {
            anyhow!(
                "Unsupported less than equals between {:?} and {:?}",
                lhs,
                rhs
            )
        })
}

fn gt(lhs: &Expression, rhs: &Expression) -> anyhow::Result<Expression> {
    eq(lhs, rhs)
        .and_then(|eq| {
            lt(lhs, rhs)
                .map(|lt| Expression::Bool(!eq.as_boolean().unwrap() && !lt.as_boolean().unwrap()))
        })
        .map_err(|_| anyhow!("Unsupported greater than between {:?} and {:?}", lhs, rhs))
}

fn gte(lhs: &Expression, rhs: &Expression) -> anyhow::Result<Expression> {
    lt(lhs, rhs)
        .map(|result| Expression::Bool(!result.as_boolean().unwrap()))
        .map_err(|_| {
            anyhow!(
                "Unsupported greater than equals between {:?} and {:?}",
                lhs,
                rhs
            )
        })
}
