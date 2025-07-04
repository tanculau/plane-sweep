use core::ops::Neg;

use tracing::{debug, instrument};

use crate::math::{
    CrossProduct, DotProduct, Float, calculate_multiple, homogeneous::HomogeneousCoord,
};

#[derive(Debug, Clone, PartialOrd)]
pub struct Line {
    pub a: Float,
    pub b: Float,
    pub c: Float,
}

impl Neg for Line {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            a: -self.a,
            b: -self.b,
            c: -self.c,
        }
    }
}

impl PartialEq for Line {
    fn eq(&self, other: &Self) -> bool {
        let a = calculate_multiple(&self.a, &other.a);
        let b = calculate_multiple(&self.b, &other.b);
        let c = calculate_multiple(&self.c, &other.c);
        a == b && b == c
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Slope {
    ThirdQuadrant(Float),
    Vertical,
    FourthQuadrant(Float),
    Horizontal,
    Infinity,
}

impl Line {
    #[must_use]
    pub fn x_axis() -> Self {
        Self {
            a: 0.into(),
            b: 1.into(),
            c: 0.into(),
        }
    }
    #[must_use]
    pub fn y_axis() -> Self {
        Self {
            a: 1.into(),
            b: 0.into(),
            c: 0.into(),
        }
    }
    pub fn new(a: impl Into<Float>, b: impl Into<Float>, c: impl Into<Float>) -> Self {
        Self {
            a: a.into(),
            b: b.into(),
            c: c.into(),
        }
    }

    pub fn horizontal(y: impl Into<Float>) -> Self {
        Self::new(0, -1, y.into())
    }

    pub fn vertical(x: impl Into<Float>) -> Self {
        Self::new(-1, 0, x.into())
    }

    #[must_use]
    pub const fn is_finite(&self) -> bool {
        self.a.is_finite() && self.b.is_finite() && self.c.is_finite()
    }

    #[must_use]
    pub const fn tuple(&self) -> (&Float, &Float, &Float) {
        (&self.a, &self.b, &self.c)
    }

    #[must_use]
    pub const fn array(&self) -> [&Float; 3] {
        [&self.a, &self.b, &self.c]
    }

    #[instrument(name = "Line::contains_coord", skip(self, coord))]
    pub fn contains_coord(self, coord: impl Into<HomogeneousCoord>) -> bool {
        let coord = coord.into();
        let res = self.tuple().dot_product(coord.tuple());
        let res = res == 0.into();
        debug!("Line contains coord: {coord:?} on line {self:?} -> {res}");
        res
    }

    #[must_use]
    pub fn intersection(self, other: Self) -> HomogeneousCoord {
        self.cross_product(other)
    }

    #[must_use]
    pub fn slope(self) -> Slope {
        match ((self.a == 0.into()), (self.b == 0.into())) {
            (true, true) => Slope::Infinity,
            (true, false) => Slope::Horizontal,
            (false, true) => Slope::Vertical,
            (false, false) => {
                let slope = -self.a / self.b;

                if slope.is_sign_positive() {
                    Slope::ThirdQuadrant((slope).into())
                } else {
                    Slope::FourthQuadrant(slope.into())
                }
            }
        }
    }

    #[must_use]
    pub fn angle(self) -> Float {
        let Self { a: a1, b: b1, .. } = self;
        -a1 / b1
    }
}

impl<TA: Into<Float>, TB: Into<Float>, TC: Into<Float>> From<(TA, TB, TC)> for Line {
    fn from((a, b, c): (TA, TB, TC)) -> Self {
        Self {
            a: a.into(),
            b: b.into(),
            c: c.into(),
        }
    }
}

impl CrossProduct for Line {
    type Output = HomogeneousCoord;

    fn cross_product(self, rhs: Self) -> Self::Output {
        let result = self.tuple().cross_product(rhs.tuple()).into();
        debug!("Cross product of lines {self:?} and {rhs:?} is {result:?}");
        result
    }
}

impl DotProduct for Line {
    type Output = Float;

    fn dot_product(self, rhs: Self) -> Self::Output {
        self.tuple().dot_product(rhs.tuple())
    }
}
