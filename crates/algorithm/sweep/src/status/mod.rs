mod node;

use core::{
    cmp::Ordering,
    fmt::{Debug, Display},
};

use common::{
    f_eq,
    math::{OrderedFloat, cartesian::CartesianCoord, homogeneous::HomogeneousLine},
    segment::{Segment, SegmentIdx, Segments},
};
use slotmap::{SlotMap, new_key_type};

use crate::status::node::{Node, NodeCursor};

new_key_type! {struct SQKey;}

type Storage = SlotMap<SQKey, Node>;

/// A Status Queue is an ordered collection-
///
/// Segments are sorted by the x-coordinate at which they intersect the sweep line.
/// If two segments intersect the sweep line at the same x-coordinate, their slopes are used to determine the order.
#[derive(Clone)]
pub struct StatusQueue {
    head: SQKey,
    storage: SlotMap<SQKey, Node>,
}

impl Debug for StatusQueue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", SQDebug::new(self.head, &self.storage))
    }
}

impl StatusQueue {
    #[must_use]
    pub fn new() -> Self {
        let mut storage = Storage::with_key();
        let head = Node::empty(None, &mut storage);
        Self { head, storage }
    }

    #[must_use]
    pub fn iter(&'_ self) -> SQIter<'_> {
        self.into_iter()
    }

    /// Insert a new [`Segment`] into the [`StatusQueue`].
    ///
    /// Does nothing if the [`Segment`] is already present.
    pub fn insert(
        &mut self,
        s_idx: SegmentIdx,
        segments: &Segments,
        event: impl Into<CartesianCoord>,
    ) {
        self.head = Node::insert(self.head, &mut self.storage, s_idx, segments, event.into());
        self.storage[self.head].set_parent(None);
        Node::verify(self.head, &self.storage);
    }

    /// [`Iterator`](std::iter::Iterator) over all Segments that contain the event point
    pub fn iter_contains(
        &self,
        segments: &Segments,
        event: impl Into<CartesianCoord>,
    ) -> impl Iterator<Item = SegmentIdx> + Clone {
        let event = event.into();
        Node::find_left_most(self.head, &self.storage, segments, event)
            .into_iter()
            .flat_map(|n| SQIter::new(n, &self.storage))
            .inspect(|v| {
                dbg!(v);
            })
            .filter(move |s| intersection(segments[*s], event) == OrderedFloat::new(event.x))
    }

    /// Finds the greatest segment that is strictly left of the event point
    pub fn left_of_event(
        &self,
        segments: &Segments,
        event: impl Into<CartesianCoord>,
    ) -> Option<SegmentIdx> {
        Node::find_left_of_event(self.head, &self.storage, segments, event.into())
            .and_then(|v| self.storage[v].data())
    }

    /// Finds the smallest segment that is strictly right of the event point
    pub fn right_of_event(
        &self,
        segments: &Segments,
        event: impl Into<CartesianCoord>,
    ) -> Option<SegmentIdx> {
        Node::find_right_of_event(self.head, &self.storage, segments, event.into())
            .and_then(|v| self.storage[v].data())
    }

    /// Finds the left-most segment that contains the event point.
    pub fn left_most(
        &self,
        segments: &Segments,
        event: impl Into<CartesianCoord>,
    ) -> Option<SegmentIdx> {
        Node::find_left_most(self.head, &self.storage, segments, event.into())
            .and_then(|v| self.storage[v].data())
    }

    /// Finds the right-most segment that contains the event point.
    pub fn right_most(
        &self,
        segments: &Segments,
        event: impl Into<CartesianCoord>,
    ) -> Option<SegmentIdx> {
        Node::find_right_most(self.head, &self.storage, segments, event.into())
            .and_then(|v| self.storage[v].data())
    }

    pub fn delete(
        &mut self,
        s_idx: SegmentIdx,
        segments: &Segments,
        event: impl Into<CartesianCoord>,
    ) {
        self.head = Node::delete(self.head, &mut self.storage, s_idx, segments, event.into());
        self.storage[self.head].set_parent(None);
        Node::verify(self.head, &self.storage);
    }
}

impl Default for StatusQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> IntoIterator for &'a StatusQueue {
    type Item = SegmentIdx;

    type IntoIter = SQIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let leftmost = Node::left_most_node(self.head, &self.storage);
        SQIter::new(leftmost, &self.storage)
    }
}

#[derive(Debug, Clone)]
pub struct SQIter<'a> {
    storage: &'a Storage,
    inner: SQNodeIter<'a>,
}

impl<'a> SQIter<'a> {
    fn new(curr: SQKey, storage: &'a Storage) -> Self {
        Self {
            storage,
            inner: SQNodeIter::new(NodeCursor::new(curr, storage)),
        }
    }
}

impl Iterator for SQIter<'_> {
    type Item = SegmentIdx;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().and_then(|s| self.storage[s].data())
    }
}

#[derive(Debug, Clone)]
struct SQNodeIter<'a>(Option<NodeCursor<'a>>);

