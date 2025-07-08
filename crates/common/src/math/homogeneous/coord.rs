use snafu::Snafu;

use crate::math::{
    CrossProduct, Float, calculate_multiple, cartesian::CartesianCoord,
    homogeneous::HomogeneousLine,
};

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(kani, derive(kani::Arbitrary))]
pub struct Coord {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

impl PartialEq for Coord {
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

impl Coord {
    pub fn new(x: impl Into<Float>, y: impl Into<Float>, z: impl Into<Float>) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }

    #[must_use]
    pub const fn is_finite(&self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }

    #[must_use]
    pub fn is_at_infinite(self) -> bool {
        self.z == 0.into()
    }

    #[must_use]
    pub const fn tuple(&self) -> (&Float, &Float, &Float) {
        (&self.x, &self.y, &self.z)
    }

    #[must_use]
    pub const fn array(&self) -> [&Float; 3] {
        [&self.x, &self.y, &self.z]
    }

    ///
    /// # Errors
    /// [`PointAtInfinity`] if `z` is 0.0
    #[allow(clippy::op_ref)]
    pub fn cartesian(&self) -> Result<CartesianCoord, PointAtInfinity> {
        if self.z == 0.into() {
            return PointAtInfinitySnafu.fail();
        }
        Ok(CartesianCoord::new(&self.x / &self.z, &self.y / &self.z))
    }

    #[must_use]
    pub fn line(&self, other: &Self) -> HomogeneousLine {
        self.tuple().cross_product(other.tuple()).into()
    }
}

impl<TX: Into<Float>, TY: Into<Float>, TZ: Into<Float>> From<(TX, TY, TZ)> for Coord {
    fn from((x, y, z): (TX, TY, TZ)) -> Self {
        Self::new(x, y, z)
    }
}

impl<TX: Into<Float>, TY: Into<Float>> From<(TX, TY)> for Coord {
    fn from((x, y): (TX, TY)) -> Self {
        Self::new(x, y, 1)
    }
}

impl<T: Into<Float>> From<[T; 3]> for Coord {
    fn from([x, y, z]: [T; 3]) -> Self {
        Self::new(x, y, z)
    }
}
impl<T: Into<Float>> From<[T; 2]> for Coord {
    fn from([x, y]: [T; 2]) -> Self {
        Self::new(x, y, 1)
    }
}

impl From<CartesianCoord> for Coord {
    fn from(value: CartesianCoord) -> Self {
        Self::new(value.x, value.y, 1)
    }
}
impl From<&CartesianCoord> for Coord {
    fn from(value: &CartesianCoord) -> Self {
        Self::from(value.clone())
    }
}

impl CrossProduct for Coord {
    type Output = HomogeneousLine;

    fn cross_product(self, rhs: Self) -> Self::Output {
        self.tuple().cross_product(rhs.tuple()).into()
    }
}

#[derive(Debug, Snafu, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[snafu(display("point at infinity"))]
pub struct PointAtInfinity;
