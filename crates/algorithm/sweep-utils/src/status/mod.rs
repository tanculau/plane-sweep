// Based on the book "Computational Geometry" from Mark Berg , Otfried Cheong , Marc Kreveld , Mark Overmars. [DOI](https://doi.org/10.1007/978-3-662-04245-8)
// AVL tree https://www.geeksforgeeks.org/dsa/introduction-to-avl-tree/

mod node;

use core::{
    cmp::Ordering,
    fmt::{Debug, Display},
    marker::PhantomData,
};

use common::{
    math::{A, cartesian::CartesianCoord, homogeneous::HomogeneousLine},
    segment::{Segment, SegmentIdx, Segments},
};
use slotmap::{SlotMap, new_key_type};

use crate::status::node::{Node, NodeCursor};

new_key_type! {struct SQKey;}

type Storage<T> = SlotMap<SQKey, Node<T>>;

/// A Status Queue is an ordered collection-
///
/// Segments are sorted by the x-coordinate at which they intersect the sweep line.
/// If two segments intersect the sweep line at the same x-coordinate, their slopes are used to determine the order.
#[derive(Clone)]
pub struct StatusQueue<T: A> {
    head: SQKey,
    storage: SlotMap<SQKey, Node<T>>,
    phantom: PhantomData<T>,
}

impl<T: A> Debug for StatusQueue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", SQDebug::new(self.head, &self.storage))
    }
}

impl<T: A> StatusQueue<T> {
    #[must_use]
    pub fn new() -> Self {
        let mut storage = Storage::with_key();
        let head = Node::empty(None, &mut storage);
        Self {
            head,
            storage,
            phantom: PhantomData,
        }
    }

    #[must_use]
    pub fn iter(&'_ self) -> SQIter<'_, T> {
        self.into_iter()
    }

    /// Insert a new [`Segment`] into the [`StatusQueue`].
    ///
    /// Does nothing if the [`Segment`] is already present.
    pub fn insert(&mut self, s_idx: SegmentIdx, segments: &Segments<T>, event: &CartesianCoord<T>) {
        //println!("Insert {s_idx:?} at event {event:?}");
        //println!(
        //"Insert before: {}",
        //self.iter().fold(String::new(), |mut acc, v| {
        //    use std::fmt::Write;
        //    write!(&mut acc, ", {:?}:  {:?}", v, intersection(segments[v], event)).unwrap();
        //    acc
        //})
        //);
        //Node::verify_with_event(self.head, &self.storage, segments, event);
        self.head = Node::insert(self.head, &mut self.storage, s_idx, segments, event);
        self.storage[self.head].set_parent(None);
        //Node::verify_with_event(self.head, &self.storage, segments, event);
    }

    /// [`Iterator`](std::iter::Iterator) over all Segments that contain the event point
    pub fn iter_contains(
        &self,
        segments: &Segments<T>,
        event: &CartesianCoord<T>,
    ) -> impl Iterator<Item = SegmentIdx> + Clone {
        Node::find_left_most(self.head, &self.storage, segments, event)
            .into_iter()
            .flat_map(|n| SQIter::new(n, &self.storage))
            .zip(std::iter::repeat(event))
            .take_while(move |(s, event)| intersection::<T>(&segments[*s], event) == (event.x))
            .map(|(l, _)| l)
    }

    /// Finds the greatest segment that is strictly left of the event point
    #[must_use]
    pub fn left_of_event(
        &self,
        segments: &Segments<T>,
        event: &CartesianCoord<T>,
    ) -> Option<SegmentIdx> {
        Node::find_left_of_event(self.head, &self.storage, segments, event)
            .and_then(|v| self.storage[v].data())
    }

    /// Finds the smallest segment that is strictly right of the event point
    #[must_use]
    pub fn right_of_event(
        &self,
        segments: &Segments<T>,
        event: &CartesianCoord<T>,
    ) -> Option<SegmentIdx> {
        Node::find_right_of_event(self.head, &self.storage, segments, event)
            .and_then(|v| self.storage[v].data())
    }

    /// Finds the left-most segment that contains the event point.
    #[must_use]
    pub fn left_most(
        &self,
        segments: &Segments<T>,
        event: &CartesianCoord<T>,
    ) -> Option<SegmentIdx> {
        Node::find_left_most(self.head, &self.storage, segments, event)
            .and_then(|v| self.storage[v].data())
    }

    /// Finds the right-most segment that contains the event point.
    #[must_use]
    pub fn right_most(
        &self,
        segments: &Segments<T>,
        event: &CartesianCoord<T>,
    ) -> Option<SegmentIdx> {
        Node::find_right_most(self.head, &self.storage, segments, event)
            .and_then(|v| self.storage[v].data())
    }

    pub fn delete(&mut self, s_idx: SegmentIdx, segments: &Segments<T>, event: &CartesianCoord<T>) {
        //println!("Delete {s_idx:?} at event {event:?}");

        // Node::verify_with_event(self.head, &self.storage, segments, event); // Not valid, but that is okay
        self.head = Node::delete(self.head, &mut self.storage, s_idx, segments, event);
        self.storage[self.head].set_parent(None);
        //Node::verify_with_event(self.head, &self.storage, segments, event); // Not valid, but that is okay
    }
}

impl<T: A> Default for StatusQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T: A> IntoIterator for &'a StatusQueue<T> {
    type Item = SegmentIdx;

