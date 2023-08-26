use crate::interpreter::context::Context;

use super::Expression;
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
    pub fn execute(&self, context: &mut Context) -> anyhow::Result<Expression> {
        Ok(match self {
            Self::Add(lhs, rhs) => add(lhs.execute(context)?, rhs.execute(context)?)?,
            Self::Sub(lhs, rhs) => sub(lhs.execute(context)?, rhs.execute(context)?)?,
            Self::Mul(lhs, rhs) => mul(lhs.execute(context)?, rhs.execute(context)?)?,
            Self::Div(lhs, rhs) => div(lhs.execute(context)?, rhs.execute(context)?)?,
            Self::BitAnd(lhs, rhs) => bitand(lhs.execute(context)?, rhs.execute(context)?)?,
            Self::BitOr(lhs, rhs) => bitor(lhs.execute(context)?, rhs.execute(context)?)?,
            Self::And(lhs, rhs) => and(lhs.execute(context)?, rhs.execute(context)?)?,
            Self::Or(lhs, rhs) => or(lhs.execute(context)?, rhs.execute(context)?)?,
            Self::Eq(lhs, rhs) => eq(&lhs.execute(context)?, &rhs.execute(context)?)?,
            Self::Ne(lhs, rhs) => ne(&lhs.execute(context)?, &rhs.execute(context)?)?,
            Self::Lt(lhs, rhs) => lt(&lhs.execute(context)?, &rhs.execute(context)?)?,
            Self::Lte(lhs, rhs) => lte(&lhs.execute(context)?, &rhs.execute(context)?)?,
            Self::Gt(lhs, rhs) => gt(&lhs.execute(context)?, &rhs.execute(context)?)?,
            Self::Gte(lhs, rhs) => gte(&lhs.execute(context)?, &rhs.execute(context)?)?,
        })
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
