use core::{borrow::Borrow, cmp::Ordering, fmt::Debug, iter, ops::Bound};
use std::collections::{BTreeSet, HashSet};

use common::{
    f_eq,
    math::{Float, OrderedFloat, cartesian::CartesianCoord, homogeneous::HomogeneousLine},
    segment::{Segment, SegmentIdx, Segments},
};
use slotmap::{Key, SlotMap, new_key_type};

use crate::event::{self, Event};

#[derive(Debug, Clone, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Status {
    pub x_intersect: OrderedFloat,
    pub segments: Vec<SegmentIdx>,
}
impl PartialEq for Status {
    fn eq(&self, other: &Self) -> bool {
        self.x_intersect.eq(&other.x_intersect)
    }
}

impl PartialOrd for Status {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Status {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.x_intersect.cmp(&other.x_intersect)
    }
}

impl Borrow<OrderedFloat> for Status {
    fn borrow(&self) -> &OrderedFloat {
        &self.x_intersect
    }
}

impl Status {
    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn new(p_y: Float, p_x: Float, segment: Segment, segment_idx: SegmentIdx) -> Self {
        let x_intersect = if segment.is_horizontal() && f_eq!(p_y, segment.upper.y) {
            p_x.clamp(segment.upper.x, segment.lower.x)
        } else {
            let horizontal = HomogeneousLine::horizontal(p_y);
            let seg = segment.line();
            horizontal.intersection(seg).cartesian().unwrap().x
        }
        .into();

        Self {
            x_intersect,
            segments: vec![segment_idx],
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StatusQueue {
    pub inner: BTreeSet<Status>,
}

impl StatusQueue {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            inner: BTreeSet::new(),
        }
    }

    pub fn update(&mut self, y: impl Into<Float>, x: impl Into<Float>, segments: &Segments) {
        let y = y.into();
        let x = x.into();

        let a = std::mem::take(self);
        *self = a
            .into_iter()
            .flat_map(|s| s.segments.into_iter())
            .map(|s| Status::new(y, x, segments[s], s))
            .collect();
    }

    pub fn iter(&self) -> std::collections::btree_set::Iter<'_, Status> {
        self.inner.iter()
    }

    pub fn remove(
        &mut self,
        elements: &HashSet<SegmentIdx>,
        y: impl Into<Float>,
        x: impl Into<Float>,
        segments: &Segments,
    ) {
        let y = y.into();
        let x = x.into();

        let a = std::mem::take(self);
        *self = a
            .into_iter()
            .flat_map(|s| s.segments.into_iter())
            .filter(|s| !elements.contains(s))
            .map(|s| Status::new(y, x, segments[s], s))
            .collect();
    }

    pub fn insert(&mut self, mut status: Status) {
        if let Some(already) = self.inner.take(&status.x_intersect) {
            status.segments.extend(&already.segments);
        }
        self.inner.insert(status);
    }

    pub fn left(&self, x: impl Into<OrderedFloat>) -> Option<&Status> {
        let x = &x.into();
        let mut range = self
            .inner
            .range::<OrderedFloat, _>((Bound::Unbounded, Bound::Excluded(x)));
        range.next_back()
    }

    pub fn right(&self, x: impl Into<OrderedFloat>) -> Option<&Status> {
        let x = &x.into();
        let mut range = self
            .inner
            .range::<OrderedFloat, _>((Bound::Excluded(x), Bound::Unbounded));
        range.next()
    }

