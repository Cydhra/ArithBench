pub use num::{FromPrimitive, BigUint, Num};
use std::ops::{Add, Mul};
use once_cell::sync::Lazy;

static FIELD_PRIME: Lazy<BigUint> = Lazy::new(|| {
    BigUint::from_str_radix("618970019642690137449562111", 10).unwrap()
});

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct NumField(BigUint);

impl From<usize> for NumField {
    fn from(n: usize) -> Self {
        NumField(BigUint::from(n) % FIELD_PRIME.clone())
    }
}

impl FromPrimitive for NumField {
    fn from_i64(n: i64) -> Option<Self> {
        BigUint::from_i64(n).map(|n| NumField(n % FIELD_PRIME.clone()))
    }

    fn from_u64(n: u64) -> Option<Self> {
        BigUint::from_u64(n).map(|n| NumField(n % FIELD_PRIME.clone()))
    }
}

impl Add for NumField {
    type Output = NumField;

    fn add(self, rhs: Self) -> Self::Output {
        NumField((self.0 + rhs.0) % FIELD_PRIME.clone())
    }
}

impl Mul for NumField {
    type Output = NumField;

    fn mul(self, rhs: Self) -> Self::Output {
        NumField((self.0 * rhs.0) % FIELD_PRIME.clone())
    }
}