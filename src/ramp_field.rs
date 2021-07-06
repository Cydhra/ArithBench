use ramp::Int;
pub use num::FromPrimitive;
use std::ops::{Add, Mul};
use once_cell::sync::Lazy;

static FIELD_PRIME: Lazy<Int> = Lazy::new(|| {
    Int::from_str_radix("618970019642690137449562111", 10).unwrap()
});

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct RampField(Int);

impl From<usize> for RampField {
    fn from(n: usize) -> Self {
        RampField(Int::from(n) % FIELD_PRIME.clone())
    }
}

impl FromPrimitive for RampField {
    fn from_i64(n: i64) -> Option<Self> {
        Some(RampField(Int::from(n) % FIELD_PRIME.clone()))
    }

    fn from_u64(n: u64) -> Option<Self> {
        Some(RampField(Int::from(n) % FIELD_PRIME.clone()))
    }
}

impl Add for RampField {
    type Output = RampField;

    fn add(self, rhs: Self) -> Self::Output {
        RampField((self.0 + rhs.0) % FIELD_PRIME.clone())
    }
}

impl Mul for RampField {
    type Output = RampField;

    fn mul(self, rhs: Self) -> Self::Output {
        RampField((self.0 * rhs.0) % FIELD_PRIME.clone())
    }
}