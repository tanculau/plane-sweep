//! Plane Sweep Algorithm
//! Based on the book "Computational Geometry" from Mark Berg , Otfried Cheong , Marc Kreveld , Mark Overmars. [DOI](https://doi.org/10.1007/978-3-662-04245-8)
pub mod event;
//pub mod status_old;
pub mod status;
pub mod step;
#[cfg(feature = "ui")]
pub mod ui;

use common::{
    AlgoSteps,
    intersection::{InterVec, Intersection, Intersections},
    math::cartesian::CartesianCoord,
    segment::{Segment, SegmentIdx, Segments},
};
use itertools::{Itertools, chain};
use smallvec::SmallVec;

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
    ($report : expr,$steps: expr, $step_type : expr, $step : expr, $event_queue : expr, $status_queue : expr, $event : expr, $c_p : expr, $l_p : expr, $inter : expr) => {
        if $report {
            $steps.push(
                Step::builder($step_type, ($step))
                    .event_queue($event_queue.clone())
                    .status_queue($status_queue.iter())
                    .event($event.clone())
                    .c_p($c_p.clone())
                    .u_p($event.segments.clone())
                    .l_p($l_p.clone())
                    .intersection($inter.clone())
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

pub fn calculate_steps<const REPORT: bool>(
    segments: &Segments,
    intersections: &mut Intersections,
    steps: &mut AlgoSteps<Step>,
) {
    let mut sc = 0;
    let s = &mut sc;
    steps.clear();
    intersections.clear();
    report!(REPORT, steps, StepType::Init, s);

    // Initialize an empty event queue Q.
    report!(REPORT, steps, StepType::StartInitQ, s);
    let mut event_queue = EventQueue::new();
    // Next, insert the segment endpoints into Q; when an upper endpoint is inserted, the corresponding segment should be stored with it.
    for (id, segment) in segments.iter_enumerated() {
        // We store the segment id for the upper one
        let event = Event::new(segment.upper.y, segment.upper.x, std::iter::once(id));
        event_queue.insert(event);
        // We do not store the segment id for the lower one
        let event = Event::new(segment.lower.y, segment.lower.x, std::iter::empty());
        event_queue.insert(event);
        report!(
            REPORT,
            steps,
            StepType::InitQ { segment: id },
            s,
            event_queue
        );
    }
    //println!("{event_queue:?}");

    // Initialize an empty status structure T.
    let mut status_queue = StatusQueue::new();
    report!(REPORT, steps, StepType::InitT, s, event_queue);

    let mut last_event: Option<Event> = None;
    // while Q is not empty. do Determine the next event point p in Q and delete it.
    while let Some(event) = event_queue.pop() {
        report!(
            REPORT,
            steps,
            StepType::PopQ,
            s,
            event_queue,
            status_queue,
            event
        );
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
fn handle_event_point<const REPORT: bool>(
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

    // "Let U(p) be the set of segments whose upper endpoint is p; these segments
    // are stored with the event point p. (For horizontal segments, the upper
    // endpoint is by definition the left endpoint.)" [1, p. 26]
    let u_p = &event.segments;

    let (l_p, c_p): (Vec<_>, Vec<_>) = status_queue
        .iter_contains(segments, &p)
        .partition(|v| segments[*v].clone().lower == p);
    //println!("Event {:?}: L_P: {l_p:?}", event.coord());
    //println!("{status_queue:?}");
    report!(
        REPORT,
        steps,
        StepType::CalculateSets,
        s,
        event_queue,
        status_queue,
        event,
        c_p,
        l_p
    );

    //println!("{s}. LP: {l_p:?} , CP: {c_p:?} , UP: {u_p:?} at {p:?}");
    // "if L(p) ∪ U(p) ∪ C(p) contains more than one segment [...]" [1, p. 26]
    let l_p_and_u_p_and_c_p: InterVec = event
        .segments
        .iter()
        .chain(l_p.iter())
        .chain(c_p.iter())
        .copied()
        .collect();

    report!(
        REPORT,
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
            common::intersection::IntersectionType::Point { coord: p.clone() },
            l_p_and_u_p_and_c_p,
            step,
        );
        let intersect = intersections.push_and_get_key(intersect);
        report!(
            REPORT,
            steps,
            StepType::ReportIntersections,
            step,
            event_queue,
            status_queue,
            event,
            c_p,
            l_p,
            intersect
        );
        // "[...] then Report p as an intersection, together with L(p), U(p), and C(p)." [1, p. 26]
    }

    // "Delete the segments in L(p) ∪ C(p) from T." [1, p. 26]
    // We only retain elements which are *not* in l_p
    // We do not do u_p, because how the status is defined and calculated, it is not needed
    for s in chain!(&l_p, &c_p) {
        status_queue.delete(*s, segments, &p);
        debug_assert!(
            !status_queue.iter().contains(s),
            "Tried to remove {s:?} from {status_queue:?} with last event {last_event:?} and even {event:?}"
        );
    }
    //println!(
    //    "{}",
    //    status_queue.iter().fold(String::new(), |mut acc, v| {
    //        use std::fmt::Write;
    //        write!(&mut acc, ", {:?}:  {:?}", v, intersection(segments[v], last_event.map_or(event.coord(), Event::coord))).unwrap();
    //        acc
    //    })
    //);

    report!(
        REPORT,
        steps,
        StepType::DeleteLpCp,
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
        status_queue.insert(*s, segments, &p);
    }

    //println!(
    //    "{}",
    //    status_queue
    //        .iter()
    //        .fold(String::from("Order: "), |mut acc, v| {
    //            use std::fmt::Write;
    //            write!(
    //                &mut acc,
    //                ", {:?}:  {:?}, {:?}",
    //                v,
    //                intersection(segments[v].clone(), event.coord()),
    //                segments[v].slope()
    //            )
    //            .unwrap();
    //            acc
    //        })
    //);

    report!(
        REPORT,
        steps,
        StepType::InsertUpCp,
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
        let l_r = status_queue.left_of_event(segments, &p);
        let u_r = status_queue.right_of_event(segments, &p);
        report!(
            REPORT,
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
        let s_dash = status_queue.left_most(segments, &p);
        let s_l = status_queue.left_of_event(segments, &p);
        let s_dash_dash = status_queue.right_most(segments, &p);
        let s_r = status_queue.right_of_event(segments, &p);
        //println!("s_dash {s_dash:?}, s_l {s_l:?}, s_dash_dash {s_dash_dash:?}, s_r {s_r:?}");
        report!(
            REPORT,
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
        if let (Some(left), Some(right)) = (s_l, s_dash) {
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
        if let (Some(left), Some(right)) = (s_dash_dash, s_r) {
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
fn find_new_event<const REPORT: bool>(
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
    report!(
        REPORT,
        steps,
        StepType::FindNewEvent { s_l, s_r },
        s,
        event_queue,
        status_queue,
        event,
        c_p.to_vec(),
        l_p.to_vec()
    );

    if let Some(intersection) = Segment::intersect(s_l, s_r, segments, 0)
        && intersection.typ().is_point()
        && (intersection.point1().y < event.y
            || intersection.point1().y == event.y && intersection.point1().x > event.x)
    {
        report!(
            REPORT,
            steps,
            StepType::InsertIntersectionEvent {
                s_l,
                s_r,
                intersection: (intersection.point1().x, intersection.point1().y,),
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
        // println!(
        //     "Found intersection between {s_l:?} and {s_r:?}:{:?}",
        //     intersection.point1()
        // );
    }
}

pub fn calculate(segments: &Segments, intersections: &mut Intersections) {
    intersections.clear();

    // Initialize an empty event queue Q.
    let mut event_queue = EventQueue::new();
    // Next, insert the segment endpoints into Q; when an upper endpoint is inserted, the corresponding segment should be stored with it.
    for (id, segment) in segments.iter_enumerated() {
        // We store the segment id for the upper one
        let event = Event::new(segment.upper.y, segment.upper.x, std::iter::once(id));
        event_queue.insert(event);
        // We do not store the segment id for the lower one
        let event = Event::new(segment.lower.y, segment.lower.x, std::iter::empty());
        event_queue.insert(event);
    }

    // Initialize an empty status structure T.
    let mut status_queue = StatusQueue::new();

    // while Q is not empty. do Determine the next event point p in Q and delete it.
    while let Some(event) = event_queue.pop() {
        handle_event_point_fast(
            &event,
            &mut event_queue,
            segments,
            intersections,
            &mut status_queue,
        );
        // HANDLE EVENT POINT(p)
    }
}

#[allow(clippy::too_many_lines, reason = "because capturing status cost a lot")]
#[allow(
    clippy::too_many_arguments,
    reason = "because capturing status cost a lot"
)]
fn handle_event_point_fast(
    event: &Event,
    event_queue: &mut EventQueue,
    segments: &Segments,
    intersections: &mut Intersections,
    status_queue: &mut StatusQueue,
) {
    let p: CartesianCoord = (event.x, event.y).into();

    // "Let U(p) be the set of segments whose upper endpoint is p; these segments
    // are stored with the event point p. (For horizontal segments, the upper
    // endpoint is by definition the left endpoint.)" [1, p. 26]
    let u_p = &event.segments;

    let (l_p, c_p): (SmallVec<_, 10>, SmallVec<_, 10>) = status_queue
        .iter_contains(segments, &p)
        .partition(|v| segments[*v].clone().lower == p);

    if u_p.len() + c_p.len() + l_p.len() > 1 {
        let intersect = Intersection::new(
            common::intersection::IntersectionType::Point { coord: p.clone() },
            c_p.iter().chain(l_p.iter()).chain(u_p).copied().collect(),
            0,
        );
        intersections.push(intersect);
        // "[...] then Report p as an intersection, together with L(p), U(p), and C(p)." [1, p. 26]
    }

    // "Delete the segments in L(p) ∪ C(p) from T." [1, p. 26]
    // We only retain elements which are *not* in l_p
    // We do not do u_p, because how the status is defined and calculated, it is not needed
    for s in chain!(&l_p, &c_p) {
        status_queue.delete(*s, segments, &p);
        debug_assert!(
            !status_queue.iter().contains(s),
            "Tried to remove {s:?} from {status_queue:?} with last event {event:?}"
        );
    }

    // "Insert the segments in U(p) ∪ C(p) into T." [1, p. 26]
    // C(p) is already in the status_queue, so we do not need this. Only U(p) gets inserted.
    for s in chain!(u_p, &c_p) {
        status_queue.insert(*s, segments, &p);
    }

    // "if U(p) ∪ C(p) = ∅" [1, p. 26]
    if chain!(u_p, &c_p).next().is_none() {
        // "then Let sl and sr be the left and right neighbors of p in T." [1, p. 26]
        let l_r = status_queue.left_of_event(segments, &p);
        let u_r = status_queue.right_of_event(segments, &p);

        if let (Some(left), Some(right)) = (l_r, u_r) {
            find_new_event_fast(left, right, event, segments, event_queue);
        }
    } else {
        let s_dash = status_queue.left_most(segments, &p);
        let s_l = status_queue.left_of_event(segments, &p);
        let s_dash_dash = status_queue.right_most(segments, &p);
        let s_r = status_queue.right_of_event(segments, &p);

        if let (Some(left), Some(right)) = (s_l, s_dash) {
            find_new_event_fast(left, right, event, segments, event_queue);
        }
        if let (Some(left), Some(right)) = (s_dash_dash, s_r) {
            find_new_event_fast(left, right, event, segments, event_queue);
        }
    }
}

fn find_new_event_fast(
    s_l: SegmentIdx,
    s_r: SegmentIdx,
    event: &Event,
    segments: &Segments,
    event_queue: &mut EventQueue,
) {
    if let Some(intersection) = Segment::intersect(s_l, s_r, segments, 0)
        && intersection.typ().is_point()
        && (intersection.point1().y < event.y
            || intersection.point1().y == event.y && intersection.point1().x > event.x)
    {
        event_queue.insert(Event::new(
            intersection.point1().y,
            intersection.point1().x,
            std::iter::empty(),
        ));
    }
}
