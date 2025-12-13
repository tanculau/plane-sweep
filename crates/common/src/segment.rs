// Based on Geometriekalküle from  Jürgen Richter-Gebert, Thorsten Orendt https://doi.org/10.1007/978-3-642-02530-3

use core::{hash::Hash, sync::atomic::AtomicUsize};
use std::cmp::PartialEq;

use smallvec::smallvec;
use tracing::{debug, instrument};
use typed_index_collections::TiVec;

use crate::{
    impl_idx,
    intersection::{Intersection, IntersectionType},
    math::{
        A, CrossProduct,
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

pub type Segments<T> = TiVec<SegmentIdx, Segment<T>>;

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
#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Segment<T: A> {
    /// The upper endpoint of the segment.
    pub upper: CartesianCoord<T>,
    /// The lower endpoint of the segment.
    pub lower: CartesianCoord<T>,
    /// Automatically generated unique identifier for the segment.
    pub id: usize,
    /// Indicates whether this segment should be highlighted in the GUI.
    #[cfg_attr(feature = "serde", serde(skip))]
    pub mark: bool,
    /// Indicates whether the segment is currently active and considered by the algorithms
    pub shown: bool,
}

impl<T: A> core::fmt::Debug for Segment<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Segment(({},{}), ({},{}))",
            self.upper.x, self.upper.y, self.lower.x, self.lower.y
        )
    }
}
impl<T: A> PartialEq for Segment<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl<T: A> Eq for Segment<T> {}

impl<T: A> Ord for Segment<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}
impl<T: A> PartialOrd for Segment<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<T: A + Hash> Hash for Segment<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<T: A> From<Segment<T>> for HomogeneousLine<T> {
    fn from(value: Segment<T>) -> Self {
        let coord1: HomogeneousCoord<T> = value.upper.into();
        let coord2: HomogeneousCoord<T> = value.lower.into();
        coord1.cross_product(coord2)
    }
}

impl<T: A> Segment<T> {
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
    pub fn new(p1: impl Into<CartesianCoord<T>>, p2: impl Into<CartesianCoord<T>>) -> Self {
        let p1: CartesianCoord<T> = p1.into();
        let p2: CartesianCoord<T> = p2.into();

        let mut coords = [(p1), p2];
        coords.sort_unstable_by(|l, r| l.y.cmp(&r.y).reverse().then(l.x.cmp(&r.x)));

        Self {
            upper: coords[0].clone(),
            lower: coords[1].clone(),
            id: COUNTER.fetch_add(1, core::sync::atomic::Ordering::SeqCst),
            mark: false,
            shown: true,
        }
    }

    pub fn update(&mut self) {
        let p1 = core::mem::take(&mut self.upper);
        let p2 = core::mem::take(&mut self.lower);

        let mut coords = [(p1), p2];
        coords.sort_unstable_by(|l, r| l.y.cmp(&r.y).reverse().then(l.x.cmp(&r.x)));
        self.upper = coords[0].clone();
        self.lower = coords[1].clone();
    }

    #[must_use]
    pub fn is_horizontal(&self) -> bool {
        self.upper.y == self.lower.y
    }
    #[must_use]
    pub fn is_vertical(&self) -> bool {
        self.upper.x == self.lower.x
    }
}

impl<T: A> Segment<T> {
    #[must_use]
    pub fn slope(&self) -> Slope<T> {
        self.line().slope()
    }

    pub fn midpoint(&self) -> CartesianCoord<T> {
        let x = self.lower.x.clone() + &((self.upper.x.clone() - &self.lower.x) / &T::from(2));
        let y = self.lower.y.clone() + &((self.upper.y.clone() - &self.lower.y) / &T::from(2));

        CartesianCoord::new(x, y)
    }
}

impl<T: A> Segment<T> {
    /// Returns the [`HomogeneousLine`]  defined by this [`Segment`]
    #[must_use]
    pub fn line(&self) -> HomogeneousLine<T> {
        self.upper
            .clone()
            .homogeneous()
            .line(&self.lower.clone().homogeneous())
    }
}

impl<T: A> Segment<T> {
    /// Returns true if the `coord` is on the [`Segment`].
    #[must_use]
    pub fn contains(&self, coord: &CartesianCoord<T>) -> bool {
        let y = &coord.y;
        let x = &coord.x;
        let max_y = &self.upper.y;
        let min_y = &self.lower.y;
        let max_x = (&self.upper.x).max(&self.lower.x);
        let min_x = (&self.upper.x).min(&self.lower.x);

        self.line().contains_coord(coord) && max_y >= y && min_y <= y && max_x >= x && min_x <= x
    }
}

