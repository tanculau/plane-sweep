use core::cmp::Ordering;

use common::{
    math::{ cartesian::CartesianCoord},
    segment::{SegmentIdx, Segments},
};
use itertools::Itertools;
use slotmap::Key;

use crate::status::{SQDebug, SQKey, Storage, compare, compare2, compare3, intersection};

#[derive(Debug, Clone, Copy)]
pub enum Node {
    Node {
        data: SegmentIdx,
        parent: Option<SQKey>,
        left: SQKey,
        right: SQKey,
        height: usize,
    },
    Leaf {
        parent: Option<SQKey>,
    },
}

impl Node {
    pub fn empty(parent: impl Into<Option<SQKey>>, storage: &mut Storage) -> SQKey {
        let node = Self::Leaf {
            parent: parent.into(),
        };
        storage.insert(node)
    }
    fn node(
        data: impl Into<SegmentIdx>,
        parent: impl Into<Option<SQKey>>,
        storage: &mut Storage,
    ) -> SQKey {
        let node = Self::Node {
            data: data.into(),
            parent: parent.into(),
            left: SQKey::null(),
            right: SQKey::null(),
            height: 1,
        };
        let ret = storage.insert(node);
        let dummy1 = Self::empty(ret, storage);
        let tmp = storage[ret].set_left(dummy1);
        debug_assert!(
            tmp,
            "Could not create new node, since the left child could not be set"
        );
        let dummy2 = Self::empty(ret, storage);
        let tmp = storage[ret].set_right(dummy2);
        debug_assert_ne!(dummy1, dummy2);
        debug_assert!(
            tmp,
            "Could not create new node, since the right child could not be set"
        );

        ret
    }

    #[must_use]
    pub const fn is_node(self) -> bool {
        matches!(self, Self::Node { .. })
    }

    pub fn data(self) -> Option<SegmentIdx> {
        match self {
            Self::Node { data, .. } => data.into(),
            Self::Leaf { .. } => None,
        }
    }
    const fn parent(self) -> Option<SQKey> {
        match self {
            Self::Node { parent, .. } | Self::Leaf { parent, .. } => parent,
        }
    }

    #[must_use]
    const fn set_left(&mut self, new: SQKey) -> bool {
        match self {
            Self::Node { left, .. } => {
                *left = new;
                true
            }
            Self::Leaf { .. } => false,
        }
    }
    #[must_use]
    const fn set_right(&mut self, new: SQKey) -> bool {
        match self {
            Self::Node { right, .. } => {
                *right = new;
                true
            }
            Self::Leaf { .. } => false,
        }
    }

    const fn left(self) -> Option<SQKey> {
        match self {
            Self::Node { left, .. } => Some(left),
            Self::Leaf { .. } => None,
        }
    }
    const fn right(self) -> Option<SQKey> {
        match self {
            Self::Node { right, .. } => Some(right),
            Self::Leaf { .. } => None,
        }
    }

    /// Will always return a Leaf
    pub fn left_most(key: SQKey, storage: &Storage) -> SQKey {
        let mut curr = key;

        while let Some(left) = storage[curr].left() {
            curr = left;
        }
        curr
    }
    pub fn left_most_node(key: SQKey, storage: &Storage) -> SQKey {
        let mut curr = key;

        while let Some(left) = storage[curr].left() {
            curr = left;
        }
        storage[curr].parent().unwrap_or(curr)
    }
    /// Will always return a Leaf
    fn right_most(key: SQKey, storage: &Storage) -> SQKey {
        let mut curr = key;

        while let Some(right) = storage[curr].right() {
            curr = right;
        }
        curr
    }

    const fn set_height(&mut self, new_height: usize) {
        if let Self::Node { height, .. } = self {
            *height = new_height;
        }
    }

    const fn height(self) -> usize {
        match self {
            Self::Node { height, .. } => height,
            Self::Leaf { .. } => 0,
        }
    }

    const fn set_data(&mut self, new_data: SegmentIdx) -> bool {
        match self {
            Self::Node { data, .. } => {
                *data = new_data;
                true
            }
            Self::Leaf { .. } => false,
        }
    }

    #[allow(clippy::cast_possible_wrap)]
    fn balance(self, storage: &Storage) -> isize {
        match self {
            Self::Node { left, right, .. } => {
                storage[left].height() as isize - storage[right].height() as isize
            }
            Self::Leaf { .. } => 0,
        }
    }

    pub fn find_left_of_event(
        curr_key: SQKey,
        storage: &Storage,
        segments: &Segments,
        event: CartesianCoord,
    ) -> Option<SQKey> {
        let curr = storage[curr_key];

        if let Some(seg) = curr.data() {
            match intersection(segments[seg].clone(), event.clone()).cmp(&(event.x.into())) {
                Ordering::Less => {
                    if let Some(right) = curr.right()
                        && let Some(v) = Self::find_left_of_event(right, storage, segments, event)
                    {
                        return Some(v);
                    }
                    return Some(curr_key);
                }
                Ordering::Equal | Ordering::Greater => {
                    return Self::find_left_of_event(curr.left()?, storage, segments, event);
                }
            }
        }
        None
    }

    pub fn find_right_of_event(
        curr_key: SQKey,
        storage: &Storage,
        segments: &Segments,
        event: CartesianCoord,
    ) -> Option<SQKey> {
        let curr = storage[curr_key];

        if let Some(seg) = curr.data() {
            match intersection(segments[seg].clone(), event.clone()).cmp(&(event.x.into())) {
                Ordering::Equal | Ordering::Less => {
                    return Self::find_right_of_event(curr.right()?, storage, segments, event);
                }
                Ordering::Greater => {
                    if let Some(left) = curr.left()
                        && let Some(v) = Self::find_right_of_event(left, storage, segments, event)
                    {
                        return Some(v);
                    }
                    return Some(curr_key);
                }
            }
        }
        None
    }

    pub fn find_left_most(
        curr_key: SQKey,
        storage: &Storage,
        segments: &Segments,
        event: CartesianCoord,
    ) -> Option<SQKey> {
        let curr = storage[curr_key];
        if let Some(seg) = curr.data() {
            match intersection(segments[seg].clone(), event.clone()).cmp(&(event.x.into())) {
                Ordering::Less => {
                    return Self::find_left_most(curr.right()?, storage, segments, event);
                }
                Ordering::Equal => {
                    if let Some(left) = curr.left()
                        && let Some(s) = Self::find_left_most(left, storage, segments, event)
                    {
                        return Some(s);
                    }
                    return Some(curr_key);
                }
                Ordering::Greater => {
                    return Self::find_left_most(curr.left()?, storage, segments, event);
                }
            }
        }

        None
    }

    pub fn find_right_most(
        curr_key: SQKey,
        storage: &Storage,
        segments: &Segments,
        event: CartesianCoord,
    ) -> Option<SQKey> {
        let curr = storage[curr_key];
        if let Some(seg) = curr.data() {
            match intersection(segments[seg].clone(), event.clone()).cmp(&(event.x.into())) {
                Ordering::Less => {
                    return Self::find_right_most(curr.right()?, storage, segments, event);
                }
                Ordering::Equal => {
                    if let Some(right) = curr.right()
                        && let Some(s) = Self::find_right_most(right, storage, segments, event)
                    {
                        return Some(s);
                    }
                    return Some(curr_key);
                }
                Ordering::Greater => {
                    return Self::find_right_most(curr.left()?, storage, segments, event);
                }
            }
        }

        None
    }

