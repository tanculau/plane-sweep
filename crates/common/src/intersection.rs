use core::fmt::Display;
use std::collections::HashMap;

use smallvec::SmallVec;
use typed_index_collections::TiVec;

use crate::{
    impl_idx,
    math::{A, cartesian::CartesianCoord},
    segment::{Segment, SegmentIdx},
};

pub type Intersections<T> = TiVec<IntersectionIdx, Intersection<T>>;
pub type LeanIntersections<T> = TiVec<LeanIntersectionIdx, LeanIntersection<T>>;

#[must_use]
pub fn lean_to_normal<'a, T: A + 'a>(
    lean: impl Iterator<Item = &'a LeanIntersection<T>>,
) -> Intersections<T> {
    lean.map(|v| Intersection::new(v.coord.clone(), v.segments.into(), v.step))
        .collect()
}

pub type InterVec = SmallVec<SegmentIdx, 4>;

impl_idx!(IntersectionIdx);
impl_idx!(LeanIntersectionIdx);

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Intersection<T: A> {
    pub typ: IntersectionType<T>,
    pub segments: InterVec,
    pub step: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LeanIntersection<T: A> {
    pub coord: IntersectionType<T>,
    pub segments: [SegmentIdx; 2],
    pub step: usize,
}

impl<T: A> LeanIntersection<T> {
    #[must_use]
    pub fn new(coord: IntersectionType<T>, mut segments: [SegmentIdx; 2], step: usize) -> Self {
        segments.sort_unstable();
        Self {
            coord,
            segments,
            step,
        }
    }

    #[must_use]
    pub const fn point1(&self) -> &CartesianCoord<T> {
        match &self.coord {
            IntersectionType::Point { coord } => coord,
            IntersectionType::Parallel { line } => &line.upper,
        }
    }
}

impl<T: A> Intersection<T> {
    #[must_use]
    pub const fn new(typ: IntersectionType<T>, segments: InterVec, step: usize) -> Self {
        Self {
            typ,
            segments,
            step,
        }
    }

    #[must_use]
    pub const fn typ(&self) -> &IntersectionType<T> {
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
    pub const fn point1(&self) -> &CartesianCoord<T> {
        match self.typ() {
            IntersectionType::Point { coord } => coord,
            IntersectionType::Parallel { line } => &line.upper,
        }
    }
    #[must_use]
    pub const fn point2(&self) -> Option<&CartesianCoord<T>> {
        match self.typ() {
            IntersectionType::Point { .. } => None,
            IntersectionType::Parallel { line } => Some(&line.lower),
        }
    }
}

#[derive(Clone, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum IntersectionType<T: A> {
    Point { coord: CartesianCoord<T> },
    Parallel { line: Segment<T> },
}

impl<T: A> core::fmt::Debug for IntersectionType<T> {
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

impl<T: A> PartialEq for IntersectionType<T> {
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

impl<T: A> IntersectionType<T> {
    /// Returns `true` if the intersection type is [`Point`].
    ///
    /// [`Point`]: IntersectionType::Point
    #[must_use]
    pub const fn is_point(&self) -> bool {
        matches!(self, Self::Point { .. })
    }
}

impl<T: A> Display for IntersectionType<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Point { .. } => write!(f, "Point"),
            Self::Parallel { .. } => write!(f, "Line"),
        }
    }
}

struct Helper<T: A> {
    inner: HashMap<[SegmentIdx; 2], Vec<CartesianCoord<T>>>,
}

impl<T: A> Helper<T> {
    fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }
    fn insert(&mut self, mut segment: [SegmentIdx; 2], inter: CartesianCoord<T>) {
        segment.sort_unstable();
        self.inner
            .entry(segment)
            .and_modify(|e| e.push(inter.clone()))
            .or_insert_with(|| vec![inter]);
    }
}

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn to_lines<T: A>(intersections: &Intersections<T>) -> Vec<LeanIntersection<T>> {
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
            1 => out.push(LeanIntersection {
                coord: IntersectionType::Point {
                    coord: val[0].clone(),
                },
                segments: key,
                step: 0,
            }),
            2.. => out.push(LeanIntersection {
                coord: IntersectionType::Parallel {
                    line: Segment::new(
                        val.iter().min().unwrap().clone(),
                        val.iter().max().unwrap().clone(),
                    ),
                },
                segments: key,
                step: 0,
            }),
        }
    }

    out
}
