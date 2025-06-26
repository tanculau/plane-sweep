//! Plane Sweep Algorithm
//! Based on the book "Computational Geometry" from Mark Berg , Otfried Cheong , Marc Kreveld , Mark Overmars. [DOI](https://doi.org/10.1007/978-3-662-04245-8)
pub mod event;
pub mod status;
pub mod step;
pub mod ui;

use common::{
    AlgoSteps, f_eq,
    intersection::{Intersection, Intersections},
    math::{cartesian::CartesianCoord, float_cmp::approx_eq},
    segment::{Segment, SegmentIdx, Segments},
};

use crate::{
    event::{Event, EventQueue},
    status::{Status, StatusQueue},
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

    status_queue.update(p.y, p.x, segments);
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
        .map(|&idx| Status::new(p.y, p.x, segments[idx], idx))
        .collect::<StatusQueue>();

    let (l_p, c_p): (Vec<_>, Vec<_>) = status_queue
        .iter()
        .filter(|s| f_eq!(*s.x_intersect.0, p.x))
        .flat_map(|s| s.segments.iter())
        .copied()
        .partition(|v| approx_eq!(CartesianCoord, segments[*v].lower, p));

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
    status_queue.remove(&l_p.iter().copied().collect(), *event.y, *event.x, segments);

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