    pub fn delete(
        curr_key: SQKey,
        storage: &mut Storage,
        s_idx: SegmentIdx,
        segments: &Segments,
        event: CartesianCoord,
    ) -> SQKey {
        let curr = storage[curr_key];
        let (Some(curr_seg), Some(left_key), Some(right_key)) =
            (curr.data(), curr.left(), curr.right())
        else {
            return curr_key;
        };
        //Self::verify_with_event(curr_key, storage, segments, event);

        match compare3(curr_seg, s_idx, segments, event.clone()) {
            Ordering::Less => {
                let right = Self::delete(right_key, storage, s_idx, segments, event);
                let tmp = storage[curr_key].set_right(right);
                debug_assert!(tmp);
                storage[right].set_parent(curr_key);
            }
            Ordering::Greater => {
                let left = Self::delete(left_key, storage, s_idx, segments, event);
                let tmp = storage[curr_key].set_left(left);
                debug_assert!(tmp);
                storage[left].set_parent(curr_key);
            }
            Ordering::Equal => match (storage[left_key].is_node(), storage[right_key].is_node()) {
                (false, _) => {
                    storage[right_key].set_parent(curr.parent());
                    storage.remove(left_key);
                    storage.remove(curr_key);
                    return right_key;
                }
                (true, false) => {
                    storage[left_key].set_parent(curr.parent());
                    storage.remove(right_key);
                    storage.remove(curr_key);
                    return left_key;
                }
                (true, true) => {
                    let temp = Self::left_most(right_key, storage);
                    let temp = storage[temp].parent().unwrap();
                    let temp_node = storage[temp];
                    debug_assert!(temp != curr_key);
                    debug_assert!(temp_node.is_node());
                    let temp_data = temp_node.data().unwrap();
                    storage[curr_key].set_data(temp_data);
                    let new_right = Self::delete(right_key, storage, temp_data, segments, event);
                    storage[new_right].set_parent(curr_key);
                    let res = storage[curr_key].set_right(new_right);
                    debug_assert!(res);
                }
            },
        }

        let ret = Self::update_balance(curr_key, storage);

        //Self::verify_with_event(curr_key, storage, segments, event);

        ret
    }

    pub fn insert(
        curr: SQKey,
        storage: &mut Storage,
        s_idx: SegmentIdx,
        segments: &Segments,
        event: CartesianCoord,
    ) -> SQKey {
        // Already in
        if let Some(data) = storage[curr].data()
            && data == s_idx
        {
            return curr;
        }

        match storage[curr] {
            // We are a Leaf, we can just insert it
            Self::Leaf { parent } => {
                storage.remove(curr);
                Self::node(s_idx, parent, storage)
            }
            Self::Node {
                data, left, right, ..
            } => {
                //Self::verify_with_event(curr, storage, segments, event);

                let curr_seg = segments[data].clone();
                let insert_seg = segments[s_idx].clone();

                match compare(insert_seg, curr_seg, event.clone()) {
                    Ordering::Less => {
                        let new_left = Self::insert(left, storage, s_idx, segments, event);
                        let tmp = storage[curr].set_left(new_left);
                        debug_assert!(tmp, "Could not set left child in insert");
                        storage[new_left].set_parent(curr);
                        debug_assert_ne!(new_left, right);
                    }
                    Ordering::Equal => unreachable!(),
                    Ordering::Greater => {
                        let new_right = Self::insert(right, storage, s_idx, segments, event);
                        let tmp = storage[curr].set_right(new_right);
                        debug_assert!(tmp, "Could not set right child in insert");
                        storage[new_right].set_parent(curr);
                        debug_assert_ne!(left, new_right);
                    }
                }

                let ret = Self::update_balance(curr, storage);
                //Self::verify_with_event(ret, storage, segments, event);

                ret
            }
        }
    }

    pub fn set_parent(&mut self, new_parent: impl Into<Option<SQKey>>) {
        match self {
            Self::Node { parent, .. } | Self::Leaf { parent } => *parent = new_parent.into(),
        }
    }

    fn right_rotate(curr: SQKey, storage: &mut Storage) -> SQKey {
        let old_parent = storage[curr].parent();
        let y = curr;
        let x = storage[y].left().unwrap();
        let t2 = storage[x].right().unwrap();

        let tmp = storage[x].set_right(y);
        debug_assert!(tmp, "Could not set right");
        storage[y].set_parent(x);
        let tmp = storage[y].set_left(t2);
        debug_assert!(tmp, "Could not set left");
        storage[t2].set_parent(y);
        Self::update_height(y, storage);
        Self::update_height(x, storage);
        storage[x].set_parent(old_parent);
        x
    }
    fn left_rotate(curr: SQKey, storage: &mut Storage) -> SQKey {
        Self::verify(curr, storage);
        let old_parent = storage[curr].parent();
        let x = curr;
        let y = storage[x].right().unwrap();
        let t2 = storage[y].left().unwrap();
        let tmp = storage[y].set_left(x);
        debug_assert!(tmp, "Could not set left");
        storage[x].set_parent(y);
        let tmp = storage[x].set_right(t2);
        debug_assert!(tmp, "Could not set left");
        storage[t2].set_parent(x);
        storage[y].set_parent(old_parent);
        Self::update_height(t2, storage);
        Self::update_height(x, storage);
        Self::update_height(y, storage);
        Self::verify(curr, storage);
        y
    }