    type IntoIter = SQIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        let leftmost = Node::left_most_node(self.head, &self.storage);
        SQIter::new(leftmost, &self.storage)
    }
}

#[derive(Debug, Clone)]
pub struct SQIter<'a, T: A> {
    storage: &'a Storage<T>,
    inner: SQNodeIter<'a, T>,
}

impl<'a, T: A> SQIter<'a, T> {
    fn new(curr: SQKey, storage: &'a Storage<T>) -> Self {
        Self {
            storage,
            inner: SQNodeIter::new(NodeCursor::new(curr, storage)),
        }
    }
}

impl<T: A> Iterator for SQIter<'_, T> {
    type Item = SegmentIdx;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().and_then(|s| self.storage[s].data())
    }
}

#[derive(Debug, Clone)]
struct SQNodeIter<'a, T: A>(Option<NodeCursor<'a, T>>);

impl<'a, T: A> SQNodeIter<'a, T> {
    pub fn new(cursor: NodeCursor<'a, T>) -> Self {
        if cursor.access().is_node() {
            Self(Some(cursor))
        } else {
            Self(None)
        }
    }
}

impl<T: A> Iterator for SQNodeIter<'_, T> {
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

fn compare3<T: A>(
    lhs: SegmentIdx,
    rhs: SegmentIdx,
    segments: &Segments<T>,
    event: &CartesianCoord<T>,
) -> Ordering {
    if lhs == rhs {
        Ordering::Equal
    } else {
        //println!("Comparing {lhs:?} and {rhs:?} at {event:?}");
        let lhs = &segments[lhs];
        let rhs = &segments[rhs];
        intersection::<T>(lhs, event)
            .cmp(&intersection::<T>(rhs, event))
            .then_with(|| {
                // Compare Upper point
                //println!("Same point, lhs: {} - angle {}, rhs: {} - angle {}", lhs.id, lhs.angle(), rhs.id, rhs.angle());
                lhs.slope().cmp(&rhs.slope()).reverse()
            })
            .then_with(|| {
                // Last compare ids
                lhs.id.cmp(&rhs.id)
            })
    }
}

fn compare<T: A>(lhs: &Segment<T>, rhs: &Segment<T>, event: &CartesianCoord<T>) -> Ordering {
    intersection::<T>(lhs, event)
        .cmp(&intersection::<T>(rhs, event))
        .then_with(|| {
            // Compare Upper point
            //println!("Same point, lhs: {} - angle {}, rhs: {} - angle {}", lhs.id, lhs.angle(), rhs.id, rhs.angle());
            lhs.slope().cmp(&rhs.slope())
        })
        .then_with(|| {
            // Last compare ids
            lhs.id.cmp(&rhs.id)
        })
}

#[allow(clippy::missing_panics_doc)]
#[must_use]
pub fn intersection<T: A>(segment: &Segment<T>, event: &CartesianCoord<T>) -> T {
    if segment.is_horizontal() && event.y == segment.upper.y {
        event
            .x
            .clone()
            .clamp(segment.upper.x.clone(), segment.lower.x.clone())
    } else {
        let horizontal = HomogeneousLine::horizontal(event.y.clone());
        let seg = segment.line();
        horizontal
            .intersection(seg)
            .cartesian()
            .unwrap_or_else(|_| panic!("{segment:?}, {event:?}"))
            .x
    }
}

