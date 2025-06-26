use std::collections::{BTreeSet, HashSet};

use common::{math::Float, segment::SegmentIdx};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Event {
    pub y: ordered_float::OrderedFloat<Float>,
    pub x: ordered_float::OrderedFloat<Float>,
    pub segments: HashSet<SegmentIdx>,
}

impl Event {
    #[must_use]
    pub fn new(
        y: impl Into<ordered_float::OrderedFloat<Float>>,
        x: impl Into<ordered_float::OrderedFloat<Float>>,
        segments: impl Iterator<Item = SegmentIdx>,
    ) -> Self {
        Self {
            y: y.into(),
            x: x.into(),
            segments: segments.collect(),
        }
    }

    #[must_use]
    pub fn cheap_copy(&self) -> Self {
        Self {
            y: self.y,
            x: self.x,
            segments: HashSet::new(),
        }
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.y.cmp(&other.y).reverse().then(self.x.cmp(&other.x))
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EventQueue {
    pub queue: BTreeSet<Event>,
}

impl EventQueue {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            queue: BTreeSet::new(),
        }
    }

    pub fn insert(&mut self, mut event: Event) {
        let entry = self.queue.get(&event.cheap_copy());
        if let Some(entry) = entry {
            event.segments.extend(entry.segments.iter());
        }
        self.queue.replace(event);
    }

    pub fn pop(&mut self) -> Option<Event> {
        self.queue.pop_first()
    }
}