use ::paste::item as paste;
use core::borrow::Borrow;
use core::ops::{Div, Mul};
use std::fmt::Display;

#[derive(Clone)]
struct Fxp {
    val: i32,
}

const SCALING_FACTOR: i32 = 1 << 16;

impl Fxp {
    fn float2fixed(a: &f32) -> Self {
        let val = *a * SCALING_FACTOR as f32;
        Self {
            val: (a * (SCALING_FACTOR as f32)) as i32,
        }
    }
    fn fixed2float(a: &Self) -> f32 {
        a.val as f32 / (SCALING_FACTOR as f32)
    }
}

use ::core;

macro_rules! impl_all_ops {(
    impl $([$($generics:tt)*])?
        $Op:ident Assign?
    for
        &? $T:ty
    {
        fn $op_assign:ident (
            $self:tt : & $('_)? mut Self,
            $other:ident : & $('_)? Self $(,)?
        )
        {
            $($body:tt)*
        }
    }
) => ($crate::paste! {
    const _: () = {
        use $crate::core::ops:: {
            $Op as __Op__,
            [<$Op Assign>] as __OpAssign__,
        };

        impl<__Right__, $($($generics)*)?>
            __OpAssign__<__Right__>
        for
            $T
        where
            __Right__ : $crate::core::borrow::Borrow<$T>,
        {
            fn $op_assign ($self: &'_ mut Self, other: __Right__)
            {
                let $other: &'_ Self = other.borrow();
                $($body)*
            }
        }

        impl<__Right__, $($($generics)*)?>
            __Op__<__Right__>
        for
            $T
        where
            $T : __OpAssign__<__Right__>,
        {
            type Output = Self;

            fn [<$Op:lower>] (mut self: Self, other: __Right__)
              -> Self
            {
                <$T as __OpAssign__<_>>::$op_assign(
                    &mut self,
                    other,
                );
                self
            }
        }

        impl< $($($generics)*)? >
            ::core::ops::$Op<$T>
        for
            &'_ $T
        {
            type Output = $T;

            fn [<$Op:lower>] (
                self: Self,
                other: $T,
            ) -> $T
            {
                let mut this: $T = self.clone();
                <$T as __OpAssign__<_>>::$op_assign(
                    &mut this,
                    other,
                );
                this
            }
        }

        impl<$($($generics)*)?> ::core::ops::$Op for &'_ $T {
            type Output = $T;

            fn [<$Op:lower>] (
                self: Self,
                other: Self,
            ) -> $T
            {
                let mut this: $T = self.clone();
                <$T as __OpAssign__<_>>::$op_assign(
                    &mut this,
                    other,
                );
                this
            }
        }
    };
})}

impl_all_ops! {impl Mul Assign? for &? Fxp {
    fn mul_assign(self: &mut Self, other: &Self)
    {
        self.val = ((self.val as i64 * other.borrow().val as i64) / SCALING_FACTOR as i64) as i32;
    }
}}

impl_all_ops! {impl Div Assign? for &? Fxp {
    fn div_assign(self: &mut Self, other: &Self)
    {
        self.val = ((self.val as i64 * SCALING_FACTOR as i64) / other.borrow().val as i64)  as i32;
    }
}}

impl_all_ops! {impl Add Assign? for &? Fxp {
    fn add_assign(self: &mut Self, other: &Self)
    {
        self.val += other.borrow().val;
    }
}}

impl_all_ops! {impl Sub Assign? for &? Fxp {
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

fn main() {
    let a = Fxp::float2fixed(&1.5);
    let b = Fxp::fixed2float(&Fxp { val: 98304 });
    println!("1.5, float2fixed: {}, fixed2float: {}", a.val, b);

    let a = Fxp::float2fixed(&1.5);
    let b = Fxp::float2fixed(&40.01);
    println!("{} * {} = {}", a, b, (a.clone() * b.clone()));

    let a = Fxp::float2fixed(&1.5);
    let b = Fxp::float2fixed(&40.01);
    println!("{} + {} = {}", a, b, (a.clone() + b.clone()));

    let a = Fxp::float2fixed(&1.5);
    let b = Fxp::float2fixed(&40.01);
    println!("{} - {} = {}", a, b, (a.clone() - b.clone()));

    let a = Fxp::float2fixed(&1.5);
    let b = Fxp::float2fixed(&40.01);
    println!("{} / {} = {}", b, a, (b.clone() / a.clone()));
}
