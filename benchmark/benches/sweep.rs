use sweep::calculate;
use common::{
    intersection::Intersections,
    segment::{Segment, Segments},
};
use divan::{AllocProfiler, Bencher};
use fastrand::Rng;

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

const SIZES: &[usize] = &[0, 1, 2, 8, 16, 64, 128, 256, 1024];

fn gen_inputs(len: usize) -> impl FnMut() -> (Segments, Intersections) {
    let mut rng = Rng::with_seed(len as u64);

    move || {
        (
            std::iter::from_fn(|| {
                Some(Segment::new(
                    (rng.i16(..), rng.i64(..)),
                    (rng.i16(..), rng.i64(..)),
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

#[divan::bench(args = SIZES, max_time = 10)]
fn sweep(bencher: Bencher, len: usize) {
    bencher
        .with_inputs(gen_inputs(len))
        .counter(divan::counter::ItemsCount::new(len))
        .bench_local_refs(|(segments, intersections)| {
            calculate(segments, intersections);
        });
}
