#[cfg(feature = "ui")]
pub mod ui;

use std::collections::HashSet;

use common::{
    AlgoSteps,
    intersection::{InterVec, IntersectionType, LeanIntersection, LeanIntersections},
    math::cartesian::CartesianCoord,
    segment::{Segment, SegmentIdx, Segments},
};
use itertools::{Itertools, chain};
use sweep_utils::{event::EventQueue, status::StatusQueue};

use crate::step::{Step, StepType};

mod step;

struct State<'a, 'b> {
    segments: &'a Segments,
    intersections: &'b mut LeanIntersections,
    event_queue: EventQueue,
    status_queue: StatusQueue,
    p: Option<CartesianCoord>,
    u_p: Option<HashSet<SegmentIdx>>,
    c_p: Option<Vec<SegmentIdx>>,
    l_p: Option<Vec<SegmentIdx>>,
}

impl<'a, 'b> State<'a, 'b> {
    fn new(segments: &'a Segments, intersections: &'b mut LeanIntersections) -> Self {
        intersections.clear();
        Self {
            segments,
            intersections,
            p: None,
            u_p: None,
            c_p: None,
            l_p: None,
            event_queue: EventQueue::new(),
            status_queue: StatusQueue::new(),
        }
    }
    fn report(&self, step: StepType, steps: &mut AlgoSteps<Step>) {
        steps.push(
            Step::builder(step, steps.len())
                .maybe_c_p(self.c_p.clone())
                .maybe_l_p(self.l_p.clone())
                .status_queue(self.status_queue.iter())
                .event_queue(self.event_queue.clone())
                .maybe_u_p(self.u_p.clone())
                .maybe_p(self.p.clone())
                .merge_queue(Vec::new())
                .build(),
        );
    }
    fn set_event(&mut self, (p, u_p): (CartesianCoord, HashSet<SegmentIdx>)) {
        self.p = Some(p);
        self.u_p = Some(u_p);
    }
    fn reset(&mut self) {
        self.p = None;
        self.u_p = None;
        self.c_p = None;
        self.l_p = None;
    }
}

pub fn calculate_steps(
    segments: &Segments,
    intersections: &mut LeanIntersections,
    megerd_intersections: &mut LeanIntersections,
    steps: &mut AlgoSteps<Step>,
) {
    let mut state = State::new(segments, intersections);
    steps.clear();
    megerd_intersections.clear();
    state.report(StepType::Init, steps);

    // Initialize an empty event queue Q.
    state.report(StepType::StartInitQ, steps);
    // Next, insert the segment endpoints into Q; when an upper endpoint is inserted, the corresponding segment should be stored with it.
    for (id, segment) in state.segments.iter_enumerated() {
        // We store the segment id for the upper one
        state.event_queue.insert(segment.upper.clone(), id);
        // We do not store the segment id for the lower one
        state.event_queue.insert(segment.lower.clone(), None);
        state.report(StepType::InitQ { segment: id }, steps);
    }

    // Initialize an empty status structure T.
    state.report(StepType::InitT, steps);

    // while Q is not empty. do Determine the next event point p in Q and delete it.
    while let Some(event) = state.event_queue.pop() {
        state.reset();
        state.set_event(event);
        state.report(StepType::PopQ, steps);
        handle_event_point(&mut state, steps);
        // HANDLE EVENT POINT(p)
    }
    let _ = state;
    *megerd_intersections = merge_intersections(intersections, steps);
    steps.push(Step::builder(StepType::End, steps.len()).build());
}

