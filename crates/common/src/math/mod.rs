pub mod cartesian;
pub mod homogeneous;

use core::ops::{Add, Mul, Sub};

pub type Float = f64;

pub use float_cmp;

#[macro_export]
macro_rules! f_eq {
    ($lhs:expr, $rhs:expr) => {
        $crate::math::float_cmp::approx_eq!($crate::math::Float, $lhs, $rhs)
    };
}

#[macro_export]
macro_rules! impl_approx_eq {
    ($t : ty, $m: ty, $i : expr) => {
        impl float_cmp::ApproxEq for &$t {
            type Margin = $m;

            fn approx_eq<T: Into<Self::Margin>>(self, other: Self, margin: T) -> bool {
                fn calculate(
                    fun: impl FnOnce(&$t, &$t, $m) -> bool,
                    a: &$t,
                    b: &$t,
                    margin: $m,
                ) -> bool {
                    fun(a, b, margin)
                }

                let margin = margin.into();
                calculate($i, self, other, margin)
            }
        }

        impl float_cmp::ApproxEq for $t {
            type Margin = $m;

            fn approx_eq<T: Into<Self::Margin>>(self, other: Self, margin: T) -> bool {
                use float_cmp::ApproxEq;
                <&$t as ApproxEq>::approx_eq(&self, &other, margin)
            }
        }
        impl float_cmp::ApproxEq for &mut $t {
            type Margin = $m;

            fn approx_eq<T: Into<Self::Margin>>(self, other: Self, margin: T) -> bool {
                use float_cmp::ApproxEq;
                <&$t as ApproxEq>::approx_eq(&self, &other, margin)
            }
        }
    };
    ($t : ty, $i : expr) => {
        impl_approx_eq!($t, float_cmp::F64Margin, $i);
    };
}

pub trait CrossProduct<Rhs = Self> {
    type Output;

    fn cross_product(self, rhs: Rhs) -> Self::Output;
}

impl<T, K, L, M> CrossProduct<(M, M, M)> for (T, T, T)
where
    T: Mul<M, Output = K> + Clone,
    K: Sub<Output = L>,
    M: Clone,
{
    type Output = (L, L, L);

    fn cross_product(self, rhs: (M, M, M)) -> Self::Output {
        let (a1, a2, a3) = self;
        let (b1, b2, b3) = rhs;
        (
            a2.clone() * b3.clone() - a3.clone() * b2.clone(),
            a3 * b1.clone() - a1.clone() * b3,
            a1 * b2 - a2 * b1,
        )
    }
}

pub trait Distance<Rhs = Self> {
    type Output;
    fn distance(self, rhs: Rhs) -> Self::Output;
}

pub trait DotProduct<Rhs = Self> {
    type Output;
    fn dot_product(self, rhs: Rhs) -> Self::Output;
}
impl<Rhs, T, Out, Temp, Temp2> DotProduct<(Rhs, Rhs, Rhs)> for (T, T, T)
where
    T: Mul<Rhs, Output = Temp>,
    Temp: Add<Output = Temp2>,
    Temp2: Add<Temp, Output = Out>,
{
    type Output = Out;

    fn dot_product(self, rhs: (Rhs, Rhs, Rhs)) -> Self::Output {
        let (a1, a2, a3) = self;
        let (b1, b2, b3) = rhs;
        a1 * b1 + a2 * b2 + a3 * b3
    }
}
