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
    fn cartesian() {
        expect_eq!(
            HomogeneousCoord::new(12, 6, 1).cartesian(),
            Ok((12, 6).into())
        );
        expect_eq!(
            HomogeneousCoord::new(24, 12, 2).cartesian(),
            Ok((12, 6).into())
        );
        expect_eq!(
            HomogeneousCoord::new(-24, -12, -2).cartesian(),
            Ok((12, 6).into())
        );
        expect_eq!(
            HomogeneousCoord::new(24, -12, -2).cartesian(),
            Ok((-12, 6).into())
        );
        expect_eq!(
            HomogeneousCoord::new(0, -12, -2).cartesian(),
            Ok((0, 6).into())
        );
        expect_eq!(
            HomogeneousCoord::new(24, -12, 0).cartesian(),
            Err(PointAtInfinity)
        );
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
    use core::f64::consts::PI;

    use common::math::homogeneous::HomogeneousLine;
    use googletest::prelude::*;

    #[cfg(feature = "test")]
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

        #[gtest]
        fn does_not_intersect() {
            use common::test::approx_eq;
            let x = HomogeneousLine::Y_AXIS;
            let y = HomogeneousLine::vertical(12);
            let intersect = x.intersection(y);
            expect_that!(intersect, field!(HomogeneousCoord.z, approx_eq(0.0)));
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

    #[gtest]
    fn equal() {
        let l1 = HomogeneousLine::new(2, 3, 4);
        let l2 = HomogeneousLine::new(4, 6, 8);
        expect_eq!(l1, l2);
        expect_eq!(l1, l1);
        let l3 = HomogeneousLine::new(2, 3, 0);
        let l4 = HomogeneousLine::new(4, 6, 0);
        expect_eq!(l3, l4);
        expect_eq!(l3, l3);
        let l5 = HomogeneousLine::new(2, 0, 4);
        let l6 = HomogeneousLine::new(4, 0, 8);
        expect_eq!(l5, l6);
        expect_eq!(l5, l5);
        let l7 = HomogeneousLine::new(0, 3, 4);
        let l8 = HomogeneousLine::new(0, 6, 8);
        expect_eq!(l7, l8);
        expect_eq!(l7, l7);
        let l9 = HomogeneousLine::new(0, 0, 4);
        let l10 = HomogeneousLine::new(0, 0, 8);
        expect_eq!(l9, l10);
        expect_eq!(l9, l9);
        let l11 = HomogeneousLine::new(0, 3, 0);
        let l12 = HomogeneousLine::new(0, 6, 0);
        expect_eq!(l11, l12);
        expect_eq!(l11, l11);
        let l13 = HomogeneousLine::new(5, 0, 0);
        let l14 = HomogeneousLine::new(15, 0, 0);
        expect_eq!(l13, l14);
        expect_eq!(l13, l13);
        let l15 = HomogeneousLine::new(0, 0, 0);
        let l16 = HomogeneousLine::new(0, 0, 0);
        expect_eq!(l15, l16);
        expect_eq!(l15, l15);
        let l17 = HomogeneousLine::new(2, 3, 4);
        let l18 = HomogeneousLine::new(0, 0, 0);
        expect_ne!(l17, l18);
        let l19 = HomogeneousLine::new(2, 3, 4);
        let l20 = HomogeneousLine::new(2, 3, 0);
        expect_ne!(l19, l20);
        let l21 = HomogeneousLine::new(2, 3, 4);
        let l22 = HomogeneousLine::new(2, 0, 4);
        expect_ne!(l21, l22);
        let l23 = HomogeneousLine::new(2, 3, 4);
        let l24 = HomogeneousLine::new(0, 3, 4);
        expect_ne!(l23, l24);
        let l24 = HomogeneousLine::new(2, 3, 4);
        let l25 = HomogeneousLine::new(0, 0, 4);
        expect_ne!(l24, l25);
    }

    mod slope {
        use common::math::homogeneous::{HomogeneousLine, Slope};
        use googletest::prelude::*;

        #[gtest]
        fn infinite() {
            let l1 = HomogeneousLine::new(0, 0, 12);
            expect_eq!(l1.slope(), Slope::Infinity);
            let l1 = HomogeneousLine::new(0, 0, -12);
            expect_eq!(l1.slope(), Slope::Infinity);
            let l1 = HomogeneousLine::new(0, 0, 0);
            expect_eq!(l1.slope(), Slope::Infinity);
        }

        #[gtest]
        fn horizontal() {
            expect_eq!(HomogeneousLine::X_AXIS.slope(), Slope::Horizontal);
            expect_eq!(HomogeneousLine::horizontal(-12).slope(), Slope::Horizontal);
            expect_eq!(HomogeneousLine::horizontal(12).slope(), Slope::Horizontal);
            expect_eq!(HomogeneousLine::horizontal(0).slope(), Slope::Horizontal);
        }

        #[gtest]
        fn vertical() {
            expect_eq!(HomogeneousLine::Y_AXIS.slope(), Slope::Vertical);
            expect_eq!(HomogeneousLine::vertical(-12).slope(), Slope::Vertical);
            expect_eq!(HomogeneousLine::vertical(12).slope(), Slope::Vertical);
            expect_eq!(HomogeneousLine::vertical(0).slope(), Slope::Vertical);
        }

        #[gtest]
        fn value() {
            expect_eq!(
                HomogeneousLine::new(12, 6, 0).slope(),
                Slope::FourthQuadrant(0.5.into())
            );
            expect_eq!(
                HomogeneousLine::new(-12, 6, 0).slope(),
                Slope::ThirdQuadrant(0.5.into())
            );
            expect_eq!(
                HomogeneousLine::new(12, -6, 0).slope(),
                Slope::ThirdQuadrant(0.5.into())
            );
            expect_eq!(
                HomogeneousLine::new(-12, -6, 0).slope(),
                Slope::FourthQuadrant(0.5.into())
            );
        }
    }

    #[gtest]
    fn angle() {
        expect_float_eq!(HomogeneousLine::new(0, 0, 0).angle(), 0.0);
        expect_float_eq!(HomogeneousLine::new(0, 0, 12).angle(), 0.0);
        expect_float_eq!(HomogeneousLine::new(0, 0, -12).angle(), 0.0);
        expect_float_eq!(HomogeneousLine::new(0, 0, 0).angle(), 0.0);
        expect_float_eq!(HomogeneousLine::new(0, 0, 12).angle(), 0.0);
        expect_float_eq!(HomogeneousLine::new(0, 0, -12).angle(), 0.0);
        expect_ge!(HomogeneousLine::new(1, -1, 0).angle(), -PI / 2.0);
        expect_le!(HomogeneousLine::new(1, -1, 0).angle(), PI / 2.0);
        expect_ge!(HomogeneousLine::new(2, -2, 0).angle(), -PI / 2.0);
        expect_le!(HomogeneousLine::new(2, -2, 0).angle(), PI / 2.0);
        expect_ge!(HomogeneousLine::new(-2, 2, 0).angle(), -PI / 2.0);
        expect_le!(HomogeneousLine::new(-2, 2, 0).angle(), PI / 2.0);

        expect_lt!(
            HomogeneousLine::new(-1, 1000, 0).angle(),
            HomogeneousLine::new(-1, 900, 0).angle()
        );
        expect_lt!(
            (-HomogeneousLine::new(-1, 1000, 0)).angle(),
            HomogeneousLine::new(-1, 900, 0).angle()
        );
        expect_lt!(
            HomogeneousLine::new(-1, 1000, 0).angle(),
            (-HomogeneousLine::new(-1, 900, 0)).angle()
        );
        expect_lt!(
            (-HomogeneousLine::new(-1, 1000, 0)).angle(),
            (-HomogeneousLine::new(-1, 900, 0)).angle()
        );
        expect_lt!(
            HomogeneousLine::new(-1, 900, 0).angle(),
            HomogeneousLine::Y_AXIS.angle()
        );
        expect_lt!(
            HomogeneousLine::Y_AXIS.angle(),
            HomogeneousLine::new(1, 1, 0).angle()
        );
        expect_lt!(
            HomogeneousLine::Y_AXIS.angle(),
            HomogeneousLine::new(1000, 1, 0).angle()
        );
        expect_lt!(
            HomogeneousLine::new(1000, 1, 0).angle(),
            HomogeneousLine::X_AXIS.angle()
        );
    }

    #[gtest]
    fn neg() {
        let l1 = HomogeneousLine::new(2, -3, 4);
        expect_that!(
            -l1,
            pat!(HomogeneousLine {
                a: eq(-2.0),
                b: eq(3.0),
                c: eq(-4.0)
            })
        );
        expect_eq!(-l1, l1);
    }
}
