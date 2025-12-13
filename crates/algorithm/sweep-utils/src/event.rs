// Based on the book "Computational Geometry" from Mark Berg , Otfried Cheong , Marc Kreveld , Mark Overmars. [DOI](https://doi.org/10.1007/978-3-662-04245-8)

use std::collections::{BTreeMap, HashSet};

use common::{
    math::{A, cartesian::CartesianCoord},
    segment::SegmentIdx,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EventQueue<T: A> {
    pub queue: BTreeMap<CartesianCoord<T>, HashSet<SegmentIdx>>,
}

impl<T: A> EventQueue<T> {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            queue: BTreeMap::new(),
        }
    }

    pub fn insert(
        &mut self,
        coord: impl Into<CartesianCoord<T>>,
        seg: impl Into<Option<SegmentIdx>>,
    ) {
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

    pub fn pop(&mut self) -> Option<(CartesianCoord<T>, HashSet<SegmentIdx>)> {
        self.queue.pop_first()
    }
}
