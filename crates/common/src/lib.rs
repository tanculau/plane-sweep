use typed_index_collections::TiVec;

use crate::{intersection::IntersectionIdx, math::cartesian::CartesianCoord, segment::SegmentIdx};

pub mod intersection;
pub mod math;
pub mod segment;

#[cfg(feature = "ui")]
pub mod ui;

/// Common trait to generalize algorithm.
/// It represents a step in an algorithm that can be iterated over.
pub trait AlgrorithmStep {
    /// Returns the segment that are currently looked at by the algorithm.
    fn segments(&self) -> impl Iterator<Item = SegmentIdx>;
    /// Returns the intersections that are currently looked at by the algorithm.
    fn intersections(&self) -> impl Iterator<Item = IntersectionIdx>;

    fn sweep_line(&self) -> Option<CartesianCoord> {
        None
    }
}

pub type AlgoSteps<T> = TiVec<AlgoStepIdx, T>;

impl_idx!(AlgoStepIdx);
pub trait PushStep<T> {
    fn push(&mut self, step: T);
    fn clear(&mut self);
}

impl<T> PushStep<T> for AlgoSteps<T> {
    fn push(&mut self, step: T) {
        self.push(step);
    }
    fn clear(&mut self) {
        self.clear();
    }
}

impl<T> PushStep<T> for () {
    #[inline(always)]
    fn push(&mut self, _: T) {}
    #[inline(always)]
    fn clear(&mut self) {}
}

#[macro_export]
macro_rules! impl_idx {
    ($t : tt) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[repr(transparent)]
        pub struct $t(usize);

        impl From<usize> for $t {
            fn from(value: usize) -> Self {
                Self(value)
            }
        }
        impl From<&usize> for $t {
            fn from(value: &usize) -> Self {
                Self(*value)
            }
        }
        impl From<&mut usize> for $t {
            fn from(value: &mut usize) -> Self {
                Self(*value)
            }
        }
        impl From<$t> for usize {
            fn from(value: $t) -> Self {
                value.0
            }
        }
        impl From<&$t> for usize {
            fn from(value: &$t) -> Self {
                value.0
            }
        }
        impl From<&mut $t> for usize {
            fn from(value: &mut $t) -> Self {
                value.0
            }
        }
        impl From<&$t> for $t {
            fn from(value: &$t) -> Self {
                *value
            }
        }
        impl From<&mut $t> for $t {
            fn from(value: &mut $t) -> Self {
                *value
            }
        }
    };
}
