mod coord {
    mod distance {
        use common::math::{Distance, cartesian::CartesianCoord};
        use googletest::prelude::*;

        #[gtest]
        fn same_point() {
            let a = CartesianCoord::new(0, 0);
            let b = CartesianCoord::new(0, 0);

            expect_near!(a.distance(b), 0.0, 1e-6);
        }

        #[gtest]
        fn horizontal() {
            let a = CartesianCoord::new(0, 0);
            let b = CartesianCoord::new(3, 0);

            expect_near!(a.distance(b), 3.0, 1e-6);
        }

        #[gtest]
        fn vertical() {
            let a = CartesianCoord::new(0, 0);
            let b = CartesianCoord::new(4, 0);

            expect_near!(a.distance(b), 4.0, 1e-6);
        }

        #[gtest]
        fn diagonal() {
            let a = CartesianCoord::new(0, 0);
            let b = CartesianCoord::new(2, 2);

            expect_near!(a.distance(b), 8.0_f64.sqrt(), 1e-6);
        }

        #[gtest]
        fn negative() {
            let a = CartesianCoord::new(-2, -3);
            let b = CartesianCoord::new(1, 5);
            expect_near!(a.distance(b), 73.0_f64.sqrt(), 1e-6);
        }
    }

    mod approxeq {
        use common::math::cartesian::CartesianCoord;
        use googletest::prelude::*;

        use float_cmp::approx_eq;

        #[gtest]
        fn equal() {
            let a = CartesianCoord::new(0.15 + 0.15 + 0.15, 0.15 + 0.15 + 0.15);
            let b = CartesianCoord::new(0.15 + 0.15 + 0.15, 0.15 + 0.15 + 0.15);
            expect_true!(approx_eq!(CartesianCoord, a, b));
        }
        #[gtest]
        fn approx() {
            let a = CartesianCoord::new(0.1 + 0.1 + 0.25, 0.1 + 0.1 + 0.25);
            let b = CartesianCoord::new(0.15 + 0.15 + 0.15, 0.15 + 0.15 + 0.15);
            expect_ne!(0.15_f64 + 0.15 + 0.15, 0.1_f64 + 0.1 + 0.25);
            expect_true!(approx_eq!(CartesianCoord, a, b));
        }
        #[gtest]
        fn not() {
            let a = CartesianCoord::new(0.1 + 0.1 + 0.251, 0.1 + 0.1 + 0.25);
            let b = CartesianCoord::new(0.15 + 0.15 + 0.15, 0.15 + 0.15 + 0.15);
            expect_ne!(0.15 + 0.15 + 0.15, 0.1 + 0.1 + 0.25);
            expect_false!(approx_eq!(CartesianCoord, a, b));
        }
    }
}
