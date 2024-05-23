#[macro_export]
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
) => (
    const _: () = {
    use ::paste::item as paste;

    paste! {
        const _: () = {
            use ::core::ops:: {
                $Op as __Op__,
                [<$Op Assign>] as __OpAssign__,
            };

            impl<__Right__, $($($generics)*)?>
                __OpAssign__<__Right__>
            for
                $T
            where
                __Right__ : ::core::borrow::Borrow<$T>,
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
    };
};)}

pub(crate) use impl_all_ops;
