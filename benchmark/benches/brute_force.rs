use brute_force::calculate;
use common::{
    intersection::Intersections,
    segment::{Segment, Segments},
};
use divan::{AllocProfiler, Bencher};
use fastrand::Rng;
use malachite::rational::Rational;

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

const SIZES: &[usize] = &[0, 1, 2, 4, 8, 16, 32, 64, 128, 256, 512];
const SIZE : &[usize] = &[16,32,48,64,80,96,112,128,144,160,176,192,208,224,240,256];

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

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench(args = SIZE)]
fn brute_force(bencher: Bencher, len: usize) {
    bencher
        .with_inputs(gen_inputs(len))
        .counter(divan::counter::ItemsCount::new(len))
        .bench_local_refs(|(segments, intersections)| {
            calculate(segments, intersections);
        });
}
