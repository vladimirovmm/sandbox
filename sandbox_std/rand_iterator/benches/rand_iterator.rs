//! Как запустить:
//! ```bash
//! cargo bench --manifest-path sandbox_std/Cargo.toml -- fib_20
//! ```

use std::ops::Range;

use criterion::{Criterion, criterion_group, criterion_main};
use rand_iterator::{function::*, rand_iter};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("rand_iterator");

    const INPUT_RANGE: Range<u64> = 0..1000;
    const COUNT: usize = 100;

    group.bench_with_input("rand_iter_v1", &(), |b, _| {
        let Range { start, end } = INPUT_RANGE;
        b.iter(|| {
            let _t: Vec<u64> = rand_iter_v1(start, end).take(COUNT).collect();
        });
    });
    group.bench_with_input("rand_iter_v2", &(), |b, _| {
        let Range { start, end } = INPUT_RANGE;
        b.iter(|| {
            let _t: Vec<u64> = rand_iter_v2(start, end).take(COUNT).collect();
        });
    });
    group.bench_with_input("rand_iter_v3", &(), |b, _| {
        b.iter(|| {
            let _t: Vec<u64> = rand_iter_v3(INPUT_RANGE).take(COUNT).collect();
        });
    });
    group.bench_with_input("rand_iter", &(), |b, _| {
        b.iter(|| {
            let _t: Vec<u64> = rand_iter(INPUT_RANGE).take(COUNT).collect();
        });
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
