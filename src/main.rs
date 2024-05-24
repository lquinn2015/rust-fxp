mod macros;

use core::borrow::Borrow;
use std::{fmt::Display, ops::Shl, ops::Shr};

#[derive(Clone, PartialEq, Eq, Debug, PartialOrd)]
pub struct Fxp {
    pub val: i32,
}

const SCALING_FACTOR: i32 = 1 << 16;

impl Fxp {
    pub fn new(a: i32) -> Self {
        Self { val: a }
    }
    pub const fn cnew(a: i32) -> Self {
        Self { val: a }
    }
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

macros::impl_all_ops! {impl Shl Assign? for &? Fxp {
    fn shl_assign(self: &mut Self, other: &Self)
    {
        self.val <<= other.borrow().val;
    }
}}

macros::impl_all_ops! {impl Shr Assign? for &? Fxp {
    fn shr_assign(self: &mut Self, other: &Self)
    {
        self.val <<= other.borrow().val;
    }
}}

impl Shl<i32> for Fxp {
    type Output = Fxp;
    fn shl(self, rhs: i32) -> Self::Output {
        Fxp {
            val: self.val << rhs,
        }
    }
}

impl Shr<i32> for Fxp {
    type Output = Fxp;
    fn shr(self, rhs: i32) -> Self::Output {
        Fxp {
            val: self.val >> rhs,
        }
    }
}

impl Display for Fxp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} [{}]", Fxp::fixed2float(self), self.val)
    }
}

const COS_PROD: Fxp = Fxp::cnew(39796);
const COS_PROD2: Fxp = Fxp::cnew(24166);
const CORDIC_ANGLES: [Fxp; 16] = [
    Fxp::cnew(51471),
    Fxp::cnew(30385),
    Fxp::cnew(16054),
    Fxp::cnew(8149),
    Fxp::cnew(4090),
    Fxp::cnew(2047),
    Fxp::cnew(1023),
    Fxp::cnew(511),
    Fxp::cnew(255),
    Fxp::cnew(127),
    Fxp::cnew(63),
    Fxp::cnew(31),
    Fxp::cnew(16),
    Fxp::cnew(8),
    Fxp::cnew(4),
    Fxp::cnew(2),
];

fn cordic_iter(
    theta: &Fxp,
    x0: &Fxp,
    y0: &Fxp,
    phi0: &Fxp,
    phi_i: &Fxp,
    i: i32,
) -> (Fxp, Fxp, Fxp, i32) {
    if phi0 < theta {
        let xi = x0 - (y0.clone() >> i);
        let yi = y0 + (x0.clone() >> i);
        (xi, yi, phi0 + phi_i, i + 1)
    } else {
        let xi = x0 + (y0.clone() >> i);
        let yi = y0 - (x0.clone() >> i);
        (xi, yi, phi0 - phi_i, i + 1)
    }
}

fn cordic(theta: Fxp) -> (Fxp, Fxp) {
    let (cos_theta, sin_theta, _phi0, _i) = CORDIC_ANGLES.iter().fold(
        (Fxp::cnew(65536), Fxp::cnew(0), Fxp::cnew(0), 0),
        |(x0, y0, phi0, i), phi_i| cordic_iter(&theta, &x0, &y0, &phi0, phi_i, i),
    );
    (cos_theta * COS_PROD, sin_theta * COS_PROD)
}

fn cordic2(theta: Fxp) -> (Fxp, Fxp) {
    let (cos_theta, sin_theta, _phi0, _i) = CORDIC_ANGLES.iter().fold(
        (Fxp::cnew(65536), Fxp::cnew(0), Fxp::cnew(0), 0),
        |(x0, y0, phi0, i), phi_i| {
            let out = cordic_iter(&theta, &x0, &y0, &phi0, phi_i, i);
            cordic_iter(&theta, &out.0, &out.1, &out.2, phi_i, i)
        },
    );
    (cos_theta * COS_PROD2, sin_theta * COS_PROD2)
}

fn main() {
    let (cos, sin) = cordic(Fxp::float2fixed(&0.5));
    println!(
        "cos(0.5) = {}, sin(0.5) = {}, tan(0.5) = {}",
        cos,
        sin,
        &sin / &cos
    );
    let (cos, sin) = cordic2(Fxp::float2fixed(&2.0));
    println!(
        "cos(2.0) = {}, sin(2.0) = {}, tan(2.0) = {}",
        cos,
        sin,
        &sin / &cos
    );
    //println!("{}", Fxp::float2fixed(&1.0) >> 2);
}

#[cfg(test)]
mod tests {
    use crate::*;
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

    #[test]
    fn cordic_test() {
        let (cos, sin) = cordic(Fxp::float2fixed(&0.5));
        assert_eq!(cos, Fxp::cnew(57510));
        assert_eq!(sin, Fxp::cnew(31422));
        let (cos, sin) = cordic2(Fxp::float2fixed(&2.0));
        assert_eq!(cos, Fxp::cnew(-27274));
        assert_eq!(sin, Fxp::cnew(59590));
        let (cos2, sin2) = cordic2(Fxp::float2fixed(&-2.0));
        // Delta should be tiny but could accumulate
        assert!((cos2 - cos).val < 10);
        assert!((sin + sin2).val < 10);
    }
}
