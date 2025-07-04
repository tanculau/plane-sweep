use core::{hash::Hash, sync::atomic::AtomicUsize};

use tracing::{debug, instrument};
use typed_index_collections::TiVec;

use crate::{
    impl_idx,
    intersection::{Intersection, IntersectionType},
    math::{
        CrossProduct, Float,
        cartesian::CartesianCoord,
        homogeneous::{HomogeneousCoord, HomogeneousLine, Slope},
    },
};

/// Counter to generate IDs for the Segments
static COUNTER: AtomicUsize = AtomicUsize::new(0);

/// Set the counter to a specific ID, should be used after serialization
pub fn set_counter(value: usize) {
    COUNTER.store(value, core::sync::atomic::Ordering::SeqCst);
}

pub fn get_counter() -> usize {
    COUNTER.load(core::sync::atomic::Ordering::SeqCst)
}

pub type Segments = TiVec<SegmentIdx, Segment>;

impl_idx!(SegmentIdx);

/// Represents a line segment defined by two Cartesian coordinates.
///
/// A `Segment` is defined by its `upper` and `lower` endpoints, where the ordering of the
/// coordinates follows specific rules:
/// - `upper.y >= lower.y`
/// - If `upper.y == lower.y`, then `upper.x <= lower.x`
///
/// This ordering ensures a consistent directional interpretation of the segment
/// (e.g., top-to-bottom or left-to-right).
///
/// # Fields
///
/// - `upper`: The upper endpoint of the segment. Must satisfy `upper.y >= lower.y`,
///   and if equal in `y`, then `upper.x <= lower.x`.
/// - `lower`: The lower endpoint of the segment, constrained by the rules above.
/// - `id`: A unique identifier for the segment. This is automatically generated
///   during construction and should not be set manually.
/// - `mark`: Indicates whether the segment should be highlighted in the GUI.
///   This field is not serialized when using `serde`.
/// - `shown`: Controls the visibility and activity of the segment. If `true`, the segment
///   is currently active and considered during computations or rendering.
///
/// # Serialization
///
/// If the `serde` feature is enabled, the struct can be serialized and deserialized,
/// except for the `mark` field, which is skipped.
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Segment {
    /// The upper endpoint of the segment.
    pub upper: CartesianCoord,
    /// The lower endpoint of the segment.
    pub lower: CartesianCoord,
    /// Automatically generated unique identifier for the segment.
    pub id: usize,
    /// Indicates whether this segment should be highlighted in the GUI.
    #[cfg_attr(feature = "serde", serde(skip))]
    pub mark: bool,
    /// Indicates whether the segment is currently active and considered by the algorithms
    pub shown: bool,
}

impl core::fmt::Debug for Segment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Segment(({},{}), ({},{}))",
            self.upper.x, self.upper.y, self.lower.x, self.lower.y
        )
    }
}
impl PartialEq for Segment {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Segment {}

impl Ord for Segment {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}
impl PartialOrd for Segment {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Hash for Segment {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl From<Segment> for HomogeneousLine {
    fn from(value: Segment) -> Self {
        let coord1: HomogeneousCoord = value.upper.into();
        let coord2: HomogeneousCoord = value.lower.into();
        coord1.cross_product(coord2)
    }
}

impl Segment {
    /// Constructs a new [`Segment`].
    ///
    ///  # Parameters
    ///
    /// - `p1`: The first endpoint of the segment. Can be any type that implements [`Into<CartesianCoord>`].
    /// - `p2`: The second endpoint of the segment. Also must implement [`Into<CartesianCoord>`].
    ///
    /// The order is not relevant and will be sorted by the constructor.
    ///
    /// # Returns
    ///
    /// A new [`Segment`] instance with:
    /// - `upper` and `lower` endpoints sorted according to the rules above.
    /// - A unique `id`
    /// - `mark` set to `false`.
    /// - `shown` set to `true`.
    ///
    #[allow(clippy::useless_let_if_seq)]
    pub fn new(p1: impl Into<CartesianCoord>, p2: impl Into<CartesianCoord>) -> Self {
        let p1: CartesianCoord = p1.into();
        let p2: CartesianCoord = p2.into();

        let mut coords = [(p1), p2];
        coords.sort_unstable_by(|l, r| (&l.y).cmp(&(&r.y)).reverse().then((&l.x).cmp(&(&r.x))));

        Self {
            upper: coords[0].clone(),
            lower: coords[1].clone(),
            id: COUNTER.fetch_add(1, core::sync::atomic::Ordering::SeqCst),
            mark: false,
            shown: true,
        }
    }

