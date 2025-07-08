use common::{intersection::Intersections, segment::{Segment, Segments}};
use fastrand::Rng;
use sweep_fast::calculate;


fn main() {
    let (segments, mut intersections) = gen_inputs(128)();
    calculate(&segments, &mut intersections);
    println!("{}", intersections.len());

}


fn gen_inputs(len: usize) -> impl FnMut() -> (Segments, Intersections) {
    let mut rng = Rng::with_seed(len as u64);

    move || {
        (
            std::iter::from_fn(|| {
                Some(Segment::new(
                    (rng.i16(..), rng.i16(..)),
                    (rng.i16(..), rng.i16(..)),
                ))
            })
            .take(len)
            .collect(),
            Intersections::with_capacity(len * len),
        )
    }
}
