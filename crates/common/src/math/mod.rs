pub mod cartesian;
pub mod homogeneous;

use core::ops::{Add, Mul, Sub};

pub type Float = Fraction;

use fraction::Fraction;

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

impl<Rhs, T, Out, Temp> DotProduct<(Rhs, Rhs)> for (T, T)
where
    T: Mul<Rhs, Output = Temp>,
    Temp: Add<Output = Out>,
{
    type Output = Out;

    fn dot_product(self, rhs: (Rhs, Rhs)) -> Self::Output {
        let (a1, a2) = self;
        let (b1, b2) = rhs;
        a1 * b1 + a2 * b2
    }
}

#[derive(Debug)]
enum Multiple {
    Mult(Float),
    Zero,
    None,
}

impl PartialEq for Multiple {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Mult(v1), Self::Mult(v2)) => v1 == v2,
            (_, Self::None) | (Self::None, _) => false,
            (Self::Zero, _) | (_, Self::Zero) => true,
        }
    }
}

fn calculate_multiple(lhs: &Float, rhs: &Float) -> Multiple {
    match (lhs == &Float::from(0), rhs == &Float::from(0)) {
        (true, true) => Multiple::Zero,
        (true, false) | (false, true) => Multiple::None,
        (false, false) => Multiple::Mult(lhs / rhs),
    }
}