pub struct SQDebug<'a, T: A> {
    node: SQKey,
    storage: &'a Storage<T>,
}

impl<T: A> Debug for SQDebug<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let curr = self.storage[self.node];
        match curr {
            Node::Node {
                data,
                left,
                right,
                height,
                parent,
                ..
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

impl<T: A> Display for SQDebug<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl<'a, T: A> SQDebug<'a, T> {
    const fn new(node: SQKey, storage: &'a Storage<T>) -> Self {
        Self { node, storage }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;

    #[gtest]
    fn feature() {
        let segments = Segments::<Rational>::from_iter([
            Segment::new((2, 2), (-2, -2)),
            Segment::new((-2, 2), (2, -2)),
            Segment::new((-1, 2), (-1, -2)),
        ]);

        let mut sq = StatusQueue::new();

        sq.insert(0.into(), &segments, &(2, 2).into());
        sq.insert(1.into(), &segments, &(-2, 2).into());
        sq.insert(2.into(), &segments, &(-1, 2).into());
        let iter = sq
            .iter_contains(&segments, &(-1, 1).into())
            .collect::<Vec<_>>();
        expect_that!(iter, elements_are![eq(&1.into()), eq(&2.into())]);
        expect_eq!(
            sq.iter().collect::<Vec<_>>(),
            vec![1.into(), 2.into(), 0.into()]
        );
    }

    #[gtest]
    fn simple() {
        let segments = Segments::<Rational>::from_iter([
            Segment::new((-2, 2), (2, -2)),
            Segment::new((2, 2), (-2, -2)),
        ]);
        let mut sq = StatusQueue::new();
        sq.insert(0.into(), &segments, &(-2, 2).into());
        expect_eq!(sq.left_most(&segments, &(-2, 2).into()), Some(0.into()));
        expect_eq!(sq.left_of_event(&segments, &(2, 2).into()), Some(0.into()));
        sq.insert(1.into(), &segments, &(2, 2).into());
        expect_eq!(sq.left_most(&segments, &(2, 2).into()), Some(1.into()));
        expect_eq!(sq.left_of_event(&segments, &(2, 2).into()), Some(0.into()));
        expect_eq!(sq.right_most(&segments, &(2, 2).into()), Some(1.into()));
        expect_eq!(sq.right_of_event(&segments, &(2, 2).into()), None);
        sq.delete(1.into(), &segments, &(2, 2).into());
        expect_eq!(sq.iter().next(), Some(0.into()));
    }

    use common::{math::Rational, segment::Segment};

    use crate::status::StatusQueue;

    #[gtest]
    fn test_name() {
        let segments = Segments::from_iter([
            Segment::<Rational>::new((2, 2), (-2, -2)),
            Segment::<Rational>::new((-2, 2), (2, -2)),
            Segment::<Rational>::new((-1, 2), (-1, -2)),
        ]);
        let mut sq = StatusQueue::new();
        sq.insert(0.into(), &segments, &(2, 2).into());
        sq.insert(1.into(), &segments, &(-1, 1).into());
        sq.insert(2.into(), &segments, &(-1, 1).into());
        expect_eq!(sq.right_most(&segments, &(-1, 1).into()), Some(1.into()));
        expect_eq!(
            sq.right_of_event(&segments, &(-1, 1).into()),
            Some(0.into())
        );
    }

    #[test]
    fn feature2() {
        let segments = Segments::from_iter([
            Segment::<Rational>::new((-254, 9992), (-1, -258)),
            Segment::new((-258, 8), (113, 0)),
            Segment::new((188, 0), (0, 0)),
        ]);
        let mut queue = StatusQueue::new();
        let intersect = Segment::intersect(0, 1, &segments, 0).unwrap();
        queue.insert(0.into(), &segments, &(-254, 9992).into());
        queue.insert(1.into(), &segments, &(-258, 8).into());
        queue.delete(0.into(), &segments, &(-258, 8).into());
        queue.insert(0.into(), &segments, intersect.point1());
        queue.insert(2.into(), &segments, &(0, 0).into());
    }
}
