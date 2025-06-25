//! A line in Cartesian coordinates.
//! 
//! # Normed
//! 1. The line will be converted into a [`HomogeneousLine`].
//! 2. First it will be checked if the line as a intersection with the x-axis. If yes, this will be `coord1`, if not it will be matched with the y-axis.
//! 3. Next from `coord1` with the slope of the line a distance of 1.0 will be taken. This will be `coord2`.
//! 


use crate::{impl_approx_eq, math::homogeneous::{HomogeneousLine, PointAtInfinity}};

use super::CartesianCoord;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(kani, derive(kani::Arbitrary))]
pub struct Line {
    pub coord1: CartesianCoord,
    pub coord2: CartesianCoord,
}

impl Line {
    pub fn new(coord1: impl Into<CartesianCoord>, coord2: impl Into<CartesianCoord>) -> Self {
        Self {
            coord1: coord1.into(),
            coord2: coord2.into(),
        }
    }

    #[must_use]
    pub const fn is_finite(self) -> bool {
        self.coord1.is_finite() && self.coord2.is_finite()
    }

    pub fn normalize(&mut self) {
        todo!()
    }
}

impl<T, K> From<(T, K)> for Line
where
    T: Into<CartesianCoord>,
    K: Into<CartesianCoord>,
{
    fn from((coord1, coord2): (T, K)) -> Self {
        Self::new(coord1, coord2)
    }
}

impl TryFrom<HomogeneousLine> for Line {
    type Error = PointAtInfinity;

    fn try_from(value: HomogeneousLine) -> Result<Self, Self::Error> {
        todo!()
    }
}


impl TryFrom<&HomogeneousLine> for Line {
    type Error = PointAtInfinity;

    fn try_from(value: &HomogeneousLine) -> Result<Self, Self::Error> {
        Self::try_from(*value)
    }
}

impl TryFrom<&mut HomogeneousLine> for Line {
    type Error = PointAtInfinity;

    fn try_from(value: &mut HomogeneousLine) -> Result<Self, Self::Error> {
        Self::try_from(*value)
    }
}




impl_approx_eq!(Line, |l,r, margin| {l.coord1.approx_eq(r.coord1, margin) && l.coord2.approx_eq(r.coord2, margin)});