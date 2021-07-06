use ramp::Int;
use std::ops::{Add, Mul, Deref};
use once_cell::sync::Lazy;

static FIELD_PRIME: Lazy<Int> = Lazy::new(|| {
    Int::from_str_radix("618970019642690137449562111", 10).unwrap()
});

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct RampField(Int);

impl From<usize> for RampField {
    fn from(n: usize) -> Self {
        RampField(Int::from(n) % FIELD_PRIME.deref())
    }
}

impl Add for RampField {
    type Output = RampField;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.0 += rhs.0;
        self.0 %= FIELD_PRIME.deref();
        self
    }
}

impl Mul for RampField {
    type Output = RampField;

    fn mul(mut self, rhs: Self) -> Self::Output {
        self.0 *= rhs.0;
        self.0 %= FIELD_PRIME.deref();
        self
    }
}


impl Add<&RampField> for RampField {
    type Output = RampField;

    fn add(mut self, rhs: &Self) -> Self::Output {
        self.0 += &rhs.0;
        self.0 %= FIELD_PRIME.deref();
        self
    }
}

impl Mul<&RampField> for RampField {
    type Output = RampField;

    fn mul(mut self, rhs: &Self) -> Self::Output {
        self.0 *= &rhs.0;
        self.0 %= FIELD_PRIME.deref();
        self
    }
}