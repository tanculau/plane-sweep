use std::collections::{BTreeSet, HashSet};

use bolero::{TypeGenerator, check};
use common::{
    AlgoSteps,
    intersection::{Intersections, LeanIntersection},
    segment::Segment,
};

fn main() {
    check!()
        .with_type::<Vec<Input>>()
        .with_max_len(100)
        .for_each(|input| {
            let segments = input
                .iter()
                .map(|i| Segment::new((i.upper_x, i.upper_y), (i.lower_x, i.lower_y)))
                .filter(|s| s.upper != s.lower)
                .collect();
            let mut sweep_intersections = Intersections::new();
            sweep_fast::calculate(&segments, &mut sweep_intersections);
            let mut brute_intersections = Intersections::new();
            brute_force::calculate(&segments, &mut brute_intersections);
            let sweep: BTreeSet<LeanIntersection> =
                common::intersection::to_lines(&sweep_intersections)
                    .into_iter()
                    .collect();
            let brute: BTreeSet<LeanIntersection> =
                (common::intersection::to_lines(&brute_intersections)
                    .into_iter()
                    .collect());
            assert_eq!(sweep, brute);
        });
}
#[derive(TypeGenerator)]
pub struct Input {
    upper_x: i32,
    upper_y: i32,
    lower_x: i32,
    lower_y: i32,
}

impl core::fmt::Debug for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Segment::new(({}, {}), ({}, {}))",
            self.upper_x, self.upper_y, self.lower_x, self.lower_y
        )
    }
}
