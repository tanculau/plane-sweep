//! Plane Sweep Algorithm
//! Based on the book "Computational Geometry" from Mark Berg , Otfried Cheong , Marc Kreveld , Mark Overmars. [DOI](https://doi.org/10.1007/978-3-662-04245-8)
pub mod event;
//pub mod status_old;
pub mod status;
pub mod step;
pub mod ui;

use common::{
    AlgoSteps, f_eq,
    intersection::{Intersection, Intersections},
    math::{cartesian::CartesianCoord, float_cmp::approx_eq},
    segment::{Segment, SegmentIdx, Segments},
};
use itertools::chain;

use crate::{
    event::{Event, EventQueue},
    status::StatusQueue,
    step::{Step, StepType},
};

macro_rules! report {
    ($report : expr, $steps: expr, $step_type : expr,  $step : expr) => {
        if $report {
            $steps.push(Step::builder($step_type, step_count($step)).build());
        }
    };
    ($report : expr,$steps: expr, $step_type : expr,  $step : expr, $event_queue : expr) => {
        if $report {
            $steps.push(
                Step::builder($step_type, step_count($step))
                    .event_queue($event_queue.clone())
                    .build(),
            );
        }
    };
    ($report : expr,$steps: expr, $step_type : expr,  $step : expr, $event_queue : expr, $status_queue : expr) => {
        if $report {
            steps.push(
                Step::builder($step_type, step_count($step))
                    .event_queue($event_queue.clone())
                    .status_queue($status_queue.iter())
                    .event($event.clone())
                    .build(),
            );
        }
    };
    ($report : expr,$steps: expr, $step_type : expr,  $step : expr, $event_queue : expr, $status_queue : expr, $event : expr) => {
        if $report {
            $steps.push(
                Step::builder($step_type, step_count($step))
                    .event_queue($event_queue.clone())
                    .status_queue($status_queue.iter())
                    .event($event.clone())
                    .build(),
            );
        }
    };
    ($report : expr,$steps: expr, $step_type : expr,  $step : expr, $event_queue : expr, $status_queue : expr, $event : expr, $c_p : expr, $l_p : expr) => {
        if $report {
            $steps.push(
                Step::builder($step_type, step_count($step))
                    .event_queue($event_queue.clone())
                    .status_queue($status_queue.iter())
                    .event($event.clone())
                    .c_p($c_p.clone())
                    .u_p($event.segments.clone())
                    .l_p($l_p.clone())
                    .build(),
            );
        }
    };
    ($report : expr,$steps: expr, $step_type : expr, DONT_CHANGE $step : expr, $event_queue : expr, $status_queue : expr, $event : expr, $c_p : expr, $l_p : expr) => {
        if $report {
            $steps.push(
                Step::builder($step_type, ($step))
                    .event_queue($event_queue.clone())
                    .status_queue($status_queue.iter())
                    .event($event.clone())
                    .c_p($c_p.clone())
                    .u_p($event.segments.clone())
                    .l_p($l_p.clone())
                    .build(),
            );
        }
    };
}

const fn step_count(step: &mut usize) -> usize {
    let out = *step;
    *step += 1;
    out
}

pub fn calculate_steps<const REPORT : bool>(
    segments: &Segments,
    intersections: &mut Intersections,
    steps: &mut AlgoSteps<Step>,
) {
    let mut sc = 0;
    let s = &mut sc;
    steps.clear();
    intersections.clear();
    report!(REPORT,steps, StepType::Init, s);

    // Initialize an empty event queue Q.
    report!(REPORT,steps, StepType::StartInitQ, s);
    let mut event_queue = EventQueue::new();
    // Next, insert the segment endpoints into Q; when an upper endpoint is inserted, the corresponding segment should be stored with it.
    for (id, segment) in segments.iter_enumerated() {
        // We store the segment id for the upper one
        let event = Event::new(segment.upper.y, segment.upper.x, std::iter::once(id));
        event_queue.insert(event);
        // We do not store the segment id for the lower one
        let event = Event::new(segment.lower.y, segment.lower.x, std::iter::empty());
        event_queue.insert(event);
        report!(REPORT,steps, StepType::InitQ { segment: id }, s, event_queue);
    }

    // Initialize an empty status structure T.
    let mut status_queue = StatusQueue::new();
    report!(REPORT,steps, StepType::InitT, s, event_queue);

    let mut last_event = None;
    // while Q is not empty. do Determine the next event point p in Q and delete it.
    while let Some(event) = event_queue.pop() {
        report!(REPORT,steps, StepType::PopQ, s, event_queue, status_queue, event);
        handle_event_point::<REPORT>(
            &event,
            last_event.as_ref(),
            &mut event_queue,
            segments,
            intersections,
            &mut status_queue,
            s,
            steps,
        );
        last_event = Some(event);
        // HANDLE EVENT POINT(p)
    }

    steps.push(Step::builder(StepType::End, step_count(s)).build());
}

