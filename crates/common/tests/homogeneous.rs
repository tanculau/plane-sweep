mod point {
    use common::math::homogeneous::*;
    use googletest::prelude::*;

    #[gtest]
    fn equal() {
        fn check(a: usize, b: usize, c: usize) {
            let p1 = HomogeneousCoord::new(a, b, c);
            let p2 = HomogeneousCoord::new(a * 2, b * 2, c * 2);
            expect_eq!(p1, p2);
        }
        let p1 = HomogeneousCoord::new(4, 2, 1);
        expect_eq!(p1, p1);
        check(4, 2, 1);
        check(0, 2, 1);
        check(4, 0, 1);
        check(4, 2, 0);
        check(0, 0, 1);
        check(0, 2, 0);
        check(4, 0, 0);
        check(0, 0, 0);
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
        let p1 = HomogeneousCoord::new(4, 2, 1);
        let p2 = HomogeneousCoord::new(4, 2, 2);
        expect_ne!(p1, p2);
        let p3 = HomogeneousCoord::new(4, 2, 1);
        let p4 = HomogeneousCoord::new(4, 2, 0);
        expect_ne!(p3, p4);
        let p5 = HomogeneousCoord::new(4, 2, 1);
        let p6 = HomogeneousCoord::new(4, 0, 1);
        expect_ne!(p5, p6);
        let p7 = HomogeneousCoord::new(4, 0, 1);
        let p8 = HomogeneousCoord::new(4, 0, 0);
        expect_ne!(p7, p8);

        let p9 = HomogeneousCoord::new(0, 0, 0);
        let p10 = HomogeneousCoord::new(4, 2, 0);
        expect_ne!(p9, p10);

        let p11 = HomogeneousCoord::new(4, 2, 1);
        let p12 = HomogeneousCoord::new(0, 0, 0);
        expect_ne!(p11, p12);
    }
}

mod line {

    use common::math::homogeneous::HomogeneousLine;
    use googletest::prelude::*;

    mod intersection {
        use common::math::{cartesian::CartesianCoord, homogeneous::*};
        use googletest::prelude::*;

        #[gtest]
        fn origin() {
            let x = HomogeneousLine::x_axis();
            let y = HomogeneousLine::y_axis();
            let intersection = x.intersection(y);
            let expected = HomogeneousCoord::new(0, 0, 1);
            expect_eq!(intersection, expected);
            let x = HomogeneousLine::x_axis();
            let y = HomogeneousLine::new(1, 1, 0);
            let intersection = x.intersection(y);
            let expected = HomogeneousCoord::new(0, 0, 1);
            expect_eq!(intersection, expected);
        }

        #[gtest]
        fn simple() {
            let x = HomogeneousLine::x_axis();
            let y = HomogeneousLine::new(1, 0, -5);
            let intersection = x.intersection(y).cartesian();
            let expected = CartesianCoord::new(5, 0);
            expect_that!(intersection, ok(eq(&expected)));
        }

        #[gtest]
        fn does_not_intersect() {
            let x = HomogeneousLine::y_axis();
            let y = HomogeneousLine::vertical(12);
            let intersect = x.intersection(y);
            expect_that!(intersect, field!(HomogeneousCoord.z, eq(&0.into())));
        }
    }

    mod contains {
        use common::math::homogeneous::*;
        use googletest::prelude::*;
        #[gtest]
        fn contains() {
            let x = HomogeneousLine::x_axis();
            expect_true!(x.contains_coord((5, 0)));
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
            expect_eq!(HomogeneousLine::x_axis().slope(), Slope::Horizontal);
            expect_eq!(HomogeneousLine::horizontal(-12).slope(), Slope::Horizontal);
            expect_eq!(HomogeneousLine::horizontal(12).slope(), Slope::Horizontal);
            expect_eq!(HomogeneousLine::horizontal(0).slope(), Slope::Horizontal);
        }

        #[gtest]
        fn vertical() {
            expect_eq!(HomogeneousLine::y_axis().slope(), Slope::Vertical);
            expect_eq!(HomogeneousLine::vertical(-12).slope(), Slope::Vertical);
            expect_eq!(HomogeneousLine::vertical(12).slope(), Slope::Vertical);
            expect_eq!(HomogeneousLine::vertical(0).slope(), Slope::Vertical);
        }
    }

    #[gtest]
    fn neg() {
        let l1 = HomogeneousLine::new(2, -3, 4);
        expect_that!(
            -l1.clone(),
            pat!(HomogeneousLine {
                a: eq(&(-2).into()),
                b: eq(&3.into()),
                c: eq(&(-4).into())
            })
        );
        expect_eq!(-l1.clone(), l1);
    }
}
