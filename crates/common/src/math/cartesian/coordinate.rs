use crate::math::{
    Float,
    homogeneous::{HomogeneousCoord, PointAtInfinity},
};

#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(kani, derive(kani::Arbitrary))]
pub struct Coord {
    pub x: Float,
    pub y: Float,
}

impl core::fmt::Debug for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Coord({},{})", self.x, self.y)
    }
}

impl Coord {
    pub fn new(x: impl Into<Float>, y: impl Into<Float>) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }

    #[must_use]
    pub const fn is_finite(&self) -> bool {
        self.x.is_finite() && self.y.is_finite()
    }

    #[must_use]
    pub const fn tuple(&self) -> (&Float, &Float) {
        (&self.x, &self.y)
    }

    #[must_use]
    pub const fn array(&self) -> [&Float; 2] {
        [&self.x, &self.y]
    }
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn array_float(&self) -> [f64; 2] {
        [self.x.try_into().unwrap(), self.y.try_into().unwrap()]
    }

    #[must_use]
    pub fn homogeneous(self) -> HomogeneousCoord {
        HomogeneousCoord::new(self.x, self.y, 1)
    }
}

impl<TX: Into<Float>, TY: Into<Float>> From<(TX, TY)> for Coord {
    fn from((x, y): (TX, TY)) -> Self {
        Self::new(x, y)
    }
}

impl<T: Into<Float>> From<[T; 2]> for Coord {
    fn from([x, y]: [T; 2]) -> Self {
        Self::new(x, y)
    }
}

impl TryFrom<HomogeneousCoord> for Coord {
    type Error = PointAtInfinity;

    fn try_from(value: HomogeneousCoord) -> Result<Self, Self::Error> {
        value.cartesian()
    }
}
impl TryFrom<&HomogeneousCoord> for Coord {
    type Error = PointAtInfinity;

    fn try_from(value: &HomogeneousCoord) -> Result<Self, Self::Error> {
        Self::try_from(value.clone())
    }
}
impl TryFrom<&mut HomogeneousCoord> for Coord {
    type Error = PointAtInfinity;

    fn try_from(value: &mut HomogeneousCoord) -> Result<Self, Self::Error> {
        Self::try_from(value.clone())
    }
}
