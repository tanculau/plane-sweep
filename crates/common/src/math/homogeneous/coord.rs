use float_cmp::approx_eq;
use snafu::Snafu;

use crate::{
    f_eq, impl_approx_eq,
    math::{
        CrossProduct, Distance, Float, cartesian::CartesianCoord, homogeneous::HomogeneousLine,
    },
};

#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(kani, derive(kani::Arbitrary))]
pub struct Coord {
    pub x: Float,
    pub y: Float,
    pub z: Float,
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
    pub const fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }

    #[must_use]
    pub fn is_at_infinite(self) -> bool {
        approx_eq!(Float, self.z, 0.0)
    }

    #[must_use]
    pub const fn tuple(self) -> (Float, Float, Float) {
        (self.x, self.y, self.z)
    }

    #[must_use]
    pub const fn array(self) -> [Float; 3] {
        [self.x, self.y, self.z]
    }

    ///
    /// # Errors
    /// [`PointAtInfinity`] if `z` is 0.0
    pub fn cartesian(self) -> Result<CartesianCoord, PointAtInfinity> {
        if approx_eq!(Float, self.z, 0.0) {
            return PointAtInfinitySnafu.fail();
        }
        Ok(CartesianCoord::new(self.x / self.z, self.y / self.z))
    }

    #[must_use]
    pub fn line(self, other: Self) -> HomogeneousLine {
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
        Self::new(x, y, 1.0_f64)
    }
}

impl<T: Into<Float>> From<[T; 3]> for Coord {
    fn from([x, y, z]: [T; 3]) -> Self {
        Self::new(x, y, z)
    }
}
impl<T: Into<Float>> From<[T; 2]> for Coord {
    fn from([x, y]: [T; 2]) -> Self {
        Self::new(x, y, 1.0_f64)
    }
}

impl From<CartesianCoord> for Coord {
    fn from(value: CartesianCoord) -> Self {
        value.tuple().into()
    }
}
impl From<&CartesianCoord> for Coord {
    fn from(value: &CartesianCoord) -> Self {
        value.tuple().into()
    }
}

impl Distance for Coord {
    type Output = Float;

    fn distance(self, rhs: Self) -> Self::Output {
        let Self {
            x: x1,
            y: y1,
            z: z1,
        } = self;
        let Self {
            x: x2,
            y: y2,
            z: z2,
        } = rhs;
        let x = x1 - x2;
        let y = y1 - y2;
        let z = z1 - z2;

        x.mul_add(x, y.mul_add(y, (z).powi(2))).sqrt()
    }
}

impl Distance<CartesianCoord> for Coord {
    type Output = Float;

    fn distance(self, rhs: CartesianCoord) -> Self::Output {
        self.cartesian()
            .map_or(Float::INFINITY, |v| v.distance(rhs))
    }
}

impl_approx_eq!(Coord, |l, r, margin| {
    match (l.cartesian(), r.cartesian()) {
        (Ok(v1), Ok(v2)) => v1.approx_eq(v2, margin),
        (Ok(_), Err(_)) | (Err(_), Ok(_)) => false,
        (Err(_), Err(_)) => {
            let Coord { x: x1, y: y1, .. } = *l;
            let Coord { x: x2, y: y2, .. } = *r;
            if f_eq!(x1, 0.0) && f_eq!(y1, 0.0) {
                return f_eq!(x2, 0.0) && f_eq!(y2, 0.0);
            }
            let x_ratio = x1 / x2;
            let y_ratio = y1 / y2;
            let x_zero = f_eq!(x1, 0.0) && f_eq!(x2, 0.0);
            let y_zero = f_eq!(y1, 0.0) && f_eq!(y2, 0.0);
            if x_ratio.is_nan() && !(x_zero) {
                return false;
            }
            if y_ratio.is_nan() && !(y_zero) {
                return false;
            }
            match (x_zero, y_zero) {
                (true | false, true) | (true, false) => true,
                (false, false) => f_eq!(x_ratio, y_ratio),
            }
        }
    }
});

impl CrossProduct for Coord {
    type Output = HomogeneousLine;

    fn cross_product(self, rhs: Self) -> Self::Output {
        self.tuple().cross_product(rhs.tuple()).into()
    }
}

#[derive(Debug, Snafu, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[snafu(display("point at infinity"))]
pub struct PointAtInfinity;