    /// Calculates the intersection of two [`Segments`](Segment).
    ///
    #[must_use]
    #[instrument(name = "Segment::intersect", skip_all)]
    #[allow(clippy::too_many_lines)]
    pub fn intersect(
        key1: impl Into<SegmentIdx>,
        key2: impl Into<SegmentIdx>,
        segments: &Segments,
        step: usize,
    ) -> Option<Intersection> {
        let key1 = key1.into();
        let key2 = key2.into();
        let segment_left @ Self {
            upper: upper1,
            lower: lower1,
            ..
        } = &segments[key1];
        let segment_right @ Self {
            upper: upper2,
            lower: lower2,
            ..
        } = &segments[key2];
        dbg!(upper2);
        dbg!(lower2);

        let line1 = HomogeneousLine::from(segment_left.clone());
        let line2 = HomogeneousLine::from(segment_right.clone());

        let intersect = line1.intersection(line2).cartesian().inspect(|v| {
            debug!(
                "Calculating intersection between segments {segment_left:?} and {segment_right:?} in step {step}: {v:?}"
            );
        });
        dbg!(&intersect);
        if let Ok(coord) = intersect
            && (((&upper1.x).min(&lower1.x)..=(&upper1.x).max(&lower1.x)).contains(&&coord.x))
            && (((&upper1.y).min(&lower1.y)..=(&upper1.y).max(&lower1.y)).contains(&&coord.y))
            && (((&upper2.x).min(&lower2.x)..=(&upper2.x).max(&lower2.x)).contains(&&coord.x))
            && (((&upper2.y).min(&lower2.y)..=(&upper2.y).max(&lower2.y)).contains(&&coord.y))
        {
            debug!(
                "Intersection found between segments {segment_left:?} and {segment_right:?} at {coord:?} in step {step}"
            );
            Some(Intersection::new(
                IntersectionType::Point { coord },
                vec![key1, key2],
                step,
            ))
        } else {
            // Check if lines are parallel.

            // Check if Segments lie on each other
            if upper1 == upper2 && lower1 == lower2 {
                return Some(Intersection::new(
                    IntersectionType::Parallel {
                        line: Self {
                            upper: upper1.clone(),
                            lower: lower1.clone(),
                            id: usize::MAX,
                            mark: false,
                            shown: false,
                        },
                    },
                    vec![key1, key2],
                    step,
                ));
            }
            let p1 = segment_left.contains(upper2).then_some(upper2);
            let p2 = segment_left.contains(lower2).then_some(lower2);
            let p3 = segment_right.contains(upper1).then_some(upper1);
            let p4 = segment_right.contains(lower1).then_some(lower1);
            let mut iter = p1
                .iter()
                .chain(p2.iter())
                .chain(p3.iter())
                .chain(p4.iter())
                .copied();
            if let (Some(p1), Some(mut p2)) = (iter.next(), iter.next()) {
                while p1 == p2 {
                    if let Some(p3) = iter.next() {
                        p2 = p3;
                    } else {
                        return Some(Intersection::new(
                            IntersectionType::Point { coord: p1.clone() },
                            vec![key1, key2],
                            step,
                        ));
                    }
                }

                let mut segment = Self {
                    upper: p1.clone(),
                    lower: p2.clone(),
                    id: usize::MAX,
                    mark: false,
                    shown: false,
                };
                segment.update();
                return Some(Intersection::new(
                    IntersectionType::Parallel { line: segment },
                    vec![key1, key2],
                    step,
                ));
            }

            debug!(
                "No intersection found between segments {segment_left:?} and {segment_right:?} in step {step}"
            );
            None
        }
    }

    /// Returns true if the `coord` is on the [`Segment`].
    pub fn contains(&self, coord: &CartesianCoord) -> bool {
        let y = &coord.y;
        let x = &coord.x;
        let max_y = &self.upper.y;
        let min_y = &self.lower.y;
        let max_x = (&self.upper.x).max(&self.lower.x);
        let min_x = (&self.upper.x).min(&self.lower.x);

        self.line().contains_coord(coord) && max_y >= y && min_y <= y && max_x >= x && min_x <= x
    }

    pub fn update(&mut self) {
        let p1 = core::mem::take(&mut self.upper);
        let p2 = core::mem::take(&mut self.lower);

        let mut coords = [(p1), p2];
        coords.sort_unstable_by(|l, r| (&l.y).cmp(&(&r.y)).reverse().then((&l.x).cmp(&(&r.x))));
        self.upper = coords[0].clone();
        self.lower = coords[1].clone();
    }

    /// Returns the [`HomogeneousLine`]  defined by this [`Segment`]
    #[must_use]
    pub fn line(&self) -> HomogeneousLine {
        self.upper
            .clone()
            .homogeneous()
            .line(self.lower.clone().homogeneous())
    }

    #[must_use]
    pub fn is_horizontal(&self) -> bool {
        self.upper.y == self.lower.y
    }
    #[must_use]
    pub fn is_vertical(&self) -> bool {
        self.upper.x == self.lower.x
    }

    #[must_use]
    pub fn angle(&self) -> Float {
        self.line().angle()
    }

    #[must_use]
    pub fn slope(&self) -> Slope {
        self.line().slope()
    }
}
