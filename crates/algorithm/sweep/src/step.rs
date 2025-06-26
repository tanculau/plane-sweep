use core::iter;

use bon::Builder;
use common::{intersection::IntersectionIdx, math::{cartesian::CartesianCoord, OrderedFloat}, segment::SegmentIdx, AlgrorithmStep};

use crate::{event::{Event, EventQueue}, status::StatusQueue};


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
    pub status_queue: StatusQueue,
    #[builder(default)]
    pub u_p: Vec<SegmentIdx>,
    #[builder(default)]
    pub c_p: Vec<SegmentIdx>,
    #[builder(default)]
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
    HEPUpdateT,
    CalculateSets,
    CalculateUpCpLp {
        up_cp_lp: Vec<SegmentIdx>,
    },
    ReportIntersections,
    DeleteLp,
    InsertUp,
    UpCpEmpty {
        s_l: Vec<SegmentIdx>,
        s_r: Vec<SegmentIdx>,
    },
    UpCpNotEmpty {
        s_dash: Option<SegmentIdx>,
        s_dash_dash: Option<SegmentIdx>,
        s_l: Vec<SegmentIdx>,
        s_r: Vec<SegmentIdx>,
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
            self,
            Self::HEPUpdateT
                | Self::CalculateSets
                | Self::CalculateUpCpLp { .. }
                | Self::ReportIntersections
                | Self::DeleteLp
                | Self::InsertUp
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
