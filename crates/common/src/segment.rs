use core::{hash::Hash, sync::atomic::AtomicUsize};

use float_cmp::{ApproxEq, F64Margin};
use ordered_float::OrderedFloat;
use tracing::{debug, instrument};
use typed_index_collections::TiVec;

use crate::{
    impl_idx,
    intersection::{Intersection, IntersectionType},
    math::{
        CrossProduct,
        cartesian::CartesianCoord,
        homogeneous::{HomogeneousCoord, HomogeneousLine},
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
#[derive(Debug, Clone, Copy)]
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

impl ApproxEq for Segment {
    type Margin = F64Margin;

    fn approx_eq<T: Into<Self::Margin>>(self, other: Self, margin: T) -> bool {
        let margin = margin.into();
        self.upper.approx_eq(other.upper, margin) && self.lower.approx_eq(other.lower, margin)
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
        coords.sort_unstable_by(|l, r| {
            OrderedFloat(l.y)
                .cmp(&OrderedFloat(r.y))
                .reverse()
                .then(OrderedFloat(l.x).cmp(&OrderedFloat(r.x)))
        });

        Self {
            upper: coords[0],
            lower: coords[1],
            id: COUNTER.fetch_add(1, core::sync::atomic::Ordering::SeqCst),
            mark: false,
            shown: true,
        }
    }
    /// Constructs a new [`Segment`].
    ///
    /// For more details see [`Segment::new`]
    ///
    /// # Parameters
    ///
    /// Accepts any type that implements [`TryInto<CartesianCoord, Error = E>`].
    #[allow(clippy::missing_errors_doc)]
    pub fn try_new<E>(
        p1: impl TryInto<CartesianCoord, Error = E>,
        p2: impl TryInto<CartesianCoord, Error = E>,
    ) -> Result<Self, E> {
        let p1 = p1.try_into()?;
        let p2 = p2.try_into()?;

        Ok(Self::new(p1, p2))
    }

    /// Calculates the intersection of two [`Segments`](Segment).
    ///
    #[must_use]
    #[instrument(name = "Segment::intersect")]
    pub fn intersect(
        [key1, key2]: [SegmentIdx; 2],
        segments: &Segments,
        step: usize,
    ) -> Option<Intersection> {
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

        let line1 = HomogeneousLine::from(*segment_left);
        let line2 = HomogeneousLine::from(*segment_right);

        let intersect = line1.intersection(line2).cartesian().inspect(|v| {
            debug!(
                "Calculating intersection between segments {segment_left:?} and {segment_right:?} in step {step}: {v:?}"
            );
        });
        if let Ok(coord) = intersect
            && ((min(upper1.x, lower1.x)..=max(upper1.x, lower1.x)).contains(&coord.x))
            && ((min(upper1.y, lower1.y)..=max(upper1.y, lower1.y)).contains(&coord.y))
            && ((min(upper2.x, lower2.x)..=max(upper2.x, lower2.x)).contains(&coord.x))
            && ((min(upper2.y, lower2.y)..=max(upper2.y, lower2.y)).contains(&coord.y))
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
            debug!(
                "No intersection found between segments {segment_left:?} and {segment_right:?} in step {step}"
            );
            None
        }
    }

    /// Returns true if the `coord` is on the [`Segment`].
    pub fn contains(&self, coord: impl Into<CartesianCoord>) -> bool {
        let coord = coord.into();
        let y = OrderedFloat(coord.y);
        let x = OrderedFloat(coord.x);
        let max_y = OrderedFloat(self.upper.y);
        let min_y = OrderedFloat(self.lower.y);
        let max_x = OrderedFloat(self.upper.x).max(OrderedFloat(self.lower.x));
        let min_x = OrderedFloat(self.upper.x).min(OrderedFloat(self.lower.x));

        self.line().contains_coord(coord) && max_y >= y && min_y <= y && max_x >= x && min_x <= x
    }

    /// Returns the [`HomogeneousLine`]  defined by this [`Segment`]
    #[must_use]
    pub fn line(&self) -> HomogeneousLine {
        self.upper.homogeneous().line(self.lower.homogeneous())
    }
}

fn min(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}
fn max(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
}
