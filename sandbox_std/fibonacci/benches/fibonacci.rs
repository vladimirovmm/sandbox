//! Как запустить:
//! ```bash
//! cargo bench --manifest-path sandbox_std/Cargo.toml -- fib_20
//! ```

use criterion::{criterion_group, criterion_main, Criterion};
use fibonacci::{
    cycle::{cycle_for_v1, cycle_for_v2, cycle_for_v3, cycle_for_v4, cycle_for_v5, fast_cycle_for},
    iterator::iterator_fold,
    recursion::*,
};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("fibonacci");
    for i in &[20, 50] {
        group.bench_with_input(format!("slow_rec_{i}"), i, |b, i| {
            b.iter(|| slow_recursion(*i));
        });
        group.bench_with_input(format!("recursion_v2_{i}"), i, |b, i| {
            b.iter(|| recursion_v2(*i));
        });
        group.bench_with_input(format!("recursion_v3_{i}"), i, |b, i| {
            b.iter(|| recursion_v3(*i));
        });
        group.bench_with_input(format!("fast_recursion_{i}"), i, |b, i| {
            b.iter(|| fast_recursion(*i));
        });
        group.bench_with_input(format!("iterator_fold_{i}"), i, |b, i| {
            b.iter(|| iterator_fold(*i));
        });
        group.bench_with_input(format!("cycle_for_v1_{i}"), i, |b, i| {
            b.iter(|| cycle_for_v1(*i));
        });
        group.bench_with_input(format!("cycle_for_v2_{i}"), i, |b, i| {
            b.iter(|| cycle_for_v2(*i));
        });
        group.bench_with_input(format!("cycle_for_v3_{i}"), i, |b, i| {
            b.iter(|| cycle_for_v3(*i));
        });
        group.bench_with_input(format!("cycle_for_v4_{i}"), i, |b, i| {
            b.iter(|| cycle_for_v4(*i));
        });
        group.bench_with_input(format!("cycle_for_v5_{i}"), i, |b, i| {
            b.iter(|| cycle_for_v5(*i));
        });
        group.bench_with_input(format!("fast_cycle_for_{i}"), i, |b, i| {
            b.iter(|| fast_cycle_for(*i));
        });
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
