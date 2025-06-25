use common::test::approx_eq;
use common::{
    AlgoSteps,
    intersection::{Intersection, IntersectionType, Intersections},
    math::cartesian::CartesianCoord,
    segment::{Segment, SegmentIdx, Segments},
};
use googletest::prelude::*;
use sweep::{Step, StepType, calculate_steps};
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
                typ: eq(&StepType::InitQ),
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
                typ: eq(&StepType::HEPUpdateT),
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
                typ: eq(&StepType::DeleteLp),
                ..
            }),
            pat!(Step {
                typ: eq(&StepType::InsertUp),
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
                typ: eq(&StepType::HEPUpdateT),
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
                typ: eq(&StepType::DeleteLp),
                ..
            }),
            pat!(Step {
                typ: eq(&StepType::InsertUp),
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
#[ignore = "reason"]
fn two() {
    let mut segments = Segments::new();
    segments.push(Segment::new((2, 2), (-2, -2)));
    segments.push(Segment::new((-2, 2), (2, -2)));

    let mut intersections = Intersections::new();
    let mut steps = AlgoSteps::new();

    calculate_steps(&segments, &mut intersections, &mut steps);

    expect_that!(steps, elements_are![]);
    expect_that!(
        intersections,
        elements_are![pat!(Intersection {
            typ: pat!(IntersectionType::Point {
                coord: approx_eq(&CartesianCoord::new(0, 0))
            }),
            segments: container_eq([SegmentIdx::from(0), SegmentIdx::from(1)]),
            ..
        })]
    );
}
