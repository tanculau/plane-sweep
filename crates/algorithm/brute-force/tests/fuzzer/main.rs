use bolero::{TypeGenerator, check};
use brute_force::calculate_steps;
use common::{AlgoSteps, intersection::Intersections, segment::Segment};

fn main() {
    check!()
        .with_type::<Vec<Input>>()
        .with_max_len(10)
        .for_each(|input| {
            let segments = input
                .iter()
                .map(|i| Segment::new((i.upper_x, i.upper_y), (i.lower_x, i.lower_y)))
                .collect();
            let mut steps = AlgoSteps::new();
            let mut intersections = Intersections::new();
            calculate_steps(&segments, &mut intersections, &mut steps);
        });
}

#[derive(Debug, TypeGenerator)]
pub struct Input {
    upper_x: i32,
    upper_y: i32,
    lower_x: i32,
    lower_y: i32,
}