    fn update_balance(curr: SQKey, storage: &mut Storage) -> SQKey {
        let curr_node = storage[curr];
        let Self::Node {
            left: left_key,
            right: right_key,
            data: _data,
            parent: _parent,
            ..
        } = curr_node
        else {
            return curr;
        };
        let left = storage[left_key];
        let right = storage[right_key];
        Self::update_height(curr, storage);
        Self::verify(curr, storage);

        let balance = curr_node.balance(storage);

        if balance > 1 {
            if left.balance(storage) >= 0 {
                let ret = Self::right_rotate(curr, storage);
                Self::update_height(ret, storage);
                return ret;
            }
            let new_left = Self::left_rotate(left_key, storage);
            let tmp = storage[curr].set_left(new_left);
            Self::update_height(new_left, storage);
            debug_assert!(tmp, "Could not set left");
            storage[new_left].set_parent(curr);
            let ret = Self::right_rotate(curr, storage);
            Self::update_height(ret, storage);
            return ret;
        }

        if balance < -1 {
            if right.balance(storage) <= 0 {
                let ret = Self::left_rotate(curr, storage);
                Self::update_height(ret, storage);
                return ret;
            }

            let new_right = Self::right_rotate(right_key, storage);
            Self::update_height(new_right, storage);
            let tmp = storage[curr].set_right(new_right);
            debug_assert!(tmp, "Could not set left");
            storage[new_right].set_parent(curr);
            let ret = Self::left_rotate(curr, storage);
            Self::update_height(ret, storage);
            return ret;
        }
        Self::update_height(curr, storage);
        curr
    }

    fn update_height(curr: SQKey, storage: &mut Storage) {
        let val = if let Self::Node { left, right, .. } = storage[curr] {
            1 + storage[left].height() + storage[right].height()
        } else {
            0
        };
        storage[curr].set_height(val);
    }

    #[inline]
    pub fn verify(curr: SQKey, storage: &Storage) {
        if cfg!(debug_assertions) {
            let node = storage[curr];
            match node {
                Self::Node {
                    left,
                    right,
                    height,
                    parent,
                    ..
                } => {
                    debug_assert_eq!(
                        [curr, left, right]
                            .iter()
                            .chain(parent.iter())
                            .duplicates()
                            .count(),
                        0
                    );

                    debug_assert_eq!(
                        Some(curr),
                        storage[left].parent(),
                        "{}",
                        SQDebug::new(curr, storage)
                    );
                    debug_assert_eq!(
                        Some(curr),
                        storage[right].parent(),
                        "{}",
                        SQDebug::new(curr, storage)
                    );
                    debug_assert_eq!(
                        1 + storage[left].height() + storage[right].height(),
                        height,
                        "{}",
                        SQDebug::new(curr, storage)
                    );
                    Self::verify(left, storage);
                    Self::verify(right, storage);
                }
                Self::Leaf { .. } => {}
            }
        }
    }

