// Based on Geometriekalküle from  Jürgen Richter-Gebert, Thorsten Orendt https://doi.org/10.1007/978-3-642-02530-3

pub mod cartesian;
pub mod homogeneous;

use core::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::{Add, Div, Mul, Neg, Sub},
};

#[cfg(feature = "rational")]
pub use malachite::rational::Rational;
#[cfg(feature = "float")]
pub use ordered_float::OrderedFloat;

use crate::{math::homogeneous::HomogeneousLine, segment::Segment};

#[cfg(feature = "float")]
pub type Float = ordered_float::OrderedFloat<f64>;

pub fn abs<T: A>(v: T) -> T {
    if v < 0.into() { v.neg() } else { v }
}

pub trait CrossProduct<Rhs = Self> {
    type Output;

    fn cross_product(self, rhs: Rhs) -> Self::Output;
}

impl<T: A> CrossProduct<(&T, &T, &T)> for (T, T, T) {
    type Output = (T, T, T);

    fn cross_product(self, rhs: (&T, &T, &T)) -> Self::Output {
        let (a1, a2, a3) = self;
        let (b1, b2, b3) = rhs;

        (
            a2.clone() * b3 - &(a3.clone() * b2),
            a3 * b1 - &(a1.clone() * b3),
            a1 * b2 - &(a2 * b1),
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
impl<T: A> DotProduct<(&T, &T, &T)> for (T, T, T) {
    type Output = T;

    fn dot_product(self, rhs: (&T, &T, &T)) -> Self::Output {
        let (a1, a2, a3) = self;
        let (b1, b2, b3) = rhs;
        a1 * b1 + &(a2 * b2) + &(a3 * b3)
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
enum Multiple<T> {
    Mult(T),
    Zero,
    None,
}

impl<T: PartialEq> PartialEq for Multiple<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Mult(v1), Self::Mult(v2)) => v1 == v2,
            (_, Self::None) | (Self::None, _) => false,
            (Self::Zero, _) | (_, Self::Zero) => true,
        }
    }
}

fn calculate_multiple<Lhs: A>(lhs: &Lhs, rhs: &Lhs) -> Multiple<Lhs> {
    match (lhs.is_zero(), rhs.is_zero()) {
        (true, true) => Multiple::Zero,
        (true, false) | (false, true) => Multiple::None,
        (false, false) => Multiple::Mult(lhs.clone() / rhs),
    }
}

pub trait IsZero {
    /// Returns `true` if `self` is equal to the additive identity.
    fn is_zero(&self) -> bool;
}

#[cfg(feature = "float")]
impl IsZero for ordered_float::OrderedFloat<f64> {
    fn is_zero(&self) -> bool {
        self == &Self::from(0_u8)
    }
}
#[cfg(feature = "float")]
impl IsZero for ordered_float::OrderedFloat<f32> {
    fn is_zero(&self) -> bool {
        self == &Self::from(0_u8)
    }
}

impl IsZero for f64 {
    fn is_zero(&self) -> bool {
        self.abs() == 0.0
    }
}
impl IsZero for f32 {
    fn is_zero(&self) -> bool {
        self.abs() == 0.0
    }
}

#[cfg(feature = "rational")]
impl IsZero for malachite::rational::Rational {
    fn is_zero(&self) -> bool {
        *self == Self::const_from_unsigned(0)
    }
}

impl<T: IsZero> IsZero for &T {
    fn is_zero(&self) -> bool {
        (*self).is_zero()
    }
}

pub trait Sqrt {
    #[must_use]
    fn sqrt2(self) -> Self;
}

#[cfg(feature = "float")]
impl Sqrt for ordered_float::OrderedFloat<f64> {
    fn sqrt2(self) -> Self {
        (*self).sqrt().into()
    }
}
#[cfg(feature = "float")]
impl Sqrt for ordered_float::OrderedFloat<f32> {
    fn sqrt2(self) -> Self {
        self.sqrt().into()
    }
}

impl Sqrt for f64 {
    fn sqrt2(self) -> Self {
        self.sqrt()
    }
}
impl Sqrt for f32 {
    fn sqrt2(self) -> Self {
        self.sqrt()
    }
}

pub trait NumOps<Rhs = Self, Output = Self>:
    Add<Rhs, Output = Output>
    + Sub<Rhs, Output = Output>
    + Mul<Rhs, Output = Output>
    + Div<Rhs, Output = Output>
{
}

impl<T, Rhs, Output> NumOps<Rhs, Output> for T where
    T: Add<Rhs, Output = Output>
        + Sub<Rhs, Output = Output>
        + Mul<Rhs, Output = Output>
        + Div<Rhs, Output = Output>
{
}

pub trait NumRef: Sized + for<'r> NumOps<&'r Self> {}
impl<T> NumRef for T where T: for<'r> NumOps<&'r T> {}

pub trait B: A + Copy + Sqrt {}

impl<T: A + Copy + Sqrt> B for T {}

pub trait A:
    NumRef
    + NumOps<Self, Self>
    + Clone
    + Display
    + Debug
    + PartialEq
    + Eq
    + PartialOrd
    + Ord
    + ToFloat
    + Neg<Output = Self>
    + Default
    + From<u8>
    + From<i8>
    + From<u16>
    + From<i16>
    + From<u32>
    + From<i32>
    + IsZero
    + Hash
{
}

impl<T> A for T where
    T: NumOps
        + Clone
        + NumRef
        + Display
        + Debug
        + PartialEq
        + Eq
        + PartialOrd
        + Ord
        + ToFloat
        + Neg<Output = T>
        + Default
        + From<u8>
        + From<i8>
        + From<u16>
        + From<i16>
        + From<u32>
        + From<i32>
        + IsZero
        + NumOps<Self, Self>
        + Hash
{
}

pub trait FloatSuper:
    IsZero
    + From<u8>
    + From<i8>
    + From<isize>
    + Clone
    + Display
    + Debug
    + PartialEq
    + PartialOrd
    + Ord
    + Default
    + ToFloat
    + for<'a> Add<&'a Self, Output = Self>
    + for<'a> Sub<&'a Self, Output = Self>
    + for<'a> Mul<&'a Self, Output = Self>
    + for<'a> Div<&'a Self, Output = Self>
    + for<'a> Neg<Output = Self>
{
}

impl<T> FloatSuper for T where
    Self: IsZero
        + From<u8>
        + From<i8>
        + From<isize>
        + Clone
        + Display
        + Debug
        + PartialEq
        + PartialOrd
        + Ord
        + Default
        + ToFloat
        + for<'a> Add<&'a Self, Output = Self>
        + for<'a> Sub<&'a Self, Output = Self>
        + for<'a> Mul<&'a Self, Output = Self>
        + for<'a> Div<&'a Self, Output = Self>
        + for<'a> Neg<Output = Self>
{
}

pub trait ToFloat {
    fn to_float(&self) -> f64;
}

#[cfg(feature = "float")]
impl ToFloat for ordered_float::OrderedFloat<f64> {
    fn to_float(&self) -> f64 {
        **self
    }
}
#[cfg(feature = "float")]
impl ToFloat for ordered_float::OrderedFloat<f32> {
    fn to_float(&self) -> f64 {
        f64::from(**self)
    }
}

impl ToFloat for f64 {
    fn to_float(&self) -> f64 {
        *self
    }
}
impl ToFloat for f32 {
    fn to_float(&self) -> f64 {
        f64::from(*self)
    }
}

#[cfg(feature = "rational")]
impl ToFloat for malachite::rational::Rational {
    fn to_float(&self) -> f64 {
        use malachite::base::num::conversion::traits::RoundingFrom;
        RoundingFrom::rounding_from(self, malachite::base::rounding_modes::RoundingMode::Nearest).0
    }
}

impl<T: ToFloat> ToFloat for &T {
    fn to_float(&self) -> f64 {
        (*self).to_float()
    }
}

pub fn find_circle<T: A + Copy + Sqrt>(
    x1: T,
    y1: T,
    x2: T,
    y2: T,
    x3: T,
    y3: T,
) -> Option<(T, T, T)> {
    let s1 = Segment::<T>::new((x1, y1), (x2, y2)).pedendicular_bisector();
    let s2 = Segment::<T>::new((x2, y2), (x3, y3)).pedendicular_bisector();

    let center = s1.intersection(s2).cartesian().ok()?;
    let dx = center.x - x1;
    let dy = center.y - y1;
    let rad = (dx * dx + dy * dy).sqrt2();
    Some((center.x, center.y, rad))
}

pub fn bisector<T: A + Copy>(x1: T, y1: T, x2: T, y2: T) -> HomogeneousLine<T> {
    let line = Segment::<T>::new((x1, y1), (x2, y2)).line();
    let a = line.b;
    let b = -line.a;
    let (xm, ym) = mid_point(x1, y1, x2, y2);
    let c = -a * xm - b * ym;
    HomogeneousLine::new(a, b, c)
}

pub fn mid_point<T: A + Copy>(x1: T, y1: T, x2: T, y2: T) -> (T, T) {
    (x1 + (x2 - x1) / T::from(2), y1 + (y2 - y1) / T::from(2))
}

// Copied from:
// Original Code:
// https://github.com/gkaemmer/voronoi-rs
#[allow(clippy::suspicious_operation_groupings)]
pub fn breakpoint_between<T: A + Sqrt + Copy>(x1: T, y1: T, x2: T, y2: T, s: T) -> T {
    if y1 == y2 {
        return x1 + (x2 - x1) / T::from(2);
    }

    let sqrt = -((s * s - s * y1 - s * y2 + y1 * y2)
        * (x1 * x1 - T::from(2) * x1 * x2 + x2 * x2 + y1 * y1 - T::from(2) * y1 * y2 + y2 * y2))
        .sqrt2();
    (-sqrt + s * x1 - s * x2 - x1 * y2 + x2 * y1) / (y1 - y2)
}

// End of Copy
