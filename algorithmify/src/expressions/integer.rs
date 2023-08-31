use std::ops::{Add, BitAnd, BitOr, Div, Mul, Sub};

#[derive(Debug, Copy, Clone)]
pub enum Integer {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    Isize(isize),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    Usize(usize),
}

impl Integer {
    pub fn as_i64(&self) -> i64 {
        match self {
            Self::I8(value) => *value as i64,
            Self::I16(value) => *value as i64,
            Self::I32(value) => *value as i64,
            Self::I64(value) => *value,
            Self::Isize(value) => *value as i64,
            Self::U8(value) => *value as i64,
            Self::U16(value) => *value as i64,
            Self::U32(value) => *value as i64,
            Self::U64(value) => *value as i64,
            Self::Usize(value) => *value as i64,
        }
    }

    pub fn as_usize(&self) -> usize {
        match self {
            Self::I8(value) => *value as usize,
            Self::I16(value) => *value as usize,
            Self::I32(value) => *value as usize,
            Self::I64(value) => *value as usize,
            Self::Isize(value) => *value as usize,
            Self::U8(value) => *value as usize,
            Self::U16(value) => *value as usize,
            Self::U32(value) => *value as usize,
            Self::U64(value) => *value as usize,
            Self::Usize(value) => *value,
        }
    }
}

impl Add for Integer {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::I8(lhs), Self::I8(rhs)) => Self::I8(lhs + rhs),
            (Self::I16(lhs), Self::I16(rhs)) => Self::I16(lhs + rhs),
            (Self::I32(lhs), Self::I32(rhs)) => Self::I32(lhs + rhs),
            (Self::I64(lhs), Self::I64(rhs)) => Self::I64(lhs + rhs),
            (Self::Isize(lhs), Self::Isize(rhs)) => Self::Isize(lhs + rhs),
            (Self::U8(lhs), Self::U8(rhs)) => Self::U8(lhs + rhs),
            (Self::U16(lhs), Self::U16(rhs)) => Self::U16(lhs + rhs),
            (Self::U32(lhs), Self::U32(rhs)) => Self::U32(lhs + rhs),
            (Self::U64(lhs), Self::U64(rhs)) => Self::U64(lhs + rhs),
            (Self::Usize(lhs), Self::Usize(rhs)) => Self::Usize(lhs + rhs),
            _ => Self::I64(self.as_i64() + rhs.as_i64()),
        }
    }
}

impl Sub for Integer {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::I8(lhs), Self::I8(rhs)) => Self::I8(lhs - rhs),
            (Self::I16(lhs), Self::I16(rhs)) => Self::I16(lhs - rhs),
            (Self::I32(lhs), Self::I32(rhs)) => Self::I32(lhs - rhs),
            (Self::I64(lhs), Self::I64(rhs)) => Self::I64(lhs - rhs),
            (Self::Isize(lhs), Self::Isize(rhs)) => Self::Isize(lhs - rhs),
            (Self::U8(lhs), Self::U8(rhs)) => Self::U8(lhs - rhs),
            (Self::U16(lhs), Self::U16(rhs)) => Self::U16(lhs - rhs),
            (Self::U32(lhs), Self::U32(rhs)) => Self::U32(lhs - rhs),
            (Self::U64(lhs), Self::U64(rhs)) => Self::U64(lhs - rhs),
            (Self::Usize(lhs), Self::Usize(rhs)) => Self::Usize(lhs - rhs),
            _ => Self::I64(self.as_i64() - rhs.as_i64()),
        }
    }
}

impl Mul for Integer {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::I8(lhs), Self::I8(rhs)) => Self::I8(lhs * rhs),
            (Self::I16(lhs), Self::I16(rhs)) => Self::I16(lhs * rhs),
            (Self::I32(lhs), Self::I32(rhs)) => Self::I32(lhs * rhs),
            (Self::I64(lhs), Self::I64(rhs)) => Self::I64(lhs * rhs),
            (Self::Isize(lhs), Self::Isize(rhs)) => Self::Isize(lhs * rhs),
            (Self::U8(lhs), Self::U8(rhs)) => Self::U8(lhs * rhs),
            (Self::U16(lhs), Self::U16(rhs)) => Self::U16(lhs * rhs),
            (Self::U32(lhs), Self::U32(rhs)) => Self::U32(lhs * rhs),
            (Self::U64(lhs), Self::U64(rhs)) => Self::U64(lhs * rhs),
            (Self::Usize(lhs), Self::Usize(rhs)) => Self::Usize(lhs * rhs),
            _ => Self::I64(self.as_i64() * rhs.as_i64()),
        }
    }
}

