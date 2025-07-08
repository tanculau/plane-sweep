use std::collections::HashSet;

use common::{
    intersection::{Intersection, Intersections},
    math::cartesian::CartesianCoord,
    segment::{Segment, SegmentIdx, Segments},
};
use itertools::{Itertools, chain};
use smallvec::SmallVec;
use sweep_utils::{event::EventQueue, status::StatusQueue};

pub fn calculate(segments: &Segments, intersections: &mut Intersections) {
    intersections.clear();
    let mut event_queue = EventQueue::new();
    for (id, segment) in segments.iter_enumerated() {
        event_queue.insert(segment.upper.clone(), id);
        event_queue.insert(segment.lower.clone(), None);
    }

    let mut status_queue = StatusQueue::new();

    while let Some(event) = event_queue.pop() {
        handle_event_point_fast(
            event,
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
    (p, u_p): (CartesianCoord, HashSet<SegmentIdx>),
    event_queue: &mut EventQueue,
    segments: &Segments,
    intersections: &mut Intersections,
    status_queue: &mut StatusQueue,
) {
    let (l_p, c_p): (SmallVec<_, 10>, SmallVec<_, 10>) = status_queue
        .iter_contains(segments, &p)
        .partition(|v| segments[*v].clone().lower == p);

    if u_p.len() + c_p.len() + l_p.len() > 1 {
        let intersect = Intersection::new(
            common::intersection::IntersectionType::Point { coord: p.clone() },
            c_p.iter().chain(l_p.iter()).chain(&u_p).copied().collect(),
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
            "Tried to remove {s:?} from {status_queue:?} with last event {p:?}"
        );
    }

    // "Insert the segments in U(p) ∪ C(p) into T." [1, p. 26]
    // C(p) is already in the status_queue, so we do not need this. Only U(p) gets inserted.
    for s in chain!(&u_p, &c_p) {
        status_queue.insert(*s, segments, &p);
    }

    // "if U(p) ∪ C(p) = ∅" [1, p. 26]
    if chain!(&u_p, &c_p).next().is_none() {
        // "then Let sl and sr be the left and right neighbors of p in T." [1, p. 26]
        let l_r = status_queue.left_of_event(segments, &p);
        let u_r = status_queue.right_of_event(segments, &p);

        if let (Some(left), Some(right)) = (l_r, u_r) {
            find_new_event_fast(left, right, &p, segments, event_queue);
        }
    } else {
        let s_dash = status_queue.left_most(segments, &p);
        let s_l = status_queue.left_of_event(segments, &p);
        let s_dash_dash = status_queue.right_most(segments, &p);
        let s_r = status_queue.right_of_event(segments, &p);

        if let (Some(left), Some(right)) = (s_l, s_dash) {
            find_new_event_fast(left, right, &p, segments, event_queue);
        }
        if let (Some(left), Some(right)) = (s_dash_dash, s_r) {
            find_new_event_fast(left, right, &p, segments, event_queue);
        }
    }
}

fn find_new_event_fast(
    s_l: SegmentIdx,
    s_r: SegmentIdx,
    event: &CartesianCoord,
    segments: &Segments,
    event_queue: &mut EventQueue,
) {
    if let Some(intersection) = Segment::intersect(s_l, s_r, segments, 0)
        && intersection.typ().is_point()
        && (intersection.point1().y < event.y
            || intersection.point1().y == event.y && intersection.point1().x > event.x)
    {
        event_queue.insert(intersection.point1().clone(), None);
    }
}
