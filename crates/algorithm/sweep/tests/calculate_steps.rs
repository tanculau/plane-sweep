use common::{
    AlgoSteps,
    intersection::{InterVec, Intersection, IntersectionType, Intersections},
    math::Float,
    segment::{Segment, Segments},
};
use googletest::prelude::*;
use rstest::rstest;
use sweep::{
    calculate_steps,
    step::{Step, StepType},
};
#[gtest]
fn empty() {
    let segments = Segments::new();
    let mut intersections = Intersections::new();
    let mut steps = AlgoSteps::new();

    calculate_steps(&segments, &mut intersections, &mut steps);

    expect_that!(
        steps,
        elements_are![
            pat!(Step {
                typ: eq(&StepType::Init),
                ..
            }),
            pat!(Step {
                typ: eq(&StepType::StartInitQ),
                ..
            }),
            pat!(Step {
                typ: eq(&StepType::InitT),
                ..
            }),
            pat!(Step {
                typ: eq(&StepType::End),
                ..
            })
        ]
    );
    expect_that!(intersections, elements_are![]);
}

#[gtest]
fn one() {
    let mut segments = Segments::new();
    segments.push(Segment::new((-2, 2), (2, -2)));
    let mut intersections = Intersections::new();
    let mut steps = AlgoSteps::new();

    calculate_steps(&segments, &mut intersections, &mut steps);

    expect_that!(
        steps,
        elements_are![
            pat!(Step {
                typ: eq(&StepType::Init),
                ..
            }),
            pat!(Step {
                typ: eq(&StepType::StartInitQ),
                ..
            }),
            pat!(Step {
                typ: pat!(StepType::InitQ { .. }),
                ..
            }),
            pat!(Step {
                typ: eq(&StepType::InitT),
                ..
            }),
            pat!(Step {
                typ: eq(&StepType::PopQ),
                ..
            }),
            pat!(Step {
                typ: eq(&StepType::CalculateSets),
                ..
            }),
            pat!(Step {
                typ: pat!(StepType::CalculateUpCpLp {
                    up_cp_lp: anything()
                }),
                ..
            }),
            pat!(Step {
                typ: eq(&StepType::DeleteLpCp),
                ..
            }),
            pat!(Step {
                typ: eq(&StepType::InsertUpCp),
                ..
            }),
            pat!(Step {
                typ: pat!(StepType::UpCpNotEmpty { .. }),
                ..
            }),
            pat!(Step {
                typ: eq(&StepType::PopQ),
                ..
            }),
            pat!(Step {
                typ: eq(&StepType::CalculateSets),
                ..
            }),
            pat!(Step {
                typ: pat!(StepType::CalculateUpCpLp {
                    up_cp_lp: anything()
                }),
                ..
            }),
            pat!(Step {
                typ: eq(&StepType::DeleteLpCp),
                ..
            }),
            pat!(Step {
                typ: eq(&StepType::InsertUpCp),
                ..
            }),
            pat!(Step {
                typ: pat!(StepType::UpCpEmpty { .. }),
                ..
            }),
            pat!(Step {
                typ: eq(&StepType::End),
                ..
            })
        ]
    );
    expect_that!(intersections, elements_are![]);
}