impl<'a> SQNodeIter<'a> {
    pub fn new(cursor: NodeCursor<'a>) -> Self {
        if cursor.access().is_node() {
            Self(Some(cursor))
        } else {
            Self(None)
        }
    }
}

impl Iterator for SQNodeIter<'_> {
    type Item = SQKey;

    fn next(&mut self) -> Option<Self::Item> {
        let cursor = self.0.as_mut()?;

        let ret = cursor.curr();

        if !cursor.next() {
            self.0 = None;
        }

        Some(ret)
    }
}

fn compare2(
    lhs: SegmentIdx,
    rhs: SegmentIdx,
    segments: &Segments,
    event: CartesianCoord,
) -> Ordering {
    if lhs == rhs {
        Ordering::Equal
    } else {
        compare(segments[lhs], segments[rhs], event)
    }
}

fn compare(lhs: Segment, rhs: Segment, event: CartesianCoord) -> Ordering {
    intersection(lhs, event)
        .cmp(&intersection(rhs, event))
        .then_with(|| {
            // Compare Upper point
            OrderedFloat(lhs.angle().into()).cmp(&OrderedFloat(rhs.angle().into()))
        })
        .then_with(|| {
            // Last compare ids
            lhs.id.cmp(&rhs.id)
        })
}

pub(crate) fn intersection(segment: Segment, event: CartesianCoord) -> OrderedFloat {
    let x_intersect = if segment.is_horizontal() && f_eq!(event.y, segment.upper.y) {
        event.x.clamp(segment.upper.x, segment.lower.x)
    } else {
        let horizontal = HomogeneousLine::horizontal(event.y);
        let seg = segment.line();
        horizontal.intersection(seg).cartesian().unwrap().x
    }
    .into();
    OrderedFloat(x_intersect)
}

pub struct SQDebug<'a> {
    node: SQKey,
    storage: &'a Storage,
}

impl Debug for SQDebug<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let curr = self.storage[self.node];
        match curr {
            Node::Node {
                data,
                left,
                right,
                height,
                parent,
            } => f
                .debug_struct("Node")
                .field("data", &data)
                .field("height", &height)
                .field("id", &self.node.0)
                .field("parent", &parent)
                .field("left", &SQDebug::new(left, self.storage))
                .field("right", &SQDebug::new(right, self.storage))
                .finish(),
            Node::Leaf { .. } => write!(f, "Leaf({:?})", self.node),
        }
    }
}

impl Display for SQDebug<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl<'a> SQDebug<'a> {
    const fn new(node: SQKey, storage: &'a Storage) -> Self {
        Self { node, storage }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;

    #[gtest]
    fn feature() {
        let segments = Segments::from_iter([
            Segment::new((2, 2), (-2, -2)),
            Segment::new((-2, 2), (2, -2)),
            Segment::new((-1, 2), (-1, -2)),
        ]);

        let mut sq = StatusQueue::new();

        sq.insert(0.into(), &segments, (2, 2));
        sq.insert(1.into(), &segments, (-2, 2));
        sq.insert(2.into(), &segments, (-1, 2));
        let iter = sq.iter_contains(&segments, (-1, 1)).collect::<Vec<_>>();
        expect_that!(iter, elements_are![eq(&1.into()), eq(&2.into())]);
        expect_eq!(
            sq.iter().collect::<Vec<_>>(),
            vec![1.into(), 2.into(), 0.into()]
        );
    }

    #[gtest]
    fn simple() {
        let segments = Segments::from_iter([
            Segment::new((-2, 2), (2, -2)),
            Segment::new((2, 2), (-2, -2)),
        ]);
        let mut sq = StatusQueue::new();
        sq.insert(0.into(), &segments, (-2, 2));
        expect_eq!(sq.left_most(&segments, (-2, 2)), Some(0.into()));
        expect_eq!(sq.left_of_event(&segments, (2, 2)), Some(0.into()));
        sq.insert(1.into(), &segments, (2, 2));
        expect_eq!(sq.left_most(&segments, (2, 2)), Some(1.into()));
        expect_eq!(sq.left_of_event(&segments, (2, 2)), Some(0.into()));
        expect_eq!(sq.right_most(&segments, (2, 2)), Some(1.into()));
        expect_eq!(sq.right_of_event(&segments, (2, 2)), None);
        sq.delete(1.into(), &segments, (2, 2));
        expect_eq!(sq.iter().next(), Some(0.into()));
    }

    use common::segment::Segment;

    use crate::status::StatusQueue;

    #[gtest]
    fn test_name() {
        let segments = Segments::from_iter([
            Segment::new((2, 2), (-2, -2)),
            Segment::new((-2, 2), (2, -2)),
            Segment::new((-1, 2), (-1, -2)),
        ]);
        let mut sq = StatusQueue::new();
        sq.insert(0.into(), &segments, (2, 2));
        sq.insert(1.into(), &segments, (-1, 1));
        sq.insert(2.into(), &segments, (-1, 1));
        dbg!(&sq);
        expect_eq!(sq.right_most(&segments, (-1, 1)), Some(1.into()));
        expect_eq!(sq.right_of_event(&segments, (-1, 1)), Some(0.into()));
    }
}
