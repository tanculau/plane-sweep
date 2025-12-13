use core::iter;

use bon::Builder;
use common::{
    AlgrorithmStep,
    intersection::{InterVec, LeanIntersectionIdx},
    math::{A, cartesian::CartesianCoord},
    segment::SegmentIdx,
};
use itertools::chain;
use sweep_utils::{
    event::EventQueue,
    ui::{events_view::EventReport, set_view::SetReport, status_view::StatusReport},
};

use crate::SortedIntersections;

#[derive(Builder, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[builder(on(usize, into))]
#[allow(clippy::struct_field_names)]
pub struct Step<T: A> {
    #[builder(start_fn)]
    pub typ: StepType<T>,
    #[builder(start_fn)]
    pub step: usize,
    pub p: Option<CartesianCoord<T>>,
    #[builder(default)]
    pub event_queue: EventQueue<T>,
    #[builder(default)]
    #[builder(with = FromIterator::from_iter)]
    pub status_queue: Vec<SegmentIdx>,
    #[builder(default)]
    #[builder(with = FromIterator::from_iter)]
    pub merge_queue: Vec<([SegmentIdx; 2], SortedIntersections<T>)>,
    #[builder(default)]
    #[builder(with = FromIterator::from_iter)]
    pub u_p: Vec<SegmentIdx>,
    #[builder(default)]
    #[builder(with = FromIterator::from_iter)]
    pub c_p: Vec<SegmentIdx>,
    #[builder(default)]
    #[builder(with = FromIterator::from_iter)]
    pub l_p: Vec<SegmentIdx>,
}

impl<T: A> StatusReport<T> for Step<T> {
    fn status_queue(&self) -> &[SegmentIdx] {
        &self.status_queue
    }

    fn p(&self) -> Option<&CartesianCoord<T>> {
        self.p.as_ref()
    }
}
impl<T: A> EventReport<T> for Step<T> {
    fn event_queue(&self) -> &EventQueue<T> {
        &self.event_queue
    }

    fn p(&self) -> Option<&CartesianCoord<T>> {
        self.p.as_ref()
    }

    fn u_p(&self) -> &[SegmentIdx] {
        &self.u_p
    }
}
impl<T: A> SetReport for Step<T> {
    fn u_p(&self) -> &[SegmentIdx] {
        &self.u_p
    }

    fn c_p(&self) -> &[SegmentIdx] {
        &self.c_p
    }

    fn l_p(&self) -> &[SegmentIdx] {
        &self.l_p
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StepType<T: A> {
    Init,
    StartInitQ,
    InitQ {
        segment: SegmentIdx,
    },
    InitT,
    PopQ,
    CalculateSets,
    CalculateUpCpLp {
        up_cp_lp: InterVec,
    },
    ReportIntersections {
        intersection: LeanIntersectionIdx,
    },
    DeleteLpCp,
    InsertUpCp,
    UpCpEmpty {
        s_l: Option<SegmentIdx>,
        s_r: Option<SegmentIdx>,
    },
    UpCpNotEmpty {
        s_dash: Option<SegmentIdx>,
        s_dash_dash: Option<SegmentIdx>,
        s_l: Option<SegmentIdx>,
        s_r: Option<SegmentIdx>,
    },
    FindNewEvent {
        s_l: SegmentIdx,
        s_r: SegmentIdx,
    },
    InsertIntersectionEvent {
        s_l: SegmentIdx,
        s_r: SegmentIdx,
        intersection: (T, T),
    },
    InsertMergeQueue {
        idx: LeanIntersectionIdx,
    },
    Merge {
        seg: [SegmentIdx; 2],
        result: SortedIntersections<T>,
        idx: LeanIntersectionIdx,
    },
    End,
}
impl<T: A> StepType<T> {
    #[must_use]
    pub const fn is_init(&self) -> bool {
        matches!(self, Self::Init)
    }
    #[must_use]
    pub const fn is_find_intersections(&self) -> bool {
        matches!(
            self,
            Self::StartInitQ | Self::InitQ { .. } | Self::InitT | Self::PopQ
        )
    }
    #[must_use]
    pub const fn is_handle_event_point(&self) -> bool {
        matches!(
            self,
            Self::CalculateSets
                | Self::CalculateUpCpLp { .. }
                | Self::ReportIntersections { .. }
                | Self::DeleteLpCp
                | Self::InsertUpCp
                | Self::UpCpEmpty { .. }
                | Self::UpCpNotEmpty { .. }
        )
    }

    #[must_use]
    pub const fn is_find_new_event(&self) -> bool {
        matches!(
            self,
            Self::FindNewEvent { .. } | Self::InsertIntersectionEvent { .. }
        )
    }

    pub const fn is_merging(&self) -> bool {
        matches!(self, Self::Merge { .. } | Self::InsertMergeQueue { .. })
    }
    #[must_use]
    pub const fn is_finished(&self) -> bool {
        matches!(self, Self::End)
    }
}

impl<T: A> AlgrorithmStep<T> for Step<T> {
    fn segments(&self) -> impl Iterator<Item = common::segment::SegmentIdx> {
        chain!(&self.u_p, &self.c_p, &self.l_p).copied()
    }

    fn intersections(&self) -> impl Iterator<Item = common::intersection::IntersectionIdx> {
        iter::empty()
    }

    fn sweep_line(&self) -> Option<CartesianCoord<T>> {
        self.p.clone()
    }
}
