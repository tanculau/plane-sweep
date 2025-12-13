use common::{intersection::Intersections, math::Rational, segment::{Segment, Segments}};
use fastrand::Rng;
use sweep_fast::calculate;


fn main() {
    let (segments, mut intersections) = gen_inputs(128)();
    calculate(&segments, &mut intersections);
    println!("{}", intersections.len());

}


fn gen_inputs(len: usize) -> impl FnMut() -> (Segments<Rational>, Intersections<Rational>) {
    let mut rng = Rng::with_seed(len as u64);

    move || {
        (
            std::iter::from_fn(|| {
                Some(Segment::new(
                    (rng.i64(..), rng.i64(..)),
                    (rng.i64(..), rng.i64(..)),
                ))
            })
            .take(len)
            .collect(),
            Intersections::with_capacity(len * len),
        )
    }
}