    pub fn get(&self, x: impl Into<OrderedFloat>) -> Option<&Status> {
        self.inner.get::<OrderedFloat>(&x.into())
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl IntoIterator for StatusQueue {
    type Item = Status;

    type IntoIter = std::collections::btree_set::IntoIter<Status>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a> IntoIterator for &'a StatusQueue {
    type Item = &'a Status;

    type IntoIter = std::collections::btree_set::Iter<'a, Status>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl Extend<Status> for StatusQueue {
    fn extend<T: IntoIterator<Item = Status>>(&mut self, iter: T) {
        for v in iter {
            (self.insert(v));
        }
    }
}

impl FromIterator<Status> for StatusQueue {
    fn from_iter<T: IntoIterator<Item = Status>>(iter: T) -> Self {
        let mut ret = Self::new();
        ret.extend(iter);
        ret
    }
}

new_key_type! {struct SQKey;}

type StatusStorage = SlotMap<SQKey, SQNode>;

#[derive(Debug, Clone, Default)]
pub struct MyStatusQueue {
    head: SQKey,
    storage: StatusStorage,
}

#[derive(Debug, Clone, Copy)]

pub struct SQNode {
    left: SQKey,
    right: SQKey,
    data: Data,
    height: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Data {
    Owned(SegmentIdx),
    Left(SegmentIdx),
}

impl Data {
    const fn get(self) -> SegmentIdx {
        match self {
            Self::Owned(segment_idx) | Self::Left(segment_idx) => segment_idx,
        }
    }

    fn is_owned(self, s_idx: SegmentIdx) -> bool {
        match self {
            Self::Owned(segment_idx) => s_idx == segment_idx,
            Self::Left(_) => false,
        }
    }
}

fn intersection(segment: Segment, event: CartesianCoord) -> OrderedFloat {
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

fn get_height(key: SQKey, storage: &StatusStorage) -> usize {
    storage.get(key).map_or(0, |n| n.height)
}

fn get_balance(key: SQKey, storage: &StatusStorage) -> isize {
    storage.get(key).map_or(0, |n| {
        isize::try_from(get_height(n.left, storage)).unwrap()
            - isize::try_from(get_height(n.right, storage)).unwrap()
    })
}

fn right_rotate(y: SQKey, storage: &mut StatusStorage) -> SQKey {
    let x = storage[y].left;
    let t2 = storage[x].right;
    storage[x].right = y;
    storage[y].left = t2;
    set_data(y, storage);
    storage[y].height =
        1 + get_height(storage[y].left, storage).max(get_height(storage[y].right, storage));
    storage[x].height =
        1 + get_height(storage[x].left, storage).max(get_height(storage[x].right, storage));
    x
}

fn left_rotate(x: SQKey, storage: &mut StatusStorage) -> SQKey {
    let y = storage[x].right;
    let t2 = storage[y].left;
    storage[y].left = x;
    set_data(y, storage);
    storage[x].right = t2;
    storage[x].height =
        1 + get_height(storage[x].left, storage).max(get_height(storage[x].right, storage));
    storage[y].height =
        1 + get_height(storage[y].left, storage).max(get_height(storage[y].right, storage));
    y
}

fn set_data(x: SQKey, storage: &mut StatusStorage) {
    if let Some(data) = storage
        .get(x)
        .map(|n| n.left)
        .and_then(|left| storage.get(left).map(|n| n.data.get()))
    {
        storage[x].data = Data::Left(data);
    }
}

fn min_value_node(key: SQKey, storage: &StatusStorage) -> SQKey {
    let Some(mut current) = storage.get(key).map(|_| key) else {
        return key;
    };
    while storage.contains_key(storage[current].left) {
        current = storage[current].left;
    }
    current
}

#[must_use]
fn delete(
    curr_key: SQKey,
    storage: &mut StatusStorage,
    s_idx: SegmentIdx,
    segments: &Segments,
    event: CartesianCoord,
) -> SQKey {
    let Some(curr) = storage.get(curr_key).copied() else {
        // We do not exist anymore
        return curr_key;
    };

    // We are the searched Object, remove it.
    if curr.data.is_owned(s_idx) {
        return match (
            storage.contains_key(curr.left),
            storage.contains_key(curr.right),
        ) {
            (false, _) => {
                let temp = curr.right;
                storage.remove(curr_key);
                temp
            }
            (true, false) => {
                let temp = curr.left;
                storage.remove(curr_key);
                temp
            }
            (true, true) => {
                let temp = storage[min_value_node(curr.right, storage)];
                storage[curr_key].data = temp.data;
                storage[curr_key].right =
                    delete(curr.right, storage, temp.data.get(), segments, event);
                return update_balance(storage, curr_key);
            }
        };
    }

    // Go further in our tree
    let curr_seg = segments[curr.data.get()];
    let to_remove = segments[s_idx];

    match compare(to_remove, curr_seg, event) {
        Ordering::Less => {
            let left = delete(curr.left, storage, s_idx, segments, event);
            storage[curr_key].left = left;
            set_data(curr_key, storage);
        }
        Ordering::Greater => {
            let right = delete(curr.right, storage, s_idx, segments, event);
            storage[curr_key].right = right;
        }
        Ordering::Equal => {
            unreachable!(
                "{curr:?}: should remove {to_remove:?}, but current segment is {curr_seg:?}"
            );
        }
    }
    update_balance(storage, curr_key)
}

#[must_use]
fn insert(
    curr_key: SQKey,
    storage: &mut StatusStorage,
    s_idx: SegmentIdx,
    segments: &Segments,
    event: CartesianCoord,
) -> SQKey {
    // We check if curr_key is some and present in storage
    let Some(curr_key) = storage.contains_key(curr_key).then_some(curr_key) else {
        return storage.insert(SQNode {
            left: SQKey::null(),
            right: SQKey::null(),
            data: Data::Owned(s_idx),
            height: 0,
        });
    };

    // Safe because we checked before
    let curr = storage[curr_key];
    if s_idx == curr.data.get() {
        // Already in, nothing to do
        return curr_key;
    }

    let curr_s = segments[curr.data.get()];
    let to_insert = segments[s_idx];

    match compare(to_insert, curr_s, event) {
        Ordering::Less => {
            let left = insert(curr.left, storage, s_idx, segments, event);
            storage[curr_key].left = left;
            set_data(curr_key, storage);
        }
        Ordering::Greater => {
            let right = insert(curr.right, storage, s_idx, segments, event);
            storage[curr_key].right = right;
        }
        Ordering::Equal => {
            unreachable!("{curr:?}: should insert {to_insert:?}, but current segment is {curr_s:?}")
        }
    }
    update_balance(storage, curr_key)
}

fn compare(lhs: Segment, rhs: Segment, event: CartesianCoord) -> Ordering {
    intersection(lhs, event)
        .cmp(&intersection(rhs, event))
        .then_with(|| {
            // Compare Upper point
            let lhs_e = Event::new(lhs.upper.y, lhs.upper.x, iter::empty());
            let rhs_e = Event::new(rhs.upper.y, rhs.upper.x, iter::empty());
            lhs_e.cmp(&rhs_e)
        })
        .then_with(|| {
            // Last compare ids
            lhs.id.cmp(&rhs.id)
        })
}

fn update_balance(storage: &mut StatusStorage, curr_key: SQKey) -> SQKey {
    let curr = storage[curr_key];
    storage[curr_key].height = get_height(curr.left, storage) + get_height(curr.right, storage);
    let balance = get_balance(curr_key, storage);
    let balance_left = get_balance(curr.left, storage);
    let balance_right = get_balance(curr.right, storage);
    if balance > 1 && balance_left >= 0 {
        return right_rotate(curr_key, storage);
        //return right
    }
    if balance > 1 && balance_left < 0 {
        storage[curr_key].left = left_rotate(storage[curr_key].left, storage);
        set_data(curr_key, storage);
        return right_rotate(curr_key, storage);
    }
    if balance < -1 && balance_right <= 0 {
        return left_rotate(curr_key, storage);
        // left rotate
    }

    if balance < -1 && balance_right > 0 {
        storage[curr_key].right = left_rotate(storage[curr_key].right, storage);
        return right_rotate(curr_key, storage);
    }
    curr_key
}

impl MyStatusQueue {
    #[must_use]
    pub fn new() -> Self {
        Self {
            head: SQKey::null(),
            storage: SlotMap::with_key(),
        }
    }

    pub fn insert(&mut self, s_idx: SegmentIdx, segments: &Segments, event: CartesianCoord) {
        self.head = insert(self.head, &mut self.storage, s_idx, segments, event);
    }

    pub fn delete(&mut self, s_idx: SegmentIdx, segments: &Segments, event: CartesianCoord) {
        self.head = delete(self.head, &mut self.storage, s_idx, segments, event);
    }

    fn inner_left_most(&self, segments: &Segments, event: CartesianCoord) -> Option<SQKey> {
        let mut curr = self.head;
        let mut left_most = None;

        while let Some(node) = self.storage.get(curr).copied() {
            match intersection(segments[node.data.get()], event).cmp(&OrderedFloat(event.x.into()))
            {
                Ordering::Less => {
                    if left_most.is_some() {
                        // We are no longer in the event point, so we can stop
                        break;
                    }
                    // We have to search right of it
                    curr = node.right;
                }
                Ordering::Equal => {
                    // Found x
                    left_most = Some(curr);
                    // Search left for even smaller
                    curr = node.left;
                }
                Ordering::Greater => {
                    if left_most.is_some() {
                        // We are no longer in the event point, so we can stop
                        break;
                    }
                    // We have to search left of it
                    curr = node.left;
                }
            }
        }

        left_most
    }

    fn inner_left_most_w_parents(
        &self,
        segments: &Segments,
        event: CartesianCoord,
        parents: &mut Vec<SQKey>,
    ) -> Option<SQKey> {
        let mut curr = self.head;
        let mut left_most = None;

        while let Some(node) = self.storage.get(curr).copied() {
            match intersection(segments[node.data.get()], event).cmp(&OrderedFloat(event.x.into()))
            {
                Ordering::Less => {
                    if left_most.is_some() {
                        // We are no longer in the event point, so we can stop
                        break;
                    }
                    parents.push(curr);
                    // We have to search right of it
                    curr = node.right;
                }
                Ordering::Equal => {
                    // Found x
                    left_most = Some(curr);
                    // Search left for even smaller
                    parents.push(curr);
                    curr = node.left;
                }
                Ordering::Greater => {
                    if left_most.is_some() {
                        // We are no longer in the event point, so we can stop
                        break;
                    }
                    // We have to search left of it
                    parents.push(curr);
                    curr = node.left;
                }
            }
        }

        parents.pop();
        left_most
    }

    fn inner_right_most(&self, segments: &Segments, event: CartesianCoord) -> Option<SQKey> {
        let mut curr = self.head;
        let mut right_most = None;

        while let Some(node) = self.storage.get(curr).copied() {
            match intersection(segments[node.data.get()], event).cmp(&OrderedFloat(event.x.into()))
            {
                Ordering::Less => {
                    if right_most.is_some() {
                        // We are no longer in the event point, so we can stop
                        break;
                    }

                    // We have to search right of it
                    curr = node.right;
                }
                Ordering::Equal => {
                    // Found x
                    right_most = Some(curr);
                    // Search right for even greater
                    curr = node.right;
                }
                Ordering::Greater => {
                    if right_most.is_some() {
                        // We are no longer in the event point, so we can stop
                        break;
                    }
                    // We have to search left of it
                    curr = node.left;
                }
            }
        }

        right_most
    }

    #[must_use]
    pub fn left(&self, segments: &Segments, event: CartesianCoord) -> Option<SegmentIdx> {
        self.inner_left_most(segments, event)
            .and_then(|s| self.storage.get(s))
            .and_then(|s| self.storage.get(s.left).map(|n| n.data.get()))
    }

    #[must_use]
    pub fn right(&self, segments: &Segments, event: CartesianCoord) -> Option<SegmentIdx> {
        self.inner_right_most(segments, event)
            .and_then(|s| self.storage.get(s))
            .and_then(|s| self.storage.get(s.right).map(|n| n.data.get()))
    }

    #[must_use]
    pub fn left_most(&self, segments: &Segments, event: CartesianCoord) -> Option<SegmentIdx> {
        self.inner_left_most(segments, event)
            .map(|k| self.storage[k].data.get())
    }
    #[must_use]
    pub fn right_most(&self, segments: &Segments, event: CartesianCoord) -> Option<SegmentIdx> {
        self.inner_right_most(segments, event)
            .map(|k| self.storage[k].data.get())
    }

    #[must_use]
    pub fn iter_contains<'a, 'b>(
        &'a self,
        segments: &'b Segments,
        event: CartesianCoord,
    ) -> SQContainedIterator<'a, 'b> {
        let mut parents = Vec::new();
        let node = self
            .inner_left_most_w_parents(segments, event, &mut parents)
            .map(|k| self.storage[k]);

        SQContainedIterator {
            inner: SQIterator {
                node,
                storage: &self.storage,
                parents,
            },
            segments,
            event,
        }
    }
}
#[derive(Debug, Clone)]
pub struct SQContainedIterator<'a, 'b> {
    inner: SQIterator<'a>,
    segments: &'b Segments,
    event: CartesianCoord,
}

impl Iterator for SQContainedIterator<'_, '_> {
    type Item = SegmentIdx;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.inner.next()?;
        let idx = node.data.get();
        let x = intersection(self.segments[idx], self.event);
        if f_eq!(*x.0, self.event.x) {
            return Some(idx);
        }
        None
    }
}

#[derive(Debug, Clone)]
pub struct SQIterator<'a> {
    // Next Node
    node: Option<SQNode>,
    storage: &'a StatusStorage,
    parents: Vec<SQKey>,
}

impl Iterator for SQIterator<'_> {
    type Item = SQNode;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.node.take()?;

        self.node = find_next_node(node, self.storage, &mut self.parents);

        Some(node)
    }
}

