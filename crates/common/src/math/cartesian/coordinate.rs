// Based on Geometriekalküle from  Jürgen Richter-Gebert, Thorsten Orendt https://doi.org/10.1007/978-3-642-02530-3

use core::fmt::Debug;

use typed_index_collections::TiVec;

use crate::{
    impl_idx,
    math::{
        A,
        homogeneous::{HomogeneousCoord, PointAtInfinity},
    },
};

pub type CartesianCoords<T> = TiVec<CartesianCoordIndex, Coord<T>>;

impl_idx!(CartesianCoordIndex);

#[derive(Clone, Copy, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(kani, derive(kani::Arbitrary))]
pub struct Coord<T: A> {
    pub x: T,
    pub y: T,
}

impl<T: A> PartialOrd for Coord<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: A> Ord for Coord<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.y.cmp(&other.y).reverse().then(self.x.cmp(&other.x))
    }
}

impl<T: A> Debug for Coord<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Coord({},{})", self.x, self.y)
    }
}

impl<T: A> Coord<T> {
    pub fn new(x: impl Into<T>, y: impl Into<T>) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }

    #[must_use]
    pub const fn tuple(&self) -> (&T, &T) {
        (&self.x, &self.y)
    }

    #[must_use]
    pub const fn array(&self) -> [&T; 2] {
        [&self.x, &self.y]
    }

    #[must_use]
    pub fn homogeneous(self) -> HomogeneousCoord<T> {
        HomogeneousCoord::new(self.x, self.y, 1)
    }
}

impl<T: A> Coord<T> {
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn array_float(&self) -> [f64; 2] {
        [self.x.clone().to_float(), self.y.clone().to_float()]
    }
}

impl<T: A, TX: Into<T>, TY: Into<T>> From<(TX, TY)> for Coord<T> {
    fn from((x, y): (TX, TY)) -> Self {
        Self::new(x, y)
    }
}

impl<TT: A, T: Into<TT>> From<[T; 2]> for Coord<TT> {
    fn from([x, y]: [T; 2]) -> Self {
        Self::new(x, y)
    }
}

impl<T: A> TryFrom<HomogeneousCoord<T>> for Coord<T> {
    type Error = PointAtInfinity;

    fn try_from(value: HomogeneousCoord<T>) -> Result<Self, Self::Error> {
        value.cartesian()
    }
}
impl<T: A> TryFrom<&HomogeneousCoord<T>> for Coord<T> {
    type Error = PointAtInfinity;

    fn try_from(value: &HomogeneousCoord<T>) -> Result<Self, Self::Error> {
        Self::try_from(value.clone())
    }
}
impl<T: A> TryFrom<&mut HomogeneousCoord<T>> for Coord<T> {
    type Error = PointAtInfinity;

    fn try_from(value: &mut HomogeneousCoord<T>) -> Result<Self, Self::Error> {
        Self::try_from(value.clone())
    }
}
