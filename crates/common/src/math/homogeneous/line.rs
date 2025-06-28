use tracing::{debug, instrument};

use crate::{
    f_eq,
    math::{CrossProduct, DotProduct, Float, OrderedFloat, homogeneous::HomogeneousCoord},
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Line {
    a: Float,
    b: Float,
    c: Float,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Slope {
    Vertiical,
    Value(OrderedFloat),
    Horizontal,
    Infinity,
}

impl Line {
    pub const X_AXIS: Self = Self {
        a: 0.0,
        b: 1.0,
        c: 0.0,
    };
    pub const Y_AXIS: Self = Self {
        a: 1.0,
        b: 0.0,
        c: 0.0,
    };

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
    pub const fn is_finite(self) -> bool {
        self.a.is_finite() && self.b.is_finite() && self.c.is_finite()
    }

    #[must_use]
    pub const fn tuple(self) -> (Float, Float, Float) {
        (self.a, self.b, self.c)
    }

    #[must_use]
    pub const fn array(self) -> [Float; 3] {
        [self.a, self.b, self.c]
    }

    #[instrument(name = "Line::contains_coord", skip(self, coord))]
    pub fn contains_coord(self, coord: impl Into<HomogeneousCoord>) -> bool {
        let coord = coord.into();
        let res = self.tuple().dot_product(coord.tuple());
        let res = f_eq!(res, 0.0);
        debug!("Line contains coord: {coord:?} on line {self:?} -> {res}");
        res
    }

    #[must_use]
    pub fn intersection(self, other: Self) -> HomogeneousCoord {
        self.cross_product(other)
    }

    #[must_use]
    pub fn slope(self) -> Slope {
        match (f_eq!(self.a, 0.0), (f_eq!(self.b, 0.0))) {
            (true, true) => Slope::Infinity,
            (true, false) => Slope::Horizontal,
            (false, true) => Slope::Vertiical,
            (false, false) => Slope::Value(OrderedFloat((self.b / self.a).abs().into())),
        }
    }

    #[must_use]
    pub fn angle(self) -> Float {
        self.b.atan2(self.a)


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