fn find_next_node(
    node: SQNode,
    storage: &StatusStorage,
    parents: &mut Vec<SQKey>,
) -> Option<SQNode> {
    if storage.contains_key(node.right) {
        let key = min_value_node_w_parents(node.right, storage, parents);
        return Some(storage[key]);
    }
    if let Some(parent) = parents.pop() {
        let next = parent_child(storage[parent], storage, parents)?;
        return Some(next);
    }
    None
}

fn parent_child(node: SQNode, storage: &StatusStorage, parents: &mut Vec<SQKey>) -> Option<SQNode> {
    if let Some(node) = storage.get(node.right) {
        let key = min_value_node_w_parents(node.right, storage, parents);
        return Some(storage[key]);
    }

    if let Some(parent) = parents.pop() {
        parent_child(storage[parent], storage, parents);
    }

    None
}

fn min_value_node_w_parents(
    key: SQKey,
    storage: &StatusStorage,
    parents: &mut Vec<SQKey>,
) -> SQKey {
    let Some(mut current) = storage.get(key).map(|_| key) else {
        return key;
    };
    while storage.contains_key(storage[current].left) {
        parents.push(current);
        current = storage[current].left;
    }
    current
}

#[cfg(test)]
mod tests {
    use googletest::prelude::*;

    use super::*;

    #[gtest]
    fn test_name() {
        let mut sq = MyStatusQueue::new();

        let segments = Segments::from_iter([Segment::new((2, 2), (-2, -2))]);

        let event = (2, 2).into();

        sq.insert(0.into(), &segments, event);

        let iter : Vec<_> = sq.iter_contains(&segments, event).collect();

        expect_that!(iter, elements_are![eq(&0.into())]);
    }
}
