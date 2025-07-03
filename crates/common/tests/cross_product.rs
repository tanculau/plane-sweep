use common::math::CrossProduct;
use googletest::prelude::*;

use rstest::rstest;
use static_assertions::assert_impl_all;

assert_impl_all!((u32, u32, u32): CrossProduct<(u32, u32, u32)>);
assert_impl_all!((u32, u32, u32): CrossProduct<(&'static u32, &'static u32, &'static u32)>);
assert_impl_all!((f32, f32, f32): CrossProduct<(f32, f32, f32)>);
assert_impl_all!((f32, f32, f32): CrossProduct<(&'static f32, &'static f32, &'static f32)>);
assert_impl_all!((f64, f64, f64): CrossProduct<(f64, f64, f64)>);
assert_impl_all!((f64, f64, f64): CrossProduct<(&'static f64, &'static f64, &'static f64)>);

#[gtest]
#[rstest]
#[case::zero((0,0,0), (0,0,0), (0,0,0))]
#[case::xy((1,0,0), (0,1,0), (0,0,1))]
#[case::y_x((0,1,0), (1,0,0), (0,0,-1))]
#[case::y_z((0,1,0), (0,0,1), (1,0,0))]
#[case::parallel((1, 1, 1), (2, 2, 2), (0,0,0))]
#[case::opposite((1, 0,0), (-1,0,0), (0,0,0))]
#[case::negative_values((2, -3, 4), (-1, 5, -2), (-14, 0, 7))]
#[case::arbitrary_vectors((3, -3, 1), (4, 9, 2), (-15, -2, 39))]

fn cross_product(
    #[case] lhs: (i32, i32, i32),
    #[case] rhs: (i32, i32, i32),
    #[case] expected: (i32, i32, i32),
) {
    assert_eq!(
        lhs.cross_product(rhs),
        expected,
        "{lhs:?} cross product {rhs:?}"
    );
}