#[gtest]
fn many() {
    let segments = Segments::from_iter([
        Segment::new((2, 2), (-2, -2)), // 0
        Segment::new((2, 2), (2, -2)),  // 1
        Segment::new(
            (-4, Float::new_neg(3_u8, 2_u8)),
            (4, Float::new_neg(3_u8, 2_u8)),
        ), // 2
        Segment::new(
            (Float::new_neg(1_u8, 2_u8), 5),
            (Float::new_neg(1_u8, 2_u8), Float::new_neg(9_u8, 2_u8)),
        ), // 3
        Segment::new(
            (Float::new_neg(3_u8, 2_u8), Float::new(7_u8, 2_u8)),
            (Float::new_neg(3_u8, 2_u8), Float::new_neg(9_u8, 2_u8)),
        ), // 4
        Segment::new(
            (Float::new_neg(3_u8, 2_u8), Float::new_neg(9_u8, 2_u8)),
            (Float::new_neg(1_u8, 2_u8), Float::new_neg(9_u8, 2_u8)),
        ), // 5
        Segment::new(
            (Float::new_neg(1_u8, 2_u8), Float::new_neg(9_u8, 2_u8)),
            (3, Float::new_neg(9_u8, 2_u8)),
        ), // 6
    ]);
    let mut intersections = Intersections::new();
    let mut steps = AlgoSteps::new();
    calculate_steps(&segments, &mut intersections, &mut steps);

    for i in &mut intersections {
        // The order is not relevant
        i.segments.sort_unstable();
    }

    expect_that!(
        &intersections,
        elements_are![
            pat!(Intersection {
                typ: pat!(IntersectionType::Point {
                    coord: eq(&(2, 2).into())
                }),
                segments: eq(&InterVec::from_iter([0.into(), 1.into()])),
                ..
            }),
            pat!(Intersection {
                typ: pat!(IntersectionType::Point {
                    coord: eq(&(Float::new_neg(1_u8, 2_u8), Float::new_neg(1_u8, 2_u8)).into())
                }),
                segments: eq(&InterVec::from_iter([0.into(), 3.into()])),
                ..
            }),
            pat!(Intersection {
                typ: pat!(IntersectionType::Point {
                    coord: eq(&(Float::new_neg(3_u8, 2_u8), Float::new_neg(3_u8, 2_u8)).into())
                }),
                segments: eq(&InterVec::from_iter([0.into(), 2.into(), 4.into()])),
                ..
            }),
            pat!(Intersection {
                typ: pat!(IntersectionType::Point {
                    coord: eq(&(Float::new_neg(1_u8, 2_u8), Float::new_neg(3_u8, 2_u8)).into())
                }),
                segments: eq(&InterVec::from_iter([2.into(), 3.into()])),
                ..
            }),
            pat!(Intersection {
                typ: pat!(IntersectionType::Point {
                    coord: eq(&(Float::new(2_u8, 1_u8), Float::new_neg(3_u8, 2_u8)).into())
                }),
                segments: eq(&InterVec::from_iter([1.into(), 2.into()])),
                ..
            }),
            pat!(Intersection {
                typ: pat!(IntersectionType::Point {
                    coord: eq(&(Float::new_neg(3_u8, 2_u8), Float::new_neg(9_u8, 2_u8)).into())
                }),
                segments: eq(&InterVec::from_iter([4.into(), 5.into()])),
                ..
            }),
            pat!(Intersection {
                typ: pat!(IntersectionType::Point {
                    coord: eq(&(Float::new_neg(1_u8, 2_u8), Float::new_neg(9_u8, 2_u8)).into())
                }),
                segments: eq(&InterVec::from_iter([3.into(), 5.into(), 6.into()])),
                ..
            }),
        ]
    );
}

#[test]
fn test_failure() {
    let segments = Segments::from_iter([
        Segment::new((-254, 9992), (-1, -258)),
        Segment::new((-258, 8), (113, 0)),
        Segment::new((188, 0), (0, 0)),
    ]);
    let mut intersections = Intersections::new();
    let mut steps = AlgoSteps::new();
    calculate_steps(&segments, &mut intersections, &mut steps);
}

/// Input that caused crashes while fuzzing
#[gtest]
#[rstest]
#[case([
    Segment::new((0, 113), (113, 0)),
    Segment::new((-1, 0), (-1, 113)),
    Segment::new((0, 79), (59, 70)),
    Segment::new((1, 117), (0, 43)),
    Segment::new((69, 10), (-1, 3)),
    Segment::new((23, 93), (0, 0)),
])]
#[case([
        Segment::new((0, 1), (-10, -105)),
        Segment::new((0, 0), (-105, -105)),
        Segment::new((-10, -105), (0, 0)),
    ])]
#[case([
    Segment::new((1, -1), (-128, -1)),
    Segment::new((0, 0), (-1, -1)),
    Segment::new((-30, -1), (-1, -30)),
    Segment::new((-30, -30), (-30, 33)),
    Segment::new((0, -1), (-30, 0)),
    ])]
#[case([
    Segment::new((1, 0), (0, -128)),
    Segment::new((2, 0), (0, 0)),
])]
#[case([
    Segment::new((-128, 0), (0, -39)),
    Segment::new((-30, -30), (35, 35)),
    Segment::new((35, 0), (0, 0)),
])]
#[case([
    Segment::new((0, 0), (0, 1)),
    Segment::new((1, 0), (0, 0)),
    Segment::new((1, 0), (0, 0)),
])]
#[case([
    Segment::new((0, -128), (1, 1)),
    Segment::new((1, 0), (0, 0)),
])]
#[case([
    Segment::new((0, 0), (1, -128)),
    Segment::new((2, -128), (-128, 0)),
])]
fn crashes<const T: usize>(#[case] segments: [Segment; T]) {
    let segments = Segments::from_iter(segments);
    let mut steps = AlgoSteps::new();
    let mut intersections = Intersections::new();
    calculate_steps(&segments, &mut intersections, &mut steps);
}