#[allow(clippy::too_many_lines, reason = "because capturing status cost a lot")]
#[allow(
    clippy::too_many_arguments,
    reason = "because capturing status cost a lot"
)]
fn handle_event_point<const REPORT : bool>(
    event: &Event,
    last_event: Option<&Event>,
    event_queue: &mut EventQueue,
    segments: &Segments,
    intersections: &mut Intersections,
    status_queue: &mut StatusQueue,
    s: &mut usize,
    steps: &mut AlgoSteps<Step>,
) {
    let p: CartesianCoord = (event.x, event.y).into();
    report!(REPORT,
        steps,
        StepType::HEPUpdateT,
        s,
        event_queue,
        status_queue,
        event
    );

    // "Let U(p) be the set of segments whose upper endpoint is p; these segments
    // are stored with the event point p. (For horizontal segments, the upper
    // endpoint is by definition the left endpoint.)" [1, p. 26]
    let u_p = &event.segments;

    let (l_p, c_p): (Vec<_>, Vec<_>) = status_queue
        .iter_contains(segments, event.coord())
        .partition(|v| approx_eq!(CartesianCoord, segments[*v].lower, p));

    report!(REPORT,
        steps,
        StepType::CalculateSets,
        s,
        event_queue,
        status_queue,
        event,
        c_p,
        l_p
    );

    // "if L(p) ∪ U(p) ∪ C(p) contains more than one segment [...]" [1, p. 26]
    let l_p_and_u_p_and_c_p: Vec<_> = event
        .segments
        .iter()
        .chain(l_p.iter())
        .chain(c_p.iter())
        .copied()
        .collect();

    report!(REPORT,
        steps,
        StepType::CalculateUpCpLp {
            up_cp_lp: l_p_and_u_p_and_c_p.clone(),
        },
        s,
        event_queue,
        status_queue,
        event,
        c_p,
        l_p
    );

    if l_p_and_u_p_and_c_p.len() > 1 {
        let step = step_count(s);
        let intersect = Intersection::new(
            common::intersection::IntersectionType::Point { coord: p },
            l_p_and_u_p_and_c_p,
            step,
        );
        report!(REPORT,
            steps,
            StepType::ReportIntersections,
            DONT_CHANGE step,
            event_queue,
            status_queue,
            event,
            c_p,
            l_p
        );
        // "[...] then Report p as an intersection, together with L(p), U(p), and C(p)." [1, p. 26]
        intersections.push(intersect);
    }

    // "Delete the segments in L(p) ∪ C(p) from T." [1, p. 26]
    // We only retain elements which are *not* in l_p
    // We do not do u_p, because how the status is defined and calculated, it is not needed
    for s in chain!(&l_p, &c_p) {
        status_queue.delete(*s, segments, last_event.map_or(event.coord(), Event::coord));
    }
    report!(REPORT,
        steps,
        StepType::DeleteLp,
        s,
        event_queue,
        status_queue,
        event,
        c_p,
        l_p
    );
    // "Insert the segments in U(p) ∪ C(p) into T." [1, p. 26]
    // C(p) is already in the status_queue, so we do not need this. Only U(p) gets inserted.
    for s in chain!(u_p, &c_p) {
        status_queue.insert(*s, segments, event.coord());
    }

    report!(REPORT,
        steps,
        StepType::InsertUp,
        s,
        event_queue,
        status_queue,
        event,
        c_p,
        l_p
    );

    // "if U(p) ∪ C(p) = ∅" [1, p. 26]
    if chain!(u_p, &c_p).next().is_none() {
        // "then Let sl and sr be the left and right neighbors of p in T." [1, p. 26]
        let l_r = status_queue.left_of_event(segments, event.coord());
        let u_r = status_queue.right_of_event(segments, event.coord());
        report!(REPORT,
            steps,
            StepType::UpCpEmpty { s_l: l_r, s_r: u_r },
            s,
            event_queue,
            status_queue,
            event,
            c_p,
            l_p
        );

        if let (Some(left), Some(right)) = (l_r, u_r) {
            find_new_event::<REPORT>(
                left,
                right,
                event,
                segments,
                s,
                event_queue,
                status_queue,
                steps,
                &c_p,
                &l_p,
            );
        }
    } else {
        let s_dash = status_queue.left_most(segments, event.coord()).unwrap();
        let s_l = status_queue.left_of_event(segments, event.coord());
        let s_dash_dash = status_queue.right_most(segments, event.coord()).unwrap();
        let s_r = status_queue.right_of_event(segments, event.coord());
        report!(REPORT,
            steps,
            StepType::UpCpNotEmpty {
                s_dash,
                s_dash_dash,
                s_l,
                s_r,
            },
            s,
            event_queue,
            status_queue,
            event,
            c_p,
            l_p
        );
        if let (Some(left), right) = (s_l, s_dash) {
            find_new_event::<REPORT>(
                left,
                right,
                event,
                segments,
                s,
                event_queue,
                status_queue,
                steps,
                &c_p.clone(),
                &l_p.clone(),
            );
        }
        if let (left, Some(right)) = (s_dash_dash, s_r) {
            find_new_event::<REPORT>(
                left,
                right,
                event,
                segments,
                s,
                event_queue,
                status_queue,
                steps,
                &c_p.clone(),
                &l_p.clone(),
            );
        }
    }
}

