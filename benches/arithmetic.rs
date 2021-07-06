use criterion::{Criterion, black_box, criterion_group, criterion_main};
use rust_arithmetic_benchmark::num_field::*;
use rust_arithmetic_benchmark::ramp_field::RampField;
use criterion::SamplingMode::*;

const BENCH_SIZE: usize = 5_000_000;
const SAMPLE_SIZE: usize = 20;

fn addition(c: &mut Criterion) {
    let mut group = c.benchmark_group("Mass Addition");
    group.sample_size(SAMPLE_SIZE);

    group.bench_function("ramp", |bench| {
        bench.iter(|| {
            let n = RampField::from(21);
            let mut result = n.clone();
            for _ in 0..BENCH_SIZE {
                result = result.clone() + result;
            }
            black_box(result)
        })
    });

    group.bench_function("num", |bench| {
        bench.iter(|| {
            let n = NumField::from(21);
            let mut result = n.clone();
            for _ in 0..BENCH_SIZE {
                result = result.clone() + result;
            }
            black_box(result)
        })
    });

    group.finish();
}

fn multiplication(c: &mut Criterion) {
    let mut group = c.benchmark_group("Mass Multiplication");
    group.sample_size(SAMPLE_SIZE);

    group.bench_function("ramp", |bench| {
        bench.iter(|| {
            let n = RampField::from(21);
            let mut result = n.clone();
            for _ in 0..BENCH_SIZE {
                result = result.clone() * result;
            }
            black_box(result)
        })
    });

    group.bench_function("num", |bench| {
        bench.iter(|| {
            let n = NumField::from(21);
            let mut result = n.clone();
            for _ in 0..BENCH_SIZE {
                result = result.clone() * result;
            }
            black_box(result)
        })
    });

    group.finish();
}

criterion_group!(arithmetic, addition, multiplication);
criterion_main!(arithmetic);