#[allow(clippy::too_many_lines, reason = "because capturing status cost a lot")]
#[allow(
    clippy::too_many_arguments,
    reason = "because capturing status cost a lot"
)]
fn handle_event_point(state: &mut State, steps: &mut AlgoSteps<Step>) {
    let p = state.p.as_ref().expect("Must be set");
    // "Let U(p) be the set of segments whose upper endpoint is p; these segments
    // are stored with the event point p. (For horizontal segments, the upper
    // endpoint is by definition the left endpoint.)" [1, p. 26]
    let u_p = state.u_p.as_ref().expect("Must be set");

    let (l_p, c_p): (Vec<_>, Vec<_>) = state
        .status_queue
        .iter_contains(state.segments, p)
        .partition(|v| &state.segments[*v].lower == p);
    state.c_p = Some(c_p);
    state.l_p = Some(l_p);
    state.report(StepType::CalculateSets, steps);
    let c_p = state.c_p.as_ref().expect("Should be set");
    let l_p = state.l_p.as_ref().expect("Should be set");

    // "if L(p) ∪ U(p) ∪ C(p) contains more than one segment [...]" [1, p. 26]
    let l_p_and_u_p_and_c_p: InterVec = chain!(u_p, l_p, c_p).copied().collect();
    state.report(
        StepType::CalculateUpCpLp {
            up_cp_lp: l_p_and_u_p_and_c_p.clone(),
        },
        steps,
    );

    if l_p_and_u_p_and_c_p.len() > 1 {
        for (s1, s2) in l_p_and_u_p_and_c_p.iter().tuple_combinations() {
            let intersect = LeanIntersection::new(
                common::intersection::IntersectionType::Point { coord: p.clone() },
                [*s1, *s2],
                steps.len(),
            );
            let intersect = state.intersections.push_and_get_key(intersect);
            state.report(
                StepType::ReportIntersections {
                    intersection: intersect,
                },
                steps,
            );
        }
        // "[...] then Report p as an intersection, together with L(p), U(p), and C(p)." [1, p. 26]
    }

    // "Delete the segments in L(p) ∪ C(p) from T." [1, p. 26]
    // We only retain elements which are *not* in l_p
    // We do not do u_p, because how the status is defined and calculated, it is not needed
    for s in chain!(l_p, c_p) {
        state.status_queue.delete(*s, state.segments, p);
    }
    state.report(StepType::DeleteLpCp, steps);

    // "Insert the segments in U(p) ∪ C(p) into T." [1, p. 26]
    // C(p) is already in the status_queue, so we do not need this. Only U(p) gets inserted.
    for s in chain!(u_p, c_p) {
        state.status_queue.insert(*s, state.segments, p);
    }

    state.report(StepType::InsertUpCp, steps);

    // "if U(p) ∪ C(p) = ∅" [1, p. 26]
    if chain!(u_p, c_p).next().is_none() {
        // "then Let sl and sr be the left and right neighbors of p in T." [1, p. 26]
        let l_r = state.status_queue.left_of_event(state.segments, p);
        let u_r = state.status_queue.right_of_event(state.segments, p);
        state.report(StepType::UpCpEmpty { s_l: l_r, s_r: u_r }, steps);

        if let (Some(left), Some(right)) = (l_r, u_r) {
            find_new_event(left, right, state, steps);
        }
    } else {
        let s_dash = state.status_queue.left_most(state.segments, p);
        let s_l = state.status_queue.left_of_event(state.segments, p);
        let s_dash_dash = state.status_queue.right_most(state.segments, p);
        let s_r = state.status_queue.right_of_event(state.segments, p);
        //println!("s_dash {s_dash:?}, s_l {s_l:?}, s_dash_dash {s_dash_dash:?}, s_r {s_r:?}");
        state.report(
            StepType::UpCpNotEmpty {
                s_dash,
                s_dash_dash,
                s_l,
                s_r,
            },
            steps,
        );
        if let (Some(left), Some(right)) = (s_l, s_dash) {
            find_new_event(left, right, state, steps);
        }
        if let (Some(left), Some(right)) = (s_dash_dash, s_r) {
            find_new_event(left, right, state, steps);
        }
    }
}

fn find_new_event(
    s_l: SegmentIdx,
    s_r: SegmentIdx,
    state: &mut State,
    steps: &mut AlgoSteps<Step>,
) {
    state.report(StepType::FindNewEvent { s_l, s_r }, steps);
    let p = state.p.as_ref().expect("Must be set");
    if let Some(intersection) = Segment::intersect(s_l, s_r, state.segments, 0)
        && intersection.typ().is_point()
        && (intersection.point1().y < p.y
            || intersection.point1().y == p.y && intersection.point1().x > p.x)
    {
        state.report(
            StepType::InsertIntersectionEvent {
                s_l,
                s_r,
                intersection: (
                    intersection.point1().x.clone(),
                    intersection.point1().y.clone(),
                ),
            },
            steps,
        );
        state
            .event_queue
            .insert(intersection.point1().clone(), None);
    }
}

fn merge_intersections(
    intersections: &LeanIntersections,
    steps: &mut AlgoSteps<Step>,
) -> LeanIntersections {
    let mut map: indexmap::IndexMap<[SegmentIdx; 2], Vec<&CartesianCoord>> =
        indexmap::IndexMap::new();
    for (idx, intersection) in intersections.iter_enumerated() {
        steps.push(
            Step::builder(StepType::InsertMergeQueue { inter: idx }, steps.len())
                .merge_queue(
                    map.iter()
                        .map(|(l, r)| (*l, r.iter().copied().cloned().collect::<Vec<_>>())),
                )
                .build(),
        );
        map.entry(intersection.segments)
            .and_modify(|v| v.push(intersection.point1()))
            .or_insert(vec![&intersection.point1()]);
    }
    let mut result = LeanIntersections::new();
    for (seg, points) in &map {
        match points.len() {
            0 => unreachable!(),
            1 => {
                result.push(LeanIntersection::new(
                    IntersectionType::Point {
                        coord: points[0].clone(),
                    },
                    *seg,
                    steps.len(),
                ));
            }
            _ => {
                let a = result.push_and_get_key(LeanIntersection::new(
                    IntersectionType::Parallel {
                        line: Segment::new(
                            points[0].clone(),
                            points.last().copied().cloned().unwrap(),
                        ),
                    },
                    *seg,
                    steps.len(),
                ));
                steps.push(
                    Step::builder(
                        StepType::Merge {
                            seg: *seg,
                            points: points.iter().copied().cloned().collect_vec(),
                            result: a,
                        },
                        steps.len(),
                    )
                    .merge_queue(
                        map.iter()
                            .map(|(l, r)| (*l, r.iter().copied().cloned().collect::<Vec<_>>())),
                    )
                    .build(),
                );
            }
        }
    }

    result
}
