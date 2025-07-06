use std::collections::{BTreeSet, HashSet};

use common::{
    math::{Float, cartesian::CartesianCoord},
    segment::SegmentIdx,
};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Event {
    pub y: Float,
    pub x: Float,
    pub segments: HashSet<SegmentIdx>,
}

impl Event {
    #[must_use]
    pub fn new(
        y: impl Into<Float>,
        x: impl Into<Float>,
        segments: impl Iterator<Item = SegmentIdx>,
    ) -> Self {
        Self {
            y: y.into(),
            x: x.into(),
            segments: segments.collect(),
        }
    }

    #[must_use]
    #[allow(clippy::clone_on_copy)]
    pub fn cheap_copy(&self) -> Self {
        Self {
            y: self.y.clone(),
            x: self.x.clone(),
            segments: HashSet::new(),
        }
    }

    #[must_use]
    #[allow(clippy::clone_on_copy)]
    pub fn coord(&self) -> CartesianCoord {
        (self.x.clone(), self.y.clone()).into()
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
