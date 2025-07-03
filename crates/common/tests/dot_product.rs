use common::math::DotProduct;
use googletest::prelude::*;

use rstest::rstest;
use static_assertions::assert_impl_all;

assert_impl_all!((u32, u32, u32): DotProduct<(u32, u32, u32), Output = u32>);
assert_impl_all!((u32, u32, u32): DotProduct<(&'static u32, &'static u32, &'static u32)>);
assert_impl_all!((f32, f32, f32): DotProduct<(f32, f32, f32), Output = f32>);
assert_impl_all!((f32, f32, f32): DotProduct<(&'static f32, &'static f32, &'static f32)>);
assert_impl_all!((f64, f64, f64): DotProduct<(f64, f64, f64)>);
assert_impl_all!((f64, f64, f64): DotProduct<(&'static f64, &'static f64, &'static f64), Output = f64>);

#[gtest]
#[rstest]
#[case::zero((0,0,0), (0,0,0), 0)]
#[case::positive_values((1,2,3), (4,5,6), 4 + 2*5 + 3*6)]
#[case::negative_values((-1, -2, -3), (-4, -5, -6), (-1)*(-4) + (-2)*(-5) + (-3)*(-6))]
#[case::mixed_signs((-1, 2, -3), (4, -5, 6), -4 + 2*(-5) + -3*6)]
#[case::orthogonal_vectors((1, 0, 0), (0, 1, 0), 0)]
#[case::parallel_vectors((2, 2, 2), (3, 3, 3), 2*3 + 2*3 + 2*3)]
#[case::anti_parallel_vectors((1, 1, 1), (-1, -1, -1), -3)]
#[case::unit_vectors((1, 0, 0),(1, 0, 0), 1)]
#[case::unit_vectors((0, 1, 0),(0, 1, 0), 1)]
#[case::unit_vectors((0, 0, 1),(0, 0, 1), 1)]
#[case::large_values((1000, 2000, 3000),(4000, 5000, 6000), 1000*4000 + 2000*5000 + 3000*6000)]
#[case::unit_vectors((0, 2, 0),(1, 0, 3), 0)]
fn dot_product(#[case] lhs: (i32, i32, i32), #[case] rhs: (i32, i32, i32), #[case] expected: i32) {
    expect_eq!(lhs.dot_product(rhs), expected);
}