#[allow(
    clippy::too_many_arguments,
    reason = "because capturing status cost a lot"
)]
fn find_new_event<const REPORT : bool>(
    s_l: SegmentIdx,
    s_r: SegmentIdx,
    event: &Event,
    segments: &Segments,
    s: &mut usize,
    event_queue: &mut EventQueue,
    status_queue: &StatusQueue,
    steps: &mut AlgoSteps<Step>,
    c_p: &[SegmentIdx],
    l_p: &[SegmentIdx],
) {
    report!(REPORT,
        steps,
        StepType::FindNewEvent { s_l, s_r },
        s,
        event_queue,
        status_queue,
        event,
        c_p.to_vec(),
        l_p.to_vec()
    );

    if let Some(intersection) = Segment::intersect([s_l, s_r], segments, 0)
        && intersection.typ().is_point()
        && (intersection.point1().y < *event.y
            || f_eq!(intersection.point1().y, *event.y) && intersection.point1().x > *event.x)
    {
        report!(REPORT,
            steps,
            StepType::InsertIntersectionEvent {
                s_l,
                s_r,
                intersection: (
                    intersection.point1().x.into(),
                    intersection.point1().y.into(),
                ),
            },
            s,
            event_queue,
            status_queue,
            event,
            c_p.to_vec(),
            l_p.to_vec()
        );
        event_queue.insert(Event::new(
            intersection.point1().y,
            intersection.point1().x,
            std::iter::empty(),
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let segments = [
            Segment::new((2, 2), (-2, -2)),
            Segment::new((-2, 2), (2, -2)),
            Segment::new((-1, 2), (-1, -2)),
        ]
        .into_iter()
        .collect();
        let mut intersections = Intersections::new();
        let mut steps = AlgoSteps::new();
        calculate_steps::<false>(&segments, &mut intersections, &mut steps);
    }
}
