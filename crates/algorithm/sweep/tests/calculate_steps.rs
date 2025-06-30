use common::{
    AlgoSteps,
    intersection::Intersections,
    segment::{Segment, Segments},
};
use googletest::prelude::*;
use sweep::{
    calculate_steps,
    step::{Step, StepType},
};
#[gtest]
fn empty() {
    let segments = Segments::new();
    let mut intersections = Intersections::new();
    let mut steps = AlgoSteps::new();

    calculate_steps::<true>(&segments, &mut intersections, &mut steps);

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

    calculate_steps::<true>(&segments, &mut intersections, &mut steps);

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
