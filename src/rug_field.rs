pub use num::FromPrimitive;
use std::ops::{Add, Mul, Deref};
use once_cell::sync::Lazy;
use rug::Integer;

static FIELD_PRIME: Lazy<Integer> = Lazy::new(|| {
    Integer::parse_radix("618970019642690137449562111", 10).unwrap()
});

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct RugField(Integer);

impl From<usize> for RugField {
    fn from(n: usize) -> Self {
        RugField(Int::from(n) % FIELD_PRIME.deref())
    }
}

impl Add for RugField {
    type Output = RugField;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.0 += rhs.0;
        self.0 %= FIELD_PRIME.deref();
        self
    }
}

impl Mul for RugField {
    type Output = RugField;

    fn mul(mut self, rhs: Self) -> Self::Output {
        self.0 *= rhs.0;
        self.0 %= FIELD_PRIME.deref();
        self
    }
}


impl Add<&RugField> for RugField {
    type Output = RugField;

    fn add(mut self, rhs: &Self) -> Self::Output {
        self.0 += &rhs.0;
        self.0 %= FIELD_PRIME.deref();
        self
    }
}

impl Mul<&RugField> for RugField {
    type Output = RugField;

    fn mul(mut self, rhs: &Self) -> Self::Output {
        self.0 *= &rhs.0;
        self.0 %= FIELD_PRIME.deref();
        self
    }
}