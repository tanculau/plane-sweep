mod point {
    use common::math::homogeneous::*;
    use float_cmp::approx_eq;
    use googletest::prelude::*;

    #[gtest]
    fn equal() {
        fn check(a: f64, b: f64, c: f64) {
            let p1 = HomogeneousCoord::new(a, b, c);
            let p2 = HomogeneousCoord::new(a * 2.0, b * 2.0, c * 2.0);
            expect_true!(approx_eq!(HomogeneousCoord, p1, p2));
        }
        let p1 = HomogeneousCoord::new(4.0, 2.0, 1.0);
        expect_true!(approx_eq!(HomogeneousCoord, p1, p1));
        check(4.0, 2.0, 1.0);
        check(0.0, 2.0, 1.0);
        check(4.0, 0.0, 1.0);
        check(4.0, 2.0, 0.0);
        check(0.0, 0.0, 1.0);
        check(0.0, 2.0, 0.0);
        check(4.0, 0.0, 0.0);
        check(0.0, 0.0, 0.0);
    }

    #[gtest]
    fn not_equal() {
        let p1 = HomogeneousCoord::new(4.0, 2.0, 1.0);
        let p2 = HomogeneousCoord::new(4.0, 2.0, 2.0);
        expect_false!(approx_eq!(HomogeneousCoord, p1, p2));
        let p3 = HomogeneousCoord::new(4.0, 2.0, 1.0);
        let p4 = HomogeneousCoord::new(4.0, 2.0, 0.0);
        expect_false!(approx_eq!(HomogeneousCoord, p3, p4));
        let p5 = HomogeneousCoord::new(4.0, 2.0, 1.0);
        let p6 = HomogeneousCoord::new(4.0, 0.0, 1.0);
        expect_false!(approx_eq!(HomogeneousCoord, p5, p6));
        let p7 = HomogeneousCoord::new(4.0, 0.0, 1.0);
        let p8 = HomogeneousCoord::new(4.0, 0.0, 0.0);
        expect_false!(approx_eq!(HomogeneousCoord, p7, p8));
        let p9 = HomogeneousCoord::new(0.0, 0.0, 0.0);
        let p10 = HomogeneousCoord::new(4.0, 2.0, 0.0);
        expect_false!(approx_eq!(HomogeneousCoord, p9, p10));
        let p11 = HomogeneousCoord::new(4.0, 2.0, 1.0);
        let p12 = HomogeneousCoord::new(0.0, 0.0, 0.0);
        expect_false!(approx_eq!(HomogeneousCoord, p11, p12));
    }
}

mod line {
    mod intersection {
        use common::{
            math::{cartesian::CartesianCoord, homogeneous::*},
            test::approx_eq,
        };
        use float_cmp::approx_eq;
        use googletest::prelude::*;

        #[gtest]
        fn origin() {
            let x = HomogeneousLine::X_AXIS;
            let y = HomogeneousLine::Y_AXIS;
            let intersection = x.intersection(y);
            let expected = HomogeneousCoord::new(0.0, 0.0, 1.0);
            expect_true!(approx_eq!(HomogeneousCoord, intersection, expected));
            let x = HomogeneousLine::X_AXIS;
            let y = HomogeneousLine::new(1.0, 1.0, 0.0);
            let intersection = x.intersection(y);
            let expected = HomogeneousCoord::new(0.0, 0.0, 1.0);
            expect_true!(approx_eq!(HomogeneousCoord, intersection, expected));
        }

        #[gtest]
        fn simple() {
            let x = HomogeneousLine::X_AXIS;
            let y = HomogeneousLine::new(1, 0, -5);
            let intersection = x.intersection(y).cartesian();
            let expected = CartesianCoord::new(5.0, 0.0);
            expect_that!(intersection, ok(approx_eq(expected)));
        }
    }

    mod contains {
        use common::math::homogeneous::*;
        use googletest::prelude::*;
        #[gtest]
        fn contains() {
            let x = HomogeneousLine::X_AXIS;
            expect_true!(x.contains_coord((5, 0)));
        }
    }

    mod intersects {
        use common::{
            math::{cartesian::CartesianCoord, homogeneous::*},
            segment::Segment,
        };
        use googletest::prelude::*;

        #[gtest]
        fn intersects() {
            use float_cmp::approx_eq;

            let x = HomogeneousLine::X_AXIS;
            let y = HomogeneousLine::Y_AXIS;
            expect_true!(approx_eq!(
                HomogeneousCoord,
                x.intersection(y),
                HomogeneousCoord::new(0.0, 0.0, 1.0)
            ));
        }

        #[gtest]
        fn does_not_intersect() {
            use common::test::approx_eq;
            let x = HomogeneousLine::Y_AXIS;
            let y = HomogeneousLine::vertical(12);
            let intersect = x.intersection(y);
            expect_that!(intersect, field!(HomogeneousCoord.z, approx_eq(0.0)));
        }

        #[gtest]
        fn endpoint() {
            use common::test::approx_eq;

            let segment = Segment::new((-2, 2), (2, -2));
            let p_y = 2;
            let horizontal = HomogeneousLine::horizontal(p_y);
            let seg = dbg!(segment.line());
            let x = horizontal.intersection(seg).cartesian();
            assert_that!(x, ok(approx_eq(CartesianCoord::new(-2, 2))));

            let segment = Segment::new((2, 2), (-2, -2));
            let p_y = 2;
            let horizontal = HomogeneousLine::horizontal(p_y);
            let seg = dbg!(segment.line());
            let x = horizontal.intersection(seg).cartesian();
            expect_that!(x, ok(approx_eq(CartesianCoord::new(2, 2))));
        }
    }

    mod angle {
        use common::segment::Segment;
        use googletest::prelude::*;

        #[gtest]
        fn simple() {
            let horizontal = Segment::new((0, 0), (2, 0));
            let down_right = Segment::new((0, 0), (2, -2));
            expect_le!(down_right.angle(), horizontal.angle());
            let vertical = Segment::new((0, 2), (0, 0));
            expect_le!(vertical.angle(), down_right.angle());
            let down_left = Segment::new((0, 0), (-2, -2));
            expect_le!(down_left.angle(), vertical.angle());
        }
    }
}
