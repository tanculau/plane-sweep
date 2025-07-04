use common::segment::Segment;
use googletest::prelude::*;

#[gtest]
fn new() {
    let p1 = (4, 4);
    let p2 = (-4, -4);

    let s1 = Segment::new(p1, p2);
    let s2 = Segment::new(p2, p1);

    expect_eq!(s1.upper, p1.into());
    expect_eq!(s1.upper, s2.upper);
    expect_eq!(s1.lower, s2.lower);

    let p3 = (4, 4);
    let p4 = (6, 4);

    let s3 = Segment::new(p3, p4);
    let s4 = Segment::new(p4, p3);

    expect_eq!(s3.upper, p3.into());
    expect_eq!(s3.upper, s4.upper);
    expect_eq!(s3.lower, s4.lower);
}
#[gtest]
fn update() {
    let p1 = (4, 4);
    let p2 = (-4, -4);

    let mut s1 = Segment::new(p1, p2);
    expect_eq!(s1.upper, p1.into());
    expect_eq!(s1.lower, p2.into());
    core::mem::swap(&mut s1.upper, &mut s1.lower);
    s1.update();
    expect_eq!(s1.upper, p1.into());
    expect_eq!(s1.lower, p2.into());
}

#[gtest]
fn vertical_horizontal() {
    let vertical = Segment::new((0, 4), (0, -4));
    expect_true!(vertical.is_vertical());
    expect_false!(vertical.is_horizontal());

    let horizontal = Segment::new((4, 0), (0, 0));
    expect_true!(horizontal.is_horizontal());
    expect_false!(horizontal.is_vertical());

    let tilted = Segment::new((4, 0), (0, 1));
    expect_false!(tilted.is_horizontal());
    expect_false!(tilted.is_vertical());
}

mod intersection {
    use common::{
        intersection::{InterVec, Intersection, IntersectionType},
        segment::{Segment, Segments},
    };
    use googletest::prelude::*;

    #[gtest]
    fn none() {
        let segments = Segments::from_iter([
            Segment::new((12, 0), (0, 0)),
            Segment::new((-12, 0), (-4, 0)),
        ]);

        let actual = Segment::intersect(0, 1, &segments, 0);
        expect_eq!(actual, None);
    }

    #[gtest]
    fn simple() {
        let segments = Segments::from_iter([
            Segment::new((12, 0), (-12, 0)),
            Segment::new((0, 12), (0, -12)),
        ]);
        let actual = Segment::intersect(0, 1, &segments, 0);
        expect_eq!(
            actual,
            Some(Intersection::new(
                IntersectionType::Point {
                    coord: (0, 0).into()
                },
                smallvec::smallvec![0.into(), 1.into()],
                0
            ))
        );
    }

    #[gtest]
    #[rstest::rstest]
    #[case((-12, 0), (12, 0), (0, 0), (24, 0), (0, 0), (12, 0))]
    #[case((-12, 0), (12, 0), (-24, 0), (0, 0), (-12, 0), (0, 0))]
    #[case((-12, 0), (12, 0), (-12, 0), (12, 0), (-12, 0), (12, 0))]
    #[case((-12, 0), (12, 0), (-6, 0), (6, 0), (-6, 0), (6, 0))]
    #[case((-12, 0), (12, 0), (-12, 0), (6, 0), (-12, 0), (6, 0))]
    #[case((-12, 0), (12, 0), (-6, 0), (12, 0), (-6, 0), (12, 0))]
    fn overlay(
        #[case] upper_1: (i32, i32),
        #[case] lower_1: (i32, i32),
        #[case] upper_2: (i32, i32),
        #[case] lower_2: (i32, i32),
        #[case] expected_upper: (i32, i32),
        #[case] expected_lower: (i32, i32),
    ) {
        let segments = Segments::from_iter([
            Segment::new(upper_1, lower_1),
            Segment::new(upper_2, lower_2),
        ]);
        let actual = Segment::intersect(0, 1, &segments, 0);
        expect_eq!(
            actual.as_ref().map(Intersection::point1),
            Some(&expected_upper.into())
        );
        expect_eq!(
            actual.as_ref().map(Intersection::point2),
            Some(Some(&expected_lower.into()))
        );
        expect_eq!(
            actual.as_ref().map(Intersection::segments),
            Some([0.into(), 1.into()].as_slice())
        );
    }

    #[gtest]
    fn same() {
        let segments = Segments::from_iter([
            Segment::new((12, 0), (-12, 0)),
            Segment::new((12, 0), (-12, 0)),
        ]);
        let actual = Segment::intersect(0, 1, &segments, 0);

        expect_that!(
            actual,
            some(pat!(Intersection {
                typ: pat!(IntersectionType::Parallel {
                    line: pat!(Segment {
                        upper: eq(&(-12, 0).into()),
                        lower: eq(&(12, 0).into()),
                        ..
                    })
                }),
                segments: eq(&InterVec::from_iter([0.into(), 1.into()])),
                step: eq(&0)
            }))
        );
    }

    #[gtest]
    fn endpoint() {
        let segments = Segments::from_iter([
            Segment::new((12, 0), (-12, 0)),
            Segment::new((12, 0), (24, 0)),
        ]);
        let actual = Segment::intersect(0, 1, &segments, 0);

        expect_eq!(
            actual,
            Some(Intersection::new(
                IntersectionType::Point {
                    coord: (12, 0).into()
                },
                smallvec::smallvec![0.into(), 1.into()],
                0
            ))
        );
    }

    #[gtest]
    fn parallel_not_intersection() {
        let segments = Segments::from_iter([
            Segment::new((12, 0), (-12, 0)),
            Segment::new((12, 1), (-12, 1)),
        ]);
        let actual = Segment::intersect(0, 1, &segments, 0);

        expect_eq!(actual, None);
    }
}

mod contains {
    use common::segment::Segment;
    use googletest::prelude::*;

    #[gtest]
    fn not() {
        let s1 = Segment::new((12, 0), (-12, 0));

        expect_false!(s1.contains(&(0, 1).into()));
    }
    #[gtest]
    fn endpoint() {
        let s1 = Segment::new((12, 0), (-12, 0));

        expect_true!(s1.contains(&(12, 0).into()));
        expect_true!(s1.contains(&(-12, 0).into()));
    }

    #[gtest]
    fn middle() {
        let s1 = Segment::new((12, 0), (-12, 0));

        expect_true!(s1.contains(&(0.0 + f64::MIN_POSITIVE, 0).into()));
    }
}

#[gtest]
fn slope() {
    let s1 = Segment::new((-1, 0), (1, 0));
    dbg!(s1.line());
    let s2 = Segment::new((-1000, 1), (0, 0));
    dbg!(s2.line());
    expect_gt!(s1.slope(), &s2.slope());
    let s3 = Segment::new((-100, 1), (0, 0));
    dbg!(s3.line());
    expect_gt!(s2.slope(), &s3.slope());
    let s4 = Segment::new((-10, 1), (0, 0));
    dbg!(s4.line());
    expect_gt!(s3.slope(), &s4.slope());
    let s5 = Segment::new((0, 1), (0, 0));
    dbg!(s5.line());
    expect_gt!(s4.slope(), &s5.slope());
    let s6 = Segment::new((10, 1), (0, 0));
    dbg!(s6.line());
    expect_gt!(s5.slope(), &s6.slope());
    let s7 = Segment::new((100, 1), (0, 0));
    dbg!(s7.line());
    expect_gt!(s6.slope(), &s7.slope());
    let s8 = Segment::new((1000, 1), (0, 0));
    dbg!(s8.line());
    expect_gt!(s7.slope(), &s8.slope());
}