    pub fn verify_with_event(
        curr: SQKey,
        storage: &Storage,
        segments: &Segments,
        event: CartesianCoord,
    ) {
        if cfg!(debug_assertions) {
            let node = storage[curr];
            match node {
                Self::Node {
                    left,
                    right,
                    height,
                    data,
                    parent,
                    ..
                } => {
                    debug_assert_eq!(
                        [curr, left, right]
                            .iter()
                            .chain(parent.iter())
                            .duplicates()
                            .count(),
                        0
                    );
                    debug_assert_eq!(
                        Some(curr),
                        storage[left].parent(),
                        "{}",
                        SQDebug::new(curr, storage)
                    );
                    debug_assert_eq!(
                        Some(curr),
                        storage[right].parent(),
                        "{}",
                        SQDebug::new(curr, storage)
                    );
                    if let Some(lhs) = storage[left].data() {
                        debug_assert_eq!(
                            compare2(lhs, data, segments, event.clone()),
                            Ordering::Less,
                            "Expected {lhs:?} to be smaller than {data:?}, but {:?} < {:?}, {:?} < {:?}, {:?}, {:?}",
                            intersection(segments[lhs].clone(), event.clone()),
                            intersection(segments[data].clone(), event),
                            segments[lhs].slope(),
                            segments[data].slope(),
                            segments[lhs].line(),
                            segments[data].line()
                        );
                    }
                    if let Some(rhs) = storage[right].data() {
                        debug_assert_eq!(
                            compare2(rhs, data, segments, event.clone()),
                            Ordering::Greater,
                            "Expected {rhs:?} to be greater than {data:?}, but {:?} > {:?}, {:?} > {:?}, {:?}, {:?}",
                            intersection(segments[rhs].clone(), event.clone()),
                            intersection(segments[data].clone(), event),
                            segments[rhs].slope(),
                            segments[data].slope(),
                            segments[rhs].line(),
                            segments[data].line()
                        );
                    }
                    debug_assert_eq!(
                        1 + storage[left].height() + storage[right].height(),
                        height,
                        "{}",
                        SQDebug::new(curr, storage)
                    );
                    Self::verify(left, storage);
                    Self::verify(right, storage);
                }
                Self::Leaf { .. } => {}
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct NodeCursor<'a> {
    curr: SQKey,
    storage: &'a Storage,
}

impl<'a> NodeCursor<'a> {
    pub const fn new(curr: SQKey, storage: &'a Storage) -> Self {
        Self { curr, storage }
    }

    pub fn access(&self) -> Node {
        self.storage[self.curr]
    }

    fn get_next(&'a self) -> SQKey {
        if let Some(right) = self.access().right()
            && let Some(left_most) = self.storage[Node::left_most(right, self.storage)].parent()
            && left_most != self.curr
        {
            return left_most;
        }

        let mut parent = self.curr;

        while let Some(p) = self.storage[parent].parent() {
            let parent_left = self.storage[p].left().unwrap();
            // I am coming from left, so I can return the value
            if parent_left == parent {
                return p;
            }

            // I am coming from right, so I have to keep searching
            parent = p;
        }

        self.curr
    }
    fn get_prev(&self) -> SQKey {
        if let Some(left) = self.access().left()
            && let right_most = Node::right_most(left, self.storage)
            && let Some(p) = self.storage[right_most].parent()
            && p != self.curr
        {
            return p;
        }

        let mut parent = self.curr;

        while let Some(p) = self.storage[parent].parent() {
            // I am coming from left, so I can return the value
            if self.storage[p].right().unwrap() == parent {
                return p;
            }

            // I am coming from right, so I have to keep searching
            parent = p;
        }
        self.curr
    }

    pub const fn curr(&self) -> SQKey {
        self.curr
    }

    pub fn peek_next(&self) -> Option<SQKey> {
        let v = self.get_next();
        (self.curr != v).then_some(v)
    }

    pub fn peek_prev(&self) -> Option<SQKey> {
        let v = self.get_prev();
        (self.curr != v).then_some(v)
    }

    #[must_use]
    #[cfg_attr(test, mutants::skip)] // Timeout
    pub fn next(&mut self) -> bool {
        if let Some(new) = self.peek_next() {
            self.curr = new;
            true
        } else {
            false
        }
    }
    #[must_use]
    #[allow(dead_code)]
    #[cfg_attr(test, mutants::skip)] // dead_code
    pub fn prev(&mut self) -> bool {
        if let Some(new) = self.peek_prev() {
            self.curr = new;
            true
        } else {
            false
        }
    }
}

// fn debug_iter(curr: SQKey, storage: &Storage) -> impl Iterator<Item = (SegmentIdx, SQKey)> + Unpin {
//     gen move {
//         if let Some(left) = storage[curr].left() {
//             let mut iter = Box::pin(debug_iter(left, storage));
//             while let Some(i) = iter.as_mut().next() {
//                 yield i;
//             }
//         }
//         if let Some(data) = storage[curr].data() {
//             yield (data, curr);
//         }
//         if let Some(right) = storage[curr].right() {
//             let mut iter = Box::pin(debug_iter(right, storage));
//             while let Some(i) = iter.as_mut().next() {
//                 yield i;
//             }
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;

    #[test]
    fn test_name() {}
}
