use crate::{
    impl_approx_eq,
    math::{
        Distance, Float,
        homogeneous::{HomogeneousCoord, PointAtInfinity},
    },
};

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(kani, derive(kani::Arbitrary))]
pub struct Coord {
    pub x: Float,
    pub y: Float,
}

impl Coord {
    pub fn new(x: impl Into<Float>, y: impl Into<Float>) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }

    #[must_use]
    pub const fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite()
    }

    #[must_use]
    pub const fn tuple(self) -> (Float, Float) {
        (self.x, self.y)
    }

    #[must_use]
    pub const fn array(self) -> [Float; 2] {
        [self.x, self.y]
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

impl_approx_eq!(Coord, |a, b, margin| a.x.approx_eq(b.x, margin)
    && a.y.approx_eq(b.y, margin));

impl Distance for Coord {
    type Output = Float;

    #[cfg_attr(kani, kani::requires(self.is_finite() && rhs.is_finite()))]
    fn distance(self, rhs: Self) -> Self::Output {
        let Self { x, y } = self;
        let Self { x: x2, y: y2 } = rhs;
        if cfg!(kani) {
            // Avoiding hypot and powi, because kani has problems to verify these functinos

            let a = self.x - rhs.x;
            let b = self.y - rhs.y;

            #[expect(
                clippy::imprecise_flops,
                reason = "hypot is not yet implemented for kani"
            )]
            (a * a + b * b).sqrt()
        } else {
            (x - x2).hypot(y - y2)
        }
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
        Self::try_from(*value)
    }
}
impl TryFrom<&mut HomogeneousCoord> for Coord {
    type Error = PointAtInfinity;

    fn try_from(value: &mut HomogeneousCoord) -> Result<Self, Self::Error> {
        Self::try_from(*value)
    }
}
