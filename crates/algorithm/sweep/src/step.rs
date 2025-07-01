use core::iter;

use bon::Builder;
use common::{
    AlgrorithmStep,
    intersection::IntersectionIdx,
    math::{OrderedFloat, cartesian::CartesianCoord},
    segment::SegmentIdx,
};

use crate::event::{Event, EventQueue};

#[derive(Builder, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[builder(on(usize, into))]
pub struct Step {
    #[builder(start_fn)]
    pub typ: StepType,
    #[builder(start_fn)]
    pub step: usize,
    pub event: Option<Event>,
    #[builder(default)]
    pub event_queue: EventQueue,
    #[builder(default)]
    #[builder(with = FromIterator::from_iter)]
    pub status_queue: Vec<SegmentIdx>,
    #[builder(default)]
    #[builder(with = FromIterator::from_iter)]
    pub u_p: Vec<SegmentIdx>,
    #[builder(default)]
    #[builder(with = FromIterator::from_iter)]
    pub c_p: Vec<SegmentIdx>,
    #[builder(default)]
    #[builder(with = FromIterator::from_iter)]
    pub l_p: Vec<SegmentIdx>,
    pub intersection: Option<IntersectionIdx>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StepType {
    Init,
    StartInitQ,
    InitQ {
        segment: SegmentIdx,
    },
    InitT,
    PopQ,
    CalculateSets,
    CalculateUpCpLp {
        up_cp_lp: Vec<SegmentIdx>,
    },
    ReportIntersections,
    DeleteLpCp,
    InsertUpCp,
    UpCpEmpty {
        s_l: Option<SegmentIdx>,
        s_r: Option<SegmentIdx>,
    },
    UpCpNotEmpty {
        s_dash: SegmentIdx,
        s_dash_dash: SegmentIdx,
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
        intersection: (OrderedFloat, OrderedFloat),
    },
    End,
}
impl StepType {
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
            self, Self::CalculateSets
                | Self::CalculateUpCpLp { .. }
                | Self::ReportIntersections
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
    #[must_use]
    pub const fn is_finished(&self) -> bool {
        matches!(self, Self::End)
    }
}

impl AlgrorithmStep for Step {
    fn segments(&self) -> impl Iterator<Item = common::segment::SegmentIdx> {
        self.event
            .iter()
            .flat_map(|s| s.segments.iter())
            .chain(self.c_p.iter())
            .chain(self.l_p.iter())
            .copied()
    }

    fn intersections(&self) -> impl Iterator<Item = common::intersection::IntersectionIdx> {
        iter::empty()
    }

    fn sweep_line(&self) -> Option<CartesianCoord> {
        self.event.as_ref().map(|v| (v.x.0, v.y.0).into())
    }
}
