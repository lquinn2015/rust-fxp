mod macros;

use core::borrow::Borrow;
use std::fmt::Display;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Fxp {
    pub val: i32,
}

const SCALING_FACTOR: i32 = 1 << 16;

impl Fxp {
    pub fn float2fixed(a: &f32) -> Self {
        Self {
            val: (a * (SCALING_FACTOR as f32)) as i32,
        }
    }
    pub fn fixed2float(a: &Self) -> f32 {
        a.val as f32 / (SCALING_FACTOR as f32)
    }
}

macros::impl_all_ops! {impl Mul Assign? for &? Fxp {
    fn mul_assign(self: &mut Self, other: &Self)
    {
        self.val = ((self.val as i64 * other.borrow().val as i64) / SCALING_FACTOR as i64) as i32;
    }
}}

macros::impl_all_ops! {impl Div Assign? for &? Fxp {
    fn div_assign(self: &mut Self, other: &Self)
    {
        self.val = ((self.val as i64 * SCALING_FACTOR as i64) / other.borrow().val as i64)  as i32;
    }
}}

macros::impl_all_ops! {impl Add Assign? for &? Fxp {
    fn add_assign(self: &mut Self, other: &Self)
    {
        self.val += other.borrow().val;
    }
}}

macros::impl_all_ops! {impl Sub Assign? for &? Fxp {
    fn sub_assign(self: &mut Self, other: &Self)
    {
        self.val -= other.borrow().val;
    }
}}

impl Display for Fxp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} [{}]", Fxp::fixed2float(self), self.val)
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::Fxp;
    #[test]
    fn basic_ops() {
        let a = Fxp::float2fixed(&1.5);
        let b = Fxp::fixed2float(&Fxp { val: 98304 });
        assert_eq!(a, Fxp { val: 98304 });
        assert_eq!(b, 1.5);

        let a = Fxp::float2fixed(&1.5);
        let b = Fxp::float2fixed(&40.01);
        assert_eq!(a * b, Fxp { val: 3933142 });

        let a = Fxp::float2fixed(&1.5);
        let b = Fxp::float2fixed(&40.01);
        assert_eq!(a + b, Fxp { val: 2720399 });

        let a = Fxp::float2fixed(&1.5);
        let b = Fxp::float2fixed(&40.01);
        assert_eq!(a - b, Fxp { val: -2523791 });

        let a = Fxp::float2fixed(&1.5);
        let b = Fxp::float2fixed(&40.01);
        assert_eq!(b / a, Fxp { val: 1748063 });
    }
}
