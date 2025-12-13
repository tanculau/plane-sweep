use common::math::B;

use crate::{
    Circle, Site,
    beachline::{BeachLine, SQKey},
    dcel::Dcel,
    event_queue::{Event, EventQueue},
};

#[derive(Debug, Clone)]
pub struct Step<T: B> {
    pub typ: StepType<T>,
    pub step: usize,
    pub event_queue: EventQueue<T>,
    pub beachline: BeachLine<T>,
    pub dcel: Dcel<T>,
    pub event: Option<Event<T>>,
}

impl<T: B> Step<T> {
    #[must_use]
    pub fn sweep_line(&self) -> Option<(T, T)> {
        self.event.map(|v| match v {
            Event::Site(site) => (site.x, site.y),
            Event::Circle(_, Circle { x, y, r }) => (x, y - r),
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum StepType<T: B> {
    Init,
    InitQ,
    PopQ,
    HandleSiteEvent,
    SearchT(SQKey),
    DeleteVertexEvent(Option<Event<T>>),
    ReplaceT,
    ReplaceLeft,
    ReplaceRight,
    CreateHalfEdges { e1: Site<T>, e2: Site<T> },
    CreateVertexEvent(SQKey, Option<Event<T>>),
    HandleCircleEvent,
    Delete,
    DeleteVertexEvent2(SQKey, Option<Event<T>>),
    AddVertexToEdges(usize),
    CreateVertexCircle(SQKey, Option<Event<T>>),

    Bound,
    FullInfinite(usize, usize, Site<T>, Site<T>, usize, usize),
    HalfInfinite(usize, usize, Site<T>, Site<T>, Site<T>, usize, usize),
    Stopped,
}

impl<T: B> StepType<T> {
    pub const fn is_site_event(&self) -> bool {
        matches!(
            self,
            Self::HandleSiteEvent
                | Self::SearchT(..)
                | Self::DeleteVertexEvent(..)
                | Self::ReplaceT
                | Self::ReplaceLeft
                | Self::ReplaceRight
                | Self::CreateHalfEdges { .. }
                | Self::CreateVertexEvent(..)
        )
    }
    pub const fn is_circle_event(&self) -> bool {
        matches!(
            self,
            Self::HandleCircleEvent
                | Self::Delete
                | Self::DeleteVertexEvent2(..)
                | Self::AddVertexToEdges(..)
                | Self::CreateVertexCircle { .. }
        )
    }

    pub const fn is_bound(&self) -> bool {
        matches!(
            self,
            Self::Bound | Self::FullInfinite(..) | Self::HalfInfinite(..)
        )
    }

    pub const fn is_replacing(&self) -> bool {
        matches!(
            self,
            Self::ReplaceLeft | Self::ReplaceRight | Self::ReplaceT
        )
    }
}
