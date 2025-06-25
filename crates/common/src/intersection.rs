use core::fmt::Display;

use typed_index_collections::TiVec;

use crate::{
    impl_approx_eq, impl_idx,
    math::cartesian::CartesianCoord,
    segment::{Segment, SegmentIdx},
};

pub type Intersections = TiVec<IntersectionIdx, Intersection>;

impl_idx!(IntersectionIdx);

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Intersection {
    pub typ: IntersectionType,
    pub segments: Vec<SegmentIdx>,
    pub mark: bool,
    pub step: Option<usize>,
}

impl_approx_eq!(Intersection, |l, r, margin| l.typ.approx_eq(r.typ, margin)
    && l.segments == r.segments
    && l.mark == r.mark
    && l.step == r.step);

impl Intersection {
    pub fn new(
        typ: IntersectionType,
        segments: Vec<SegmentIdx>,
        step: impl Into<Option<usize>>,
    ) -> Self {
        Self {
            typ,
            segments,
            mark: false,
            step: step.into(),
        }
    }

    #[must_use]
    pub const fn typ(&self) -> &IntersectionType {
        &self.typ
    }

    #[must_use]
    pub fn segments(&self) -> &[SegmentIdx] {
        &self.segments
    }
    #[must_use]
    pub const fn mark_mut(&mut self) -> &mut bool {
        &mut self.mark
    }
    #[must_use]
    pub const fn step(&self) -> Option<usize> {
        self.step
    }
    #[must_use]
    pub const fn mark(&self) -> bool {
        self.mark
    }
    #[must_use]
    pub const fn point1(&self) -> CartesianCoord {
        match self.typ() {
            IntersectionType::Point { coord } => *coord,
            IntersectionType::Parallel { line } => line.upper,
        }
    }
    #[must_use]
    pub const fn point2(&self) -> Option<CartesianCoord> {
        match self.typ() {
            IntersectionType::Point { .. } => None,
            IntersectionType::Parallel { line } => Some(line.lower),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum IntersectionType {
    Point {  coord: CartesianCoord },
    Parallel {  line: Segment },
}

impl IntersectionType {
    /// Returns `true` if the intersection type is [`Point`].
    ///
    /// [`Point`]: IntersectionType::Point
    #[must_use]
    pub const fn is_point(&self) -> bool {
        matches!(self, Self::Point { .. })
    }
}

impl_approx_eq!(IntersectionType, |l, r, margin| match (l, r) {
    (IntersectionType::Point { coord }, IntersectionType::Point { coord: coord2 }) => {
        coord.approx_eq(coord2, margin)
    }
    (IntersectionType::Parallel { line }, IntersectionType::Parallel { line: line2 }) => {
        line.approx_eq(*line2, margin)
    }
    _ => false,
});

impl Display for IntersectionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Point { .. } => write!(f, "Point"),
            Self::Parallel { .. } => write!(f, "Line"),
        }
    }
}
