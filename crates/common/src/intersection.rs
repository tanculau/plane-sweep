use core::fmt::Display;
use std::collections::HashMap;

use typed_index_collections::TiVec;

use crate::{
    impl_idx,
    math::cartesian::CartesianCoord,
    segment::{Segment, SegmentIdx},
};

pub type Intersections = TiVec<IntersectionIdx, Intersection>;

impl_idx!(IntersectionIdx);

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Intersection {
    pub typ: IntersectionType,
    pub segments: Vec<SegmentIdx>,
    pub step: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IntersectionShort {
    pub typ: IntersectionType,
    pub segments: [SegmentIdx; 2],
}

impl Intersection {
    #[must_use]
    pub const fn new(typ: IntersectionType, segments: Vec<SegmentIdx>, step: usize) -> Self {
        Self {
            typ,
            segments,
            step,
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
    pub const fn step(&self) -> usize {
        self.step
    }
    #[must_use]
    pub const fn point1(&self) -> &CartesianCoord {
        match self.typ() {
            IntersectionType::Point { coord } => coord,
            IntersectionType::Parallel { line } => &line.upper,
        }
    }
    #[must_use]
    pub const fn point2(&self) -> Option<&CartesianCoord> {
        match self.typ() {
            IntersectionType::Point { .. } => None,
            IntersectionType::Parallel { line } => Some(&line.lower),
        }
    }
}

#[derive(Clone, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum IntersectionType {
    Point { coord: CartesianCoord },
    Parallel { line: Segment },
}

impl core::fmt::Debug for IntersectionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Point { coord } => write!(f, "Point({},{})", coord.x, coord.y),
            Self::Parallel { line } => write!(
                f,
                "Line(({},{}), ({},{}))",
                line.upper.x, line.upper.y, line.lower.x, line.lower.y
            ),
        }
    }
}

impl core::hash::Hash for IntersectionType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::Point { coord } => coord.hash(state),
            Self::Parallel { line } => {
                line.upper.hash(state);
                line.lower.hash(state);
            }
        }
    }
}

impl PartialEq for IntersectionType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Point { coord: l_coord }, Self::Point { coord: r_coord }) => l_coord == r_coord,
            (Self::Parallel { line: l_line }, Self::Parallel { line: r_line }) => {
                l_line.upper == r_line.upper && l_line.lower == r_line.lower
            }
            _ => false,
        }
    }
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

impl Display for IntersectionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Point { .. } => write!(f, "Point"),
            Self::Parallel { .. } => write!(f, "Line"),
        }
    }
}

struct Helper {
    inner: HashMap<[SegmentIdx; 2], Vec<CartesianCoord>>,
}

impl Helper {
    fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }
    fn insert(&mut self, mut segment: [SegmentIdx; 2], inter: CartesianCoord) {
        segment.sort_unstable();
        self.inner
            .entry(segment)
            .and_modify(|e| e.push(inter.clone()))
            .or_insert_with(|| vec![inter]);
    }
}

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn to_lines(intersections: &Intersections) -> Vec<IntersectionShort> {
    let mut helper = Helper::new();
    for intersection in intersections {
        let segments = intersection.segments().to_vec();
        for i in 0..segments.len() {
            for j in i + 1..segments.len() {
                helper.insert([segments[i], segments[j]], intersection.point1().clone());
                if let Some(point2) = intersection.point2() {
                    helper.insert([segments[i], segments[j]], point2.clone());
                }
            }
        }
    }
    let mut out = Vec::new();
    for (key, val) in helper.inner {
        match val.len() {
            0 => unreachable!("{key:?}: {val:?}"),
            1 => out.push(IntersectionShort {
                typ: IntersectionType::Point {
                    coord: val[0].clone(),
                },
                segments: key,
            }),
            2.. => out.push(IntersectionShort {
                typ: IntersectionType::Parallel {
                    line: Segment::new(
                        val.iter().min().unwrap().clone(),
                        val.iter().max().unwrap().clone(),
                    ),
                },
                segments: key,
            }),
        }
    }

    out
}
