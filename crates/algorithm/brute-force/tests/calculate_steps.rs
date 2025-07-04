use brute_force::{AlgorithmStep, calculate_steps};
use common::intersection::{Intersection, IntersectionIdx, IntersectionType};
use common::{
    AlgoSteps,
    intersection::Intersections,
    segment::{Segment, Segments},
};
use googletest::prelude::*;
use smallvec::smallvec;
#[gtest]
fn empty() {
    let segments = Segments::new();
    let mut intersections = Intersections::new();
    let mut steps = AlgoSteps::new();

    calculate_steps(&segments, &mut intersections, &mut steps);

    expect_eq!(segments.len(), 0);
    expect_eq!(intersections.len(), 0);
    expect_that!(
        steps,
        elements_are![eq(&AlgorithmStep::Init), eq(&AlgorithmStep::End),]
    );
}

#[gtest]
fn one() {
    let mut segments = Segments::new();
    segments.push(Segment::new((-50, 0), (50, 0)));
    let mut intersections = Intersections::new();
    let mut steps = AlgoSteps::new();

    calculate_steps(&segments, &mut intersections, &mut steps);

    expect_eq!(segments.len(), 1);
    expect_eq!(intersections.len(), 0);
    expect_that!(
        steps,
        elements_are![eq(&AlgorithmStep::Init), eq(&AlgorithmStep::End),]
    );
}

#[gtest]
fn two_intersection() {
    let mut segments = Segments::new();
    segments.push(Segment::new((-50, 0), (50, 0)));
    segments.push(Segment::new((0, -50), (0, 50)));
    let mut intersections = Intersections::new();
    let mut steps = AlgoSteps::new();

    calculate_steps(&segments, &mut intersections, &mut steps);

    expect_eq!(segments.len(), 2);
    expect_eq!(intersections.len(), 1);
    expect_that!(
        intersections,
        elements_are![eq(&common::intersection::Intersection::new(
            common::intersection::IntersectionType::Point {
                coord: (0.0, 0.0).into()
            },
            smallvec![0.into(), 1.into()],
            1,
        ))]
    );
    expect_that!(
        steps,
        elements_are![
            eq(&AlgorithmStep::Init),
            eq(&AlgorithmStep::Running {
                step: 1,
                i: 0,
                j: 1,
                segment_i: 0.into(),
                segment_j: 1.into(),
                intersection: Some(0.into())
            }),
            eq(&AlgorithmStep::End),
        ]
    );
}

#[gtest]
fn two_no_intersection() {
    let mut segments = Segments::new();
    segments.push(Segment::new((-50, 0), (50, 0)));
    segments.push(Segment::new((100, -50), (100, 50)));
    let mut intersections = Intersections::new();
    let mut steps = AlgoSteps::new();

    calculate_steps(&segments, &mut intersections, &mut steps);

    expect_eq!(segments.len(), 2);
    expect_that!(intersections, elements_are![]);
    expect_that!(
        steps,
        elements_are![
            eq(&AlgorithmStep::Init),
            eq(&AlgorithmStep::Running {
                step: 1,
                i: 0,
                j: 1,
                segment_i: 0.into(),
                segment_j: 1.into(),
                intersection: None
            }),
            eq(&AlgorithmStep::End),
        ]
    );
}

