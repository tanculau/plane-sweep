use common::math::CrossProduct;
use googletest::prelude::*;

use static_assertions::assert_impl_all;

assert_impl_all!((u32, u32, u32): CrossProduct<(u32, u32, u32)>);
assert_impl_all!((u32, u32, u32): CrossProduct<(&'static u32, &'static u32, &'static u32)>);
assert_impl_all!((f32, f32, f32): CrossProduct<(f32, f32, f32)>);
assert_impl_all!((f32, f32, f32): CrossProduct<(&'static f32, &'static f32, &'static f32)>);
assert_impl_all!((f64, f64, f64): CrossProduct<(f64, f64, f64)>);
assert_impl_all!((f64, f64, f64): CrossProduct<(&'static f64, &'static f64, &'static f64)>);

#[gtest]
fn zero() {
    let a = (0, 0, 0);
    let b = (0, 0, 0);
    let c = a.cross_product(b);
    expect_that!(c, eq((0, 0, 0)));
}
#[gtest]
fn x_y() {
    let a = (1, 0, 0); // x-axis
    let b = (0, 1, 0); // y-axis
    let c = a.cross_product(b);
    expect_that!(c, eq((0, 0, 1))); // x × y = z
}

#[gtest]
fn y_x() {
    let a = (0, 1, 0); // y-axis
    let b = (1, 0, 0); // x-axis
    let c = a.cross_product(b);
    expect_that!(c, eq((0, 0, -1))); // y × x = -z
}

#[gtest]
fn y_z() {
    let a = (0, 1, 0); // y-axis
    let b = (0, 0, 1); // z-axis
    let c = a.cross_product(b);
    expect_that!(c, eq((1, 0, 0))); // y × z = x
}

#[gtest]
fn parallel() {
    let a = (1, 1, 1);
    let b = (2, 2, 2);
    let c = a.cross_product(b);
    expect_that!(c, eq((0, 0, 0))); // Parallel vectors have zero cross product
}

#[gtest]
fn opposite() {
    let a = (1, 0, 0);
    let b = (-1, 0, 0);
    let c = a.cross_product(b);
    expect_that!(c, eq((0, 0, 0))); // Opposite vectors, but still collinear
}

#[gtest]
fn negative_values() {
    let a = (2, -3, 4);
    let b = (-1, 5, -2);
    let c = a.cross_product(b);
    expect_that!(c, eq((-14, 0, 7)));
}

#[gtest]
fn arbitrary_vectors() {
    let a = (3, -3, 1);
    let b = (4, 9, 2);
    let c = a.cross_product(b);
    expect_that!(c, eq((-15, -2, 39)));
}
