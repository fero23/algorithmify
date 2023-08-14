use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Float {
    F32(f32),
    F64(f64),
}

impl Float {
    fn as_f64(&self) -> f64 {
        match self {
            Self::F32(value) => *value as f64,
            Self::F64(value) => *value,
        }
    }
}

impl Add for Float {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::F32(lhs), Self::F32(rhs)) => Self::F32(lhs + rhs),
            (Self::F64(lhs), Self::F64(rhs)) => Self::F64(lhs + rhs),
            _ => Self::F64(self.as_f64() + rhs.as_f64()),
        }
    }
}

impl Sub for Float {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::F32(lhs), Self::F32(rhs)) => Self::F32(lhs - rhs),
            (Self::F64(lhs), Self::F64(rhs)) => Self::F64(lhs - rhs),
            _ => Self::F64(self.as_f64() - rhs.as_f64()),
        }
    }
}

impl Mul for Float {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::F32(lhs), Self::F32(rhs)) => Self::F32(lhs * rhs),
            (Self::F64(lhs), Self::F64(rhs)) => Self::F64(lhs * rhs),
            _ => Self::F64(self.as_f64() * rhs.as_f64()),
        }
    }
}

impl Div for Float {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::F32(lhs), Self::F32(rhs)) => Self::F32(lhs / rhs),
            (Self::F64(lhs), Self::F64(rhs)) => Self::F64(lhs / rhs),
            _ => Self::F64(self.as_f64() / rhs.as_f64()),
        }
    }
}