#[gtest]
fn five_segments() {
    let mut segments = Segments::new();
    segments.push(Segment::new((-50, 0), (50, 0)));
    segments.push(Segment::new((0, -50), (0, 50)));
    segments.push(Segment::new((-50, -50), (50, 50)));
    segments.push(Segment::new((-50, 50), (50, -50)));
    segments.push(Segment::new((-1000, -1238), (-900, -900)));

    let mut intersections = Intersections::new();
    let mut steps = AlgoSteps::new();

    calculate_steps(&segments, &mut intersections, &mut steps);

    expect_eq!(segments.len(), 5);
    expect_that!(
        intersections,
        elements_are![
            eq(&common::intersection::Intersection::new(
                common::intersection::IntersectionType::Point {
                    coord: (0.0, 0.0).into()
                },
                smallvec![0.into(), 1.into()],
                1,
            )),
            eq(&common::intersection::Intersection::new(
                common::intersection::IntersectionType::Point {
                    coord: (0.0, 0.0).into()
                },
                smallvec![0.into(), 2.into()],
                2,
            )),
            eq(&common::intersection::Intersection::new(
                common::intersection::IntersectionType::Point {
                    coord: (0.0, 0.0).into()
                },
                smallvec![0.into(), 3.into()],
                3,
            )),
            eq(&common::intersection::Intersection::new(
                common::intersection::IntersectionType::Point {
                    coord: (0.0, 0.0).into()
                },
                smallvec![1.into(), 2.into()],
                5,
            )),
            eq(&common::intersection::Intersection::new(
                common::intersection::IntersectionType::Point {
                    coord: (0.0, 0.0).into()
                },
                smallvec![1.into(), 3.into()],
                6,
            )),
            eq(&common::intersection::Intersection::new(
                common::intersection::IntersectionType::Point {
                    coord: (0.0, 0.0).into()
                },
                smallvec![2.into(), 3.into()],
                8,
            )),
        ]
    );

    expect_that!(
        steps,
        elements_are![
            eq(&AlgorithmStep::Init),
            eq(&AlgorithmStep::Running {
                step: 1,
                i: 0,
                j: 1,
                segment_i: 0.into(),
                segment_j: 1.into(),
                intersection: Some(0.into())
            }),
            eq(&AlgorithmStep::Running {
                step: 2,
                i: 0,
                j: 2,
                segment_i: 0.into(),
                segment_j: 2.into(),
                intersection: Some(1.into())
            }),
            eq(&AlgorithmStep::Running {
                step: 3,
                i: 0,
                j: 3,
                segment_i: 0.into(),
                segment_j: 3.into(),
                intersection: Some(2.into())
            }),
            eq(&AlgorithmStep::Running {
                step: 4,
                i: 0,
                j: 4,
                segment_i: 0.into(),
                segment_j: 4.into(),
                intersection: None
            }),
            eq(&AlgorithmStep::Running {
                step: 5,
                i: 1,
                j: 2,
                segment_i: 1.into(),
                segment_j: 2.into(),
                intersection: Some(3.into())
            }),
            eq(&AlgorithmStep::Running {
                step: 6,
                i: 1,
                j: 3,
                segment_i: 1.into(),
                segment_j: 3.into(),
                intersection: Some(4.into())
            }),
            eq(&AlgorithmStep::Running {
                step: 7,
                i: 1,
                j: 4,
                segment_i: 1.into(),
                segment_j: 4.into(),
                intersection: None
            }),
            eq(&AlgorithmStep::Running {
                step: 8,
                i: 2,
                j: 3,
                segment_i: 2.into(),
                segment_j: 3.into(),
                intersection: Some(5.into())
            }),
            eq(&AlgorithmStep::Running {
                step: 9,
                i: 2,
                j: 4,
                segment_i: 2.into(),
                segment_j: 4.into(),
                intersection: None
            }),
            eq(&AlgorithmStep::Running {
                step: 10,
                i: 3,
                j: 4,
                segment_i: 3.into(),
                segment_j: 4.into(),
                intersection: None
            }),
            eq(&AlgorithmStep::End)
        ]
    );
}

#[gtest]
fn feature23() {
    let segments =
        Segments::from_iter([Segment::new((-1, 0), (0, 0)), Segment::new((1, 0), (-1, 0))]);
    let mut intersections = Intersections::new();
    let mut steps = AlgoSteps::new();
    calculate_steps(&segments, &mut intersections, &mut steps);
    let inter: &Intersection = &intersections[IntersectionIdx::from(0)];
    expect_that!(
        inter,
        pat!(Intersection {
            typ: pat!(IntersectionType::Parallel {
                line: pat!(Segment {
                    upper: eq(&(-1, 0).into()),
                    lower: eq(&(0, 0).into()),
                    ..
                })
            }),
            ..
        })
    );
}
