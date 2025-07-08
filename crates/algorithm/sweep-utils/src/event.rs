use std::collections::{BTreeMap, HashSet};

use common::{math::cartesian::CartesianCoord, segment::SegmentIdx};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EventQueue {
    pub queue: BTreeMap<CartesianCoord, HashSet<SegmentIdx>>,
}

impl EventQueue {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            queue: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, coord: impl Into<CartesianCoord>, seg: impl Into<Option<SegmentIdx>>) {
        let coord = coord.into();
        let seg = seg.into();
        self.queue
            .entry(coord)
            .and_modify(|v| {
                if let Some(seg) = seg {
                    v.insert(seg);
                }
            })
            .or_insert_with(|| HashSet::from_iter(seg));
    }

    pub fn pop(&mut self) -> Option<(CartesianCoord, HashSet<SegmentIdx>)> {
        self.queue.pop_first()
    }
}
