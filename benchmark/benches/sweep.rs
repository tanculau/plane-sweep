use common::{
    intersection::Intersections,
    math::Rational,
    segment::{Segment, Segments},
};
use divan::{AllocProfiler, Bencher};
use fastrand::Rng;
use sweep_fast::calculate;

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

const SIZES: &[usize] = &[0, 1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 768, 1024];
const SIZES_SMALL: &[usize] = &[0, 1, 2, 4, 8, 16, 32, 48, 64, 72, 128, 192, 256];
const SIZE : &[usize] = &[16,32,48,64,80,96,112,128,144,160,176,192,208,224,240,256];


#[test]
fn feature() {
    for i in 0..12 {
        println!("{}", 16*i)
    }
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

fn gen_optimal_inputs(len: usize) -> impl FnMut() -> (Segments<Rational>, Intersections<Rational>) {
    move || {
        let mut segment = Segments::new();

        for i in 0..len {
            segment.push(Segment::new((0, 3 * i), (0, 3 * i + 1)));
        }

        (segment, Intersections::new())
    }
}

fn gen_optimal_horizontal_inputs(len: usize) -> impl FnMut() -> (Segments<Rational>, Intersections<Rational>) {
    move || {
        let mut segment = Segments::new();

        for i in 0..len {
            segment.push(Segment::new((3 * i, 0), (3 * i + 1, 0)));
        }

        (segment, Intersections::new())
    }
}


fn gen_parallel_inputs(len: usize) -> impl FnMut() -> (Segments<Rational>, Intersections<Rational>) {
    move || {
        let mut segment = Segments::new();

        for i in 0..len {
            segment.push(Segment::new(( i, 0), (i + 5, 4)));
        }

        (segment, Intersections::new())
    }
}

fn gen_50_inputs(len: usize) -> impl FnMut() -> (Segments<Rational>, Intersections<Rational>) {
    move || {
        let mut segment = Segments::new();

        for i in (0..len).step_by(2) {
            segment.push(Segment::new((i, i), (i - 1 , i - 1)));
            segment.push(Segment::new((i - 1, i), (i  , i - 1)));

        }

        (segment, Intersections::new())
    }
}


fn gen_worst_case(len: usize) -> impl FnMut() -> (Segments<Rational>, Intersections<Rational>) {
    move || {
        let mut segment = Segments::new();

        for i in (0..len) {
            let m = Rational::from(2 * len + 1);
            let i = Rational::from(i);
            segment.push(Segment::new((-m.clone(), i.clone() * (-m.clone()) + i.clone() * i.clone()), (m.clone(), i.clone() * m .clone()+ i.clone()*i.clone())));

        }

        (segment, Intersections::new())
    }
}


fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench(args = SIZES_SMALL, ignore)]
fn sweep(bencher: Bencher, len: usize) {
    bencher
        .with_inputs(gen_inputs(len))
        .counter(divan::counter::ItemsCount::new(len))
        .bench_local_refs(|(segments, intersections)| {
            calculate(segments, intersections);
        });
}

#[divan::bench(args = SIZES, ignore)]
fn sweep_optimal(bencher: Bencher, len: usize) {
    bencher
        .with_inputs(gen_optimal_inputs(len))
        .counter(divan::counter::ItemsCount::new(len))
        .bench_local_refs(|(segments, intersections)| {
            calculate(segments, intersections);
        });
}

#[divan::bench(args = SIZES, ignore)]
fn sweep_optimal_horizontal(bencher: Bencher, len: usize) {
    bencher
        .with_inputs(gen_optimal_horizontal_inputs(len))
        .counter(divan::counter::ItemsCount::new(len))
        .bench_local_refs(|(segments, intersections)| {
            calculate(segments, intersections);
        });
}



#[divan::bench(args = SIZES, ignore)]
fn sweep_parallel_horizontal(bencher: Bencher, len: usize) {
    bencher
        .with_inputs(gen_parallel_inputs(len))
        .counter(divan::counter::ItemsCount::new(len))
        .bench_local_refs(|(segments, intersections)| {
            calculate(segments, intersections);
        });
}

#[divan::bench(args = SIZE)]
fn sweep_worst(bencher: Bencher, len: usize) {
    bencher
        .with_inputs(gen_worst_case(len))
        .counter(divan::counter::ItemsCount::new(len))
        .bench_local_refs(|(segments, intersections)| {
            calculate(segments, intersections);
        });
}


