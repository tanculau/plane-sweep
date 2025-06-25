//! Plane Sweep Algorithm
//! Based on the book "Computational Geometry" from Mark Berg , Otfried Cheong , Marc Kreveld , Mark Overmars. [DOI](https://doi.org/10.1007/978-3-662-04245-8)
#![allow(dead_code)]
pub mod ui;

use core::{borrow::Borrow, iter, ops::Bound};
use std::collections::{BTreeSet, HashSet};

use bon::Builder;
use common::{
    AlgoSteps, AlgrorithmStep, f_eq,
    intersection::{Intersection, IntersectionIdx, Intersections},
    math::{Float, cartesian::CartesianCoord, homogeneous::HomogeneousLine},
    segment::{Segment, SegmentIdx, Segments},
};

use common::math::OrderedFloat;

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
}

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
    queue: BTreeSet<Event>,
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

#[derive(Debug, Clone, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Status {
    pub x_intersect: OrderedFloat,
    pub segments: Vec<SegmentIdx>,
}
impl PartialEq for Status {
    fn eq(&self, other: &Self) -> bool {
        self.x_intersect.eq(&other.x_intersect)
    }
}

impl PartialOrd for Status {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Status {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.x_intersect.cmp(&other.x_intersect)
    }
}

impl Borrow<OrderedFloat> for Status {
    fn borrow(&self) -> &OrderedFloat {
        &self.x_intersect
    }
}

impl Status {
    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn new(p_y: Float, segment: Segment, segment_idx: SegmentIdx) -> Self {
        let x_intersect = if f_eq!(segment.upper.y, segment.lower.y) && f_eq!(segment.upper.y, p_y)
        {
            segment.lower.x
        } else {
            let horizontal = HomogeneousLine::horizontal(p_y);
            let seg = segment.line();
            horizontal.intersection(seg).cartesian().unwrap().x
        };

        let x_intersect = if x_intersect == -0.0 {
            0.0
        } else {
            x_intersect
        }
        .into();

        Self {
            x_intersect,
            segments: vec![segment_idx],
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StatusQueue {
    pub inner: BTreeSet<Status>,
}

impl StatusQueue {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            inner: BTreeSet::new(),
        }
    }

    pub fn update(&mut self, y: impl Into<Float>, segments: &Segments) {
        let y = y.into();
        let a = std::mem::take(self);
        *self = a
            .into_iter()
            .flat_map(|s| s.segments.into_iter())
            .map(|s| Status::new(y, segments[s], s))
            .collect();
    }

    pub fn iter(&self) -> std::collections::btree_set::Iter<'_, Status> {
        self.inner.iter()
    }

    pub fn remove(
        &mut self,
        elements: &HashSet<SegmentIdx>,
        y: impl Into<Float>,
        segments: &Segments,
    ) {
        let y = y.into();
        let a = std::mem::take(self);
        *self = a
            .into_iter()
            .flat_map(|s| s.segments.into_iter())
            .filter(|s| !elements.contains(s))
            .map(|s| Status::new(y, segments[s], s))
            .collect();
    }

    pub fn insert(&mut self, mut status: Status) {
        if let Some(already) = self.inner.take(&status.x_intersect) {
            status.segments.extend(&already.segments);
        }
        self.inner.insert(status);
    }

    pub fn left(&self, x: impl Into<OrderedFloat>) -> Option<&Status> {
        let x = &x.into();
        let mut range = self
            .inner
            .range::<OrderedFloat, _>((Bound::Unbounded, Bound::Excluded(x)));
        range.next_back()
    }

    pub fn right(&self, x: impl Into<OrderedFloat>) -> Option<&Status> {
        let x = &x.into();
        let mut range = self
            .inner
            .range::<OrderedFloat, _>((Bound::Excluded(x), Bound::Unbounded));
        range.next()
    }