pub fn bisector<T: A + Copy>(x1: T, y1: T, x2: T, y2: T) -> HomogeneousLine<T> {
    let line = Segment::<T>::new((x1, y1), (x2, y2)).line();
    let a = line.b;
    let b = -line.a;
    let (xm, ym) = mid_point(x1, y1, x2, y2);
    let c = -a * xm - b * ym;
    HomogeneousLine::new(a, b, c)
}

pub fn mid_point<T: A>(x1: T, y1: T, x2: T, y2: T) -> (T, T) {
    (
        x1.clone() + (x2 - x1) / T::from(2),
        y1.clone() + (y2 - y1) / T::from(2),
    )
}

impl<T: A> Segment<T> {
    pub fn pedendicular_bisector(&self) -> HomogeneousLine<T> {
        let mid = self.midpoint();
        let line = self.line();
        let a = line.b;
        let b = -line.a;
        let c = -a.clone() * mid.x - b.clone() * mid.y;

        HomogeneousLine::<T>::new(a, b, c)
    }

    /// Calculates the intersection of two [`Segments`](Segment).
    ///
    #[must_use]
    #[instrument(name = "Segment::intersect", skip_all)]
    #[allow(clippy::too_many_lines)]
    pub fn intersect(
        key1: impl Into<SegmentIdx>,
        key2: impl Into<SegmentIdx>,
        segments: &Segments<T>,
        step: usize,
    ) -> Option<Intersection<T>> {
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

        let line1 = HomogeneousLine::<T>::from(segment_left.clone());
        let line2 = HomogeneousLine::<T>::from(segment_right.clone());

        let intersect = line1.intersection(line2).cartesian().inspect(|v| {
            debug!(
                "Calculating intersection between segments {segment_left:?} and {segment_right:?} in step {step}: {v:?}"
            );
        });
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
                smallvec![key1, key2],
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
                    smallvec![key1, key2],
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
                            smallvec![key1, key2],
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
                    smallvec![key1, key2],
                    step,
                ));
            }

            debug!(
                "No intersection found between segments {segment_left:?} and {segment_right:?} in step {step}"
            );
            None
        }
    }

    #[must_use]
    #[instrument(name = "Segment::intersect", skip_all)]
    #[allow(clippy::too_many_lines)]
    pub fn intersect2(&self, other: Self) -> bool {
        let segment_left @ Self {
            upper: upper1,
            lower: lower1,
            ..
        } = self;
        let segment_right = other.clone();
        let Self {
            upper: upper2,
            lower: lower2,
            ..
        } = other;

        let line1 = HomogeneousLine::<T>::from(segment_left.clone());
        let line2 = HomogeneousLine::<T>::from(segment_right.clone());

        let intersect = line1.intersection(line2).cartesian();
        if let Ok(coord) = intersect
            && (((&upper1.x).min(&lower1.x)..=(&upper1.x).max(&lower1.x)).contains(&&coord.x))
            && (((&upper1.y).min(&lower1.y)..=(&upper1.y).max(&lower1.y)).contains(&&coord.y))
            && (((&upper2.x).min(&lower2.x)..=(&upper2.x).max(&lower2.x)).contains(&&coord.x))
            && (((&upper2.y).min(&lower2.y)..=(&upper2.y).max(&lower2.y)).contains(&&coord.y))
        {
            true
        } else {
            // Check if lines are parallel.

            // Check if Segments lie on each other
            if upper1 == &upper2 && lower1 == &lower2 {
                return true;
            }
            let p1 = segment_left.contains(&upper2).then_some(upper2);
            let p2 = segment_left.contains(&lower2).then_some(lower2);
            let p3 = segment_right.contains(upper1).then_some(upper1);
            let p4 = segment_right.contains(lower1).then_some(lower1);
            let mut iter = p1
                .iter()
                .chain(p2.iter())
                .chain(p3.into_iter())
                .chain(p4.into_iter());
            if let (Some(p1), Some(mut p2)) = (iter.next(), iter.next()) {
                while p1 == p2 {
                    if let Some(p3) = iter.next() {
                        p2 = p3;
                    } else {
                        return true;
                    }
                }

                return true;
            }

            false
        }
    }
}
