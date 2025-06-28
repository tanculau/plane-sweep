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

    let mut last_event = None;
    // while Q is not empty. do Determine the next event point p in Q and delete it.
    while let Some(event) = event_queue.pop() {
        steps.push(
            Step::builder(StepType::PopQ, step_count(s))
                .event_queue(event_queue.clone())
                .status_queue(status_queue.iter())
                .event(event.clone())
                .build(),
        );
        handle_event_point(
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
#[allow(clippy::too_many_arguments, reason = "because capturing status cost a lot")]
fn handle_event_point(
    event: &Event,
    last_event : Option<&Event>,
    event_queue: &mut EventQueue,
    segments: &Segments,
    intersections: &mut Intersections,
    status_queue: &mut StatusQueue,
    s: &mut usize,
    steps: &mut AlgoSteps<Step>,
) {
    let p: CartesianCoord = (event.x, event.y).into();

    steps.push(
        Step::builder(StepType::HEPUpdateT, step_count(s))
            .event_queue(event_queue.clone())
            .status_queue(status_queue.iter())
            .event(event.clone())
            .build(),
    );

    // "Let U(p) be the set of segments whose upper endpoint is p; these segments
    // are stored with the event point p. (For horizontal segments, the upper
    // endpoint is by definition the left endpoint.)" [1, p. 26]
    let u_p = &event.segments;

    let (l_p, c_p): (Vec<_>, Vec<_>) = status_queue
        .iter_contains(segments, event.coord())
        .partition(|v| approx_eq!(CartesianCoord, segments[*v].lower, p));

    steps.push(
        Step::builder(StepType::CalculateSets, step_count(s))
            .event_queue(event_queue.clone())
            .status_queue(status_queue.iter())
            .event(event.clone())
            .c_p(c_p.clone())
            .u_p(event.segments.clone())
            .l_p(l_p.clone())
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
        .status_queue(status_queue.iter())
        .event(event.clone())
        .c_p(c_p.clone())
        .u_p(event.segments.clone())
        .l_p(l_p.clone())
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
                .status_queue(status_queue.iter())
                .event(event.clone())
                .c_p(c_p.clone())
                .u_p(event.segments.clone())
                .l_p(l_p.clone())
                .intersection(intersections.len().into())
                .build(),
        );
        // "[...] then Report p as an intersection, together with L(p), U(p), and C(p)." [1, p. 26]
        intersections.push(intersect);
    }

    // "Delete the segments in L(p) ∪ C(p) from T." [1, p. 26]
    // We only retain elements which are *not* in l_p
    // We do not do u_p, because how the status is defined and calculated, it is not needed
    for s in chain!(&l_p, &c_p) {
        dbg!((&status_queue, segments[*s], event.coord()));
        status_queue.delete(*s, segments, last_event.map_or(event.coord(), Event::coord));
        dbg!(&status_queue);
    }
    steps.push(
        Step::builder(StepType::DeleteLp, step_count(s))
            .event_queue(event_queue.clone())
            .status_queue(status_queue.iter())
            .event(event.clone())
            .c_p(c_p.clone())
            .u_p(event.segments.clone())
            .l_p(l_p.clone())
            .build(),
    );

    // "Insert the segments in U(p) ∪ C(p) into T." [1, p. 26]
    // C(p) is already in the status_queue, so we do not need this. Only U(p) gets inserted.
    for s in chain!(u_p, &c_p) {
        dbg!((&status_queue, segments[*s], event.coord()));
        status_queue.insert(*s, segments, event.coord());
        dbg!(&status_queue);
    }
    
    steps.push(
        Step::builder(StepType::InsertUp, step_count(s))
            .event_queue(event_queue.clone())
            .status_queue(status_queue.iter())
            .event(event.clone())
            .c_p(c_p.clone())
            .u_p(event.segments.clone())
            .l_p(l_p.clone())
            .build(),
    );


    // "if U(p) ∪ C(p) = ∅" [1, p. 26]
    if chain!(u_p, &c_p).next().is_none() {
        // "then Let sl and sr be the left and right neighbors of p in T." [1, p. 26]
        let l_r = status_queue.left_of_event(segments, event.coord());
        let u_r = status_queue.right_of_event(segments, event.coord());
        steps.push(
            Step::builder(StepType::UpCpEmpty { s_l: l_r, s_r: u_r }, step_count(s))
                .event_queue(event_queue.clone())
                .status_queue(status_queue.iter())
                .event(event.clone())
                .c_p(c_p.clone())
                .u_p(event.segments.clone())
                .l_p(l_p.clone())
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
                &c_p,
                &l_p,
            );
        }
    } else {
        let s_dash = status_queue.left_most(segments, event.coord()).unwrap();
        let s_l = status_queue.left_of_event(segments, event.coord());
        let s_dash_dash = status_queue.right_most(segments, event.coord()).unwrap();
        let s_r = status_queue.right_of_event(segments, event.coord());
        steps.push(
            Step::builder(
                StepType::UpCpNotEmpty {
                    s_dash,
                    s_dash_dash,
                    s_l,
                    s_r,
                },
                step_count(s),
            )
            .event_queue(event_queue.clone())
            .status_queue(status_queue.iter())
            .event(event.clone())
            .c_p(c_p.clone())
            .u_p(event.segments.clone())
            .l_p(l_p.clone())
            .build(),
        );

        if let (Some(left), right) = (s_l, s_dash) {
            find_new_event(
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
            find_new_event(
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
fn find_new_event(
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
    steps.push(
        Step::builder(StepType::FindNewEvent { s_l, s_r }, step_count(s))
            .event_queue(event_queue.clone())
            .status_queue(status_queue.iter())
            .event(event.clone())
            .c_p(c_p.to_vec())
            .u_p(event.segments.clone())
            .l_p(l_p.to_vec())
            .build(),
    );

    if let Some(intersection) = Segment::intersect([s_l, s_r], segments, 0)
        && intersection.typ().is_point()
        && (intersection.point1().y < *event.y
            || f_eq!(intersection.point1().y, *event.y) && intersection.point1().x > *event.x)
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
            .status_queue(status_queue.iter())
            .event(event.clone())
            .c_p(c_p.to_vec())
            .u_p(event.segments.clone())
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
        calculate_steps(&segments, &mut intersections, &mut steps);
    }

}