impl Div for Integer {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::I8(lhs), Self::I8(rhs)) => Self::I8(lhs / rhs),
            (Self::I16(lhs), Self::I16(rhs)) => Self::I16(lhs / rhs),
            (Self::I32(lhs), Self::I32(rhs)) => Self::I32(lhs / rhs),
            (Self::I64(lhs), Self::I64(rhs)) => Self::I64(lhs / rhs),
            (Self::Isize(lhs), Self::Isize(rhs)) => Self::Isize(lhs / rhs),
            (Self::U8(lhs), Self::U8(rhs)) => Self::U8(lhs / rhs),
            (Self::U16(lhs), Self::U16(rhs)) => Self::U16(lhs / rhs),
            (Self::U32(lhs), Self::U32(rhs)) => Self::U32(lhs / rhs),
            (Self::U64(lhs), Self::U64(rhs)) => Self::U64(lhs / rhs),
            (Self::Usize(lhs), Self::Usize(rhs)) => Self::Usize(lhs / rhs),
            _ => Self::I64(self.as_i64() / rhs.as_i64()),
        }
    }
}

impl BitAnd for Integer {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::I8(lhs), Self::I8(rhs)) => Self::I8(lhs & rhs),
            (Self::I16(lhs), Self::I16(rhs)) => Self::I16(lhs & rhs),
            (Self::I32(lhs), Self::I32(rhs)) => Self::I32(lhs & rhs),
            (Self::I64(lhs), Self::I64(rhs)) => Self::I64(lhs & rhs),
            (Self::Isize(lhs), Self::Isize(rhs)) => Self::Isize(lhs & rhs),
            (Self::U8(lhs), Self::U8(rhs)) => Self::U8(lhs & rhs),
            (Self::U16(lhs), Self::U16(rhs)) => Self::U16(lhs & rhs),
            (Self::U32(lhs), Self::U32(rhs)) => Self::U32(lhs & rhs),
            (Self::U64(lhs), Self::U64(rhs)) => Self::U64(lhs & rhs),
            (Self::Usize(lhs), Self::Usize(rhs)) => Self::Usize(lhs & rhs),
            _ => Self::I64(self.as_i64() & rhs.as_i64()),
        }
    }
}

impl BitOr for Integer {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::I8(lhs), Self::I8(rhs)) => Self::I8(lhs | rhs),
            (Self::I16(lhs), Self::I16(rhs)) => Self::I16(lhs | rhs),
            (Self::I32(lhs), Self::I32(rhs)) => Self::I32(lhs | rhs),
            (Self::I64(lhs), Self::I64(rhs)) => Self::I64(lhs | rhs),
            (Self::Isize(lhs), Self::Isize(rhs)) => Self::Isize(lhs | rhs),
            (Self::U8(lhs), Self::U8(rhs)) => Self::U8(lhs | rhs),
            (Self::U16(lhs), Self::U16(rhs)) => Self::U16(lhs | rhs),
            (Self::U32(lhs), Self::U32(rhs)) => Self::U32(lhs | rhs),
            (Self::U64(lhs), Self::U64(rhs)) => Self::U64(lhs | rhs),
            (Self::Usize(lhs), Self::Usize(rhs)) => Self::Usize(lhs | rhs),
            _ => Self::I64(self.as_i64() | rhs.as_i64()),
        }
    }
}

impl PartialEq for Integer {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::I8(l0), Self::I8(r0)) => l0 == r0,
            (Self::I16(l0), Self::I16(r0)) => l0 == r0,
            (Self::I32(l0), Self::I32(r0)) => l0 == r0,
            (Self::I64(l0), Self::I64(r0)) => l0 == r0,
            (Self::Isize(l0), Self::Isize(r0)) => l0 == r0,
            (Self::U8(l0), Self::U8(r0)) => l0 == r0,
            (Self::U16(l0), Self::U16(r0)) => l0 == r0,
            (Self::U32(l0), Self::U32(r0)) => l0 == r0,
            (Self::U64(l0), Self::U64(r0)) => l0 == r0,
            (Self::Usize(l0), Self::Usize(r0)) => l0 == r0,
            _ => self.as_i64() == other.as_i64(),
        }
    }
}

impl PartialOrd for Integer {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.as_i64().partial_cmp(&other.as_i64())
    }
}