    pub fn get(&self, x: impl Into<OrderedFloat>) -> Option<&Status> {
        self.inner.get::<OrderedFloat>(&x.into())
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl IntoIterator for StatusQueue {
    type Item = Status;

    type IntoIter = std::collections::btree_set::IntoIter<Status>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a> IntoIterator for &'a StatusQueue {
    type Item = &'a Status;

    type IntoIter = std::collections::btree_set::Iter<'a, Status>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl Extend<Status> for StatusQueue {
    fn extend<T: IntoIterator<Item = Status>>(&mut self, iter: T) {
        for v in iter {
            (self.insert(v));
        }
    }
}

impl FromIterator<Status> for StatusQueue {
    fn from_iter<T: IntoIterator<Item = Status>>(iter: T) -> Self {
        let mut ret = Self::new();
        ret.extend(iter);
        ret
    }
}

const fn step_count(step: &mut usize) -> usize {
    let out = *step;
    *step += 1;
    out
}

pub fn calculate_steps(
    segments: &Segments,
    intersections: &mut Intersections,
    steps: &mut AlgoSteps<Step>,
) {
    let mut sc = 0;
    let s = &mut sc;
    steps.clear();
    intersections.clear();
    steps.push(Step::builder(StepType::Init, step_count(s)).build());

    // Initialize an empty event queue Q.
    steps.push(Step::builder(StepType::StartInitQ, step_count(s)).build());
    let mut event_queue = EventQueue::new();
    // Next, insert the segment endpoints into Q; when an upper endpoint is inserted, the corresponding segment should be stored with it.
    for (id, segment) in segments.iter_enumerated() {
        // We store the segment id for the upper one
        let event = Event::new(segment.upper.y, segment.upper.x, std::iter::once(id));
        event_queue.insert(event);
        // We do not store the segment id for the lower one
        let event = Event::new(segment.lower.y, segment.lower.x, std::iter::empty());
        event_queue.insert(event);
        steps.push(
            Step::builder(StepType::InitQ { segment: id }, step_count(s))
                .event_queue(event_queue.clone())
                .build(),
        );
    }

    // Initialize an empty status structure T.
    let mut status_queue = StatusQueue::new();
    steps.push(
        Step::builder(StepType::InitT, step_count(s))
            .event_queue(event_queue.clone())
            .build(),
    );

    // while Q is not empty. do Determine the next event point p in Q and delete it.
    while let Some(event) = event_queue.pop() {
        steps.push(
            Step::builder(StepType::PopQ, step_count(s))
                .event_queue(event_queue.clone())
                .status_queue(status_queue.clone())
                .event(event.clone())
                .build(),
        );
        handle_event_point(
            &event,
            &mut event_queue,
            segments,
            intersections,
            &mut status_queue,
            s,
            steps,
        );
        // HANDLE EVENT POINT(p)
    }

    steps.push(Step::builder(StepType::End, step_count(s)).build());
}

#[allow(clippy::too_many_lines, reason = "because capturing status cost a lot")]
fn handle_event_point(
    event: &Event,
    event_queue: &mut EventQueue,
    segments: &Segments,
    intersections: &mut Intersections,
    status_queue: &mut StatusQueue,
    s: &mut usize,
    steps: &mut AlgoSteps<Step>,
) {
    let p: CartesianCoord = (event.x, event.y).into();

    status_queue.update(p.y, segments);
    steps.push(
        Step::builder(StepType::HEPUpdateT, step_count(s))
            .event_queue(event_queue.clone())
            .status_queue(status_queue.clone())
            .event(event.clone())
            .build(),
    );

    // "Let U(p) be the set of segments whose upper endpoint is p; these segments
    // are stored with the event point p. (For horizontal segments, the upper
    // endpoint is by definition the left endpoint.)" [1, p. 26]
    let u_p = event
        .segments
        .iter()
        .map(|&idx| Status::new(p.y, segments[idx], idx))
        .collect::<StatusQueue>();

    // "Find all segments stored in T that contain p; they are adjacent in T." [1, p. 26]
    let found_in_status = status_queue
        .iter()
        .filter(|s| f_eq!(*s.x_intersect.0, p.x))
        .flat_map(|s| s.segments.iter())
        .copied()
        .collect::<Vec<_>>(); // Map to Segment

    // "Let L(p) denote the subset of segments found whose lower endpoint is p [...]" [1, p. 26]
    let l_p = found_in_status
        .iter()
        .filter(|&&status| f_eq!(segments[status].lower.y, p.y))
        .copied()
        .collect::<HashSet<_>>();
    // "[...] let C(p) denote the subset of segments found that contain p in their interior." [1, p. 26]
    let c_p = found_in_status
        .iter()
        .filter(|status| !l_p.contains(status)) // Not Lower
        .filter(|status| !event.segments.contains(status)) // Not Upper
        .filter(|&&status| segments[status].contains(p))
        .copied()
        .collect::<HashSet<_>>();

    steps.push(
        Step::builder(StepType::CalculateSets, step_count(s))
            .event_queue(event_queue.clone())
            .status_queue(status_queue.clone())
            .event(event.clone())
            .c_p(c_p.iter().copied().collect())
            .u_p(event.segments.iter().copied().collect())
            .l_p(l_p.iter().copied().collect())
            .build(),
    );

    // "if L(p) ∪ U(p) ∪ C(p) contains more than one segment [...]" [1, p. 26]
    let l_p_and_u_p_and_c_p: Vec<_> = event
        .segments
        .iter()
        .chain(l_p.iter())
        .chain(c_p.iter())
        .copied()
        .collect();

    steps.push(
        Step::builder(
            StepType::CalculateUpCpLp {
                up_cp_lp: l_p_and_u_p_and_c_p.clone(),
            },
            step_count(s),
        )
        .event_queue(event_queue.clone())
        .status_queue(status_queue.clone())
        .event(event.clone())
        .c_p(c_p.iter().copied().collect())
        .u_p(event.segments.iter().copied().collect())
        .l_p(l_p.iter().copied().collect())
        .build(),
    );

    if l_p_and_u_p_and_c_p.len() > 1 {
        let step = step_count(s);
        let intersect = Intersection::new(
            common::intersection::IntersectionType::Point { coord: p },
            l_p_and_u_p_and_c_p,
            step,
        );
        steps.push(
            Step::builder(StepType::ReportIntersections, step)
                .event_queue(event_queue.clone())
                .status_queue(status_queue.clone())
                .event(event.clone())
                .c_p(c_p.iter().copied().collect())
                .u_p(event.segments.iter().copied().collect())
                .l_p(l_p.iter().copied().collect())
                .intersection(intersections.len().into())
                .build(),
        );
        // "[...] then Report p as an intersection, together with L(p), U(p), and C(p)." [1, p. 26]
        intersections.push(intersect);
    }

    // "Delete the segments in L(p) ∪ C(p) from T." [1, p. 26]
    // We only retain elements which are *not* in l_p
    // We do not do u_p, because how the status is defined and calculated, it is not needed
    status_queue.remove(&l_p, *event.y, segments);

    steps.push(
        Step::builder(StepType::DeleteLp, step_count(s))
            .event_queue(event_queue.clone())
            .status_queue(status_queue.clone())
            .event(event.clone())
            .c_p(c_p.iter().copied().collect())
            .u_p(event.segments.iter().copied().collect())
            .l_p(l_p.iter().copied().collect())
            .build(),
    );

    // "Insert the segments in U(p) ∪ C(p) into T." [1, p. 26]
    // C(p) is already in the status_queue, so we do not need this. Only U(p) gets inserted.
    status_queue.extend(u_p);

    steps.push(
        Step::builder(StepType::InsertUp, step_count(s))
            .event_queue(event_queue.clone())
            .status_queue(status_queue.clone())
            .event(event.clone())
            .c_p(c_p.iter().copied().collect())
            .u_p(event.segments.iter().copied().collect())
            .l_p(l_p.iter().copied().collect())
            .build(),
    );

    // "if U(p) ∪ C(p) = ∅" [1, p. 26]
    if status_queue.is_empty() {
        // "then Let sl and sr be the left and right neighbors of p in T." [1, p. 26]
        let l_r = status_queue.left(event.x);
        let u_r = status_queue.right(event.x);
        steps.push(
            Step::builder(
                StepType::UpCpEmpty {
                    s_l: l_r.map_or(Vec::new(), |s| s.segments.clone()),
                    s_r: u_r.map_or(Vec::new(), |s| s.segments.clone()),
                },
                step_count(s),
            )
            .event_queue(event_queue.clone())
            .status_queue(status_queue.clone())
            .event(event.clone())
            .c_p(c_p.iter().copied().collect())
            .u_p(event.segments.iter().copied().collect())
            .l_p(l_p.iter().copied().collect())
            .build(),
        );

        if let (Some(left), Some(right)) = (l_r, u_r) {
            find_new_event(
                left,
                right,
                event,
                segments,
                s,
                event_queue,
                status_queue,
                steps,
                &c_p.iter().copied().collect::<Vec<_>>(),
                &l_p.iter().copied().collect::<Vec<_>>(),
            );
        }
    } else {
        let s_dash = status_queue.get(event.x);
        let s_l = status_queue.left(event.x);
        let s_dash_dash = status_queue.get(event.x);
        let s_r = status_queue.right(event.x);
        steps.push(
            Step::builder(
                StepType::UpCpNotEmpty {
                    s_dash: s_dash.and_then(|v| v.segments.first().copied()),
                    s_dash_dash: s_dash_dash.and_then(|v| v.segments.first().copied()),
                    s_l: s_l.map_or(Vec::new(), |s| s.segments.clone()),
                    s_r: s_r.map_or(Vec::new(), |s| s.segments.clone()),
                },
                step_count(s),
            )
            .event_queue(event_queue.clone())
            .status_queue(status_queue.clone())
            .event(event.clone())
            .c_p(c_p.iter().copied().collect())
            .u_p(event.segments.iter().copied().collect())
            .l_p(l_p.iter().copied().collect())
            .build(),
        );

        if let (Some(left), Some(right)) = (s_l, s_dash) {
            find_new_event(
                left,
                right,
                event,
                segments,
                s,
                event_queue,
                status_queue,
                steps,
                &c_p.iter().copied().collect::<Vec<_>>(),
                &l_p.iter().copied().collect::<Vec<_>>(),
            );
        }
        if let (Some(left), Some(right)) = (s_dash_dash, s_r) {
            find_new_event(
                left,
                right,
                event,
                segments,
                s,
                event_queue,
                status_queue,
                steps,
                &c_p.iter().copied().collect::<Vec<_>>(),
                &l_p.iter().copied().collect::<Vec<_>>(),
            );
        }
    }
}

#[allow(
    clippy::too_many_arguments,
    reason = "because capturing status cost a lot"
)]
fn find_new_event(
    left: &Status,
    right: &Status,
    event: &Event,
    segments: &Segments,
    s: &mut usize,
    event_queue: &mut EventQueue,
    status_queue: &StatusQueue,
    steps: &mut AlgoSteps<Step>,
    c_p: &[SegmentIdx],
    l_p: &[SegmentIdx],
) {
    for &s_l in &left.segments {
        for &s_r in &right.segments {
            steps.push(
                Step::builder(StepType::FindNewEvent { s_l, s_r }, step_count(s))
                    .event_queue(event_queue.clone())
                    .status_queue(status_queue.clone())
                    .event(event.clone())
                    .c_p(c_p.to_vec())
                    .u_p(event.segments.iter().copied().collect())
                    .l_p(l_p.to_vec())
                    .build(),
            );

            if let Some(intersection) = Segment::intersect([s_l, s_r], segments, 0)
                && intersection.typ().is_point()
                && (intersection.point1().y < *event.y
                    || f_eq!(intersection.point1().y, *event.y)
                        && intersection.point1().x > *event.x)
            {
                steps.push(
                    Step::builder(
                        StepType::InsertIntersectionEvent {
                            s_l,
                            s_r,
                            intersection: (
                                intersection.point1().x.into(),
                                intersection.point1().y.into(),
                            ),
                        },
                        step_count(s),
                    )
                    .event_queue(event_queue.clone())
                    .status_queue(status_queue.clone())
                    .event(event.clone())
                    .c_p(c_p.to_vec())
                    .u_p(event.segments.iter().copied().collect())
                    .l_p(l_p.to_vec())
                    .build(),
                );
                event_queue.insert(Event::new(
                    intersection.point1().y,
                    intersection.point1().x,
                    std::iter::empty(),
                ));
            }
        }
    }
}
