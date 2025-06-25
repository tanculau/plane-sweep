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
