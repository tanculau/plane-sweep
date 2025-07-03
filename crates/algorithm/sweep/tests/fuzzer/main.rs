use bolero::{TypeGenerator, check};
use common::{AlgoSteps, intersection::Intersections, segment::Segment};
use sweep::calculate_steps;

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
            let mut steps = AlgoSteps::new();
            let mut intersections = Intersections::new();
            calculate_steps::<false>(&segments, &mut intersections, &mut steps);
        });
}

#[derive(TypeGenerator)]
pub struct Input {
    upper_x: i8,
    upper_y: i8,
    lower_x: i8,
    lower_y: i8,
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
