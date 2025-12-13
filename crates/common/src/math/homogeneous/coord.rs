// Based on Geometriekalküle from  Jürgen Richter-Gebert, Thorsten Orendt https://doi.org/10.1007/978-3-642-02530-3

use core::fmt::Debug;

use snafu::Snafu;

use crate::math::{
    A, CrossProduct, calculate_multiple, cartesian::CartesianCoord, homogeneous::HomogeneousLine,
};

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(kani, derive(kani::Arbitrary))]
pub struct Coord<T: A> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: A> PartialEq for Coord<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self.cartesian(), other.cartesian()) {
            (Ok(l), Ok(r)) => l.eq(&r),
            (Err(_), Err(_)) => {
                let x = calculate_multiple(&self.x, &other.x);
                let y = calculate_multiple(&self.y, &other.y);
                let z = calculate_multiple(&self.z, &other.z);
                x == y && y == z
            }
            _ => false,
        }
    }
}

impl<T: A> Coord<T> {
    pub fn new(x: impl Into<T>, y: impl Into<T>, z: impl Into<T>) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }
    #[must_use]
    pub const fn tuple(&self) -> (&T, &T, &T) {
        (&self.x, &self.y, &self.z)
    }

    #[must_use]
    pub fn tuple_owned(&self) -> (T, T, T) {
        (self.x.clone(), self.y.clone(), self.z.clone())
    }

    #[must_use]
    pub const fn array(&self) -> [&T; 3] {
        [&self.x, &self.y, &self.z]
    }

    #[must_use]
    pub fn is_at_infinite(self) -> bool {
        self.z.is_zero()
    }
}

impl<T: A> Coord<T> {
    ///
    /// # Errors
    /// [`PointAtInfinity`] if `z` is 0.0
    #[allow(clippy::op_ref)]
    pub fn cartesian(&self) -> Result<CartesianCoord<T>, PointAtInfinity> {
        if self.z.is_zero() {
            return PointAtInfinitySnafu.fail();
        }
        Ok(CartesianCoord::new(
            self.x.clone() / &self.z,
            self.y.clone() / &self.z,
        ))
    }
}

impl<T: A> Coord<T> {
    #[must_use]
    pub fn line(&self, other: &Self) -> HomogeneousLine<T> {
        self.tuple_owned().cross_product(other.tuple()).into()
    }
}

impl<T: A, TX: Into<T>, TY: Into<T>, TZ: Into<T>> From<(TX, TY, TZ)> for Coord<T> {
    fn from((x, y, z): (TX, TY, TZ)) -> Self {
        Self::new(x, y, z)
    }
}

impl<T: A, TX: Into<T>, TY: Into<T>> From<(TX, TY)> for Coord<T> {
    fn from((x, y): (TX, TY)) -> Self {
        Self::new(x, y, 1)
    }
}

impl<TT: A, T: Into<TT>> From<[T; 3]> for Coord<TT> {
    fn from([x, y, z]: [T; 3]) -> Self {
        Self::new(x, y, z)
    }
}
impl<TT: A, T: Into<TT>> From<[T; 2]> for Coord<TT> {
    fn from([x, y]: [T; 2]) -> Self {
        Self::new(x, y, 1)
    }
}

impl<T: A> From<CartesianCoord<T>> for Coord<T> {
    fn from(value: CartesianCoord<T>) -> Self {
        Self::new(value.x, value.y, 1)
    }
}
impl<T: A> From<&CartesianCoord<T>> for Coord<T> {
    fn from(value: &CartesianCoord<T>) -> Self {
        Self::from(value.clone())
    }
}

impl<T: A> CrossProduct for Coord<T> {
    type Output = HomogeneousLine<T>;

    fn cross_product(self, rhs: Self) -> Self::Output {
        self.tuple_owned().cross_product(rhs.tuple()).into()
    }
}

#[derive(Debug, Snafu, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[snafu(display("point at infinity"))]
pub struct PointAtInfinity;
