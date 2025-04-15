//! Как запустить:
//!
//! ```bash
//! cargo bench -p fibonacci
//! ```

use criterion::{criterion_group, criterion_main, Criterion};
use fibonacci::{
    cycle::{
        cycle_for_v1, cycle_for_v3, cycle_for_v4, cycle_for_v5, cycle_for_v6, cycle_loop,
        cycle_while, fast_cycle_for,
    },
    iterator::iterator_fold,
    recursion::*,
    safe_fibonacci_v1, safe_fibonacci_v2, safe_fibonacci_v3, safe_fibonacci_v4,
    MAX_FIBONACCI_FOR_U64,
};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("fibonacci");

    group.bench_with_input("slow_rec_20", &20, |b, i| {
        b.iter(|| slow_recursion(*i));
    });
    group.bench_with_input(
        format!("fast_recursion_{MAX_FIBONACCI_FOR_U64}"),
        &MAX_FIBONACCI_FOR_U64,
        |b, i| {
            b.iter(|| fast_recursion(*i));
        },
    );
    group.bench_with_input(
        format!("recursion_v3_{MAX_FIBONACCI_FOR_U64}"),
        &MAX_FIBONACCI_FOR_U64,
        |b, i| {
            b.iter(|| recursion_v3(*i));
        },
    );
    group.bench_with_input(
        format!("recursion_v4_{MAX_FIBONACCI_FOR_U64}"),
        &MAX_FIBONACCI_FOR_U64,
        |b, i| {
            b.iter(|| recursion_v4(*i));
        },
    );
    group.bench_with_input(
        format!("iterator_fold_{MAX_FIBONACCI_FOR_U64}"),
        &MAX_FIBONACCI_FOR_U64,
        |b, i| {
            b.iter(|| iterator_fold(*i));
        },
    );
    group.bench_with_input(
        format!("cycle_for_v1_{MAX_FIBONACCI_FOR_U64}"),
        &MAX_FIBONACCI_FOR_U64,
        |b, i| {
            b.iter(|| cycle_for_v1(*i));
        },
    );
    group.bench_with_input(
        format!("fast_cycle_for_{MAX_FIBONACCI_FOR_U64}"),
        &MAX_FIBONACCI_FOR_U64,
        |b, i| {
            b.iter(|| fast_cycle_for(*i));
        },
    );
    group.bench_with_input(
        format!("cycle_for_v3_{MAX_FIBONACCI_FOR_U64}"),
        &MAX_FIBONACCI_FOR_U64,
        |b, i| {
            b.iter(|| cycle_for_v3(*i));
        },
    );
    group.bench_with_input(
        format!("cycle_for_v4_{MAX_FIBONACCI_FOR_U64}"),
        &MAX_FIBONACCI_FOR_U64,
        |b, i| {
            b.iter(|| cycle_for_v4(*i));
        },
    );
    group.bench_with_input(
        format!("cycle_for_v5_{MAX_FIBONACCI_FOR_U64}"),
        &MAX_FIBONACCI_FOR_U64,
        |b, i| {
            b.iter(|| cycle_for_v5(*i));
        },
    );
    group.bench_with_input(
        format!("cycle_for_v6_{MAX_FIBONACCI_FOR_U64}"),
        &MAX_FIBONACCI_FOR_U64,
        |b, i| {
            b.iter(|| cycle_for_v6(*i));
        },
    );
    group.bench_with_input(
        format!("cycle_while_{MAX_FIBONACCI_FOR_U64}"),
        &MAX_FIBONACCI_FOR_U64,
        |b, i| {
            b.iter(|| cycle_while(*i));
        },
    );
    group.bench_with_input(
        format!("cycle_loop_{MAX_FIBONACCI_FOR_U64}"),
        &MAX_FIBONACCI_FOR_U64,
        |b, i| {
            b.iter(|| cycle_loop(*i));
        },
    );
    group.bench_with_input(
        format!("safe_fibonacci_v1_{MAX_FIBONACCI_FOR_U64}"),
        &MAX_FIBONACCI_FOR_U64,
        |b, i| {
            b.iter(|| safe_fibonacci_v1(*i));
        },
    );
    group.bench_with_input(
        format!("safe_fibonacci_v2_{MAX_FIBONACCI_FOR_U64}"),
        &MAX_FIBONACCI_FOR_U64,
        |b, i| {
            b.iter(|| safe_fibonacci_v2(*i));
        },
    );
    group.bench_with_input(
        format!("safe_fibonacci_v3_{MAX_FIBONACCI_FOR_U64}"),
        &MAX_FIBONACCI_FOR_U64,
        |b, i| {
            b.iter(|| safe_fibonacci_v3(*i));
        },
    );
    group.bench_with_input(
        format!("safe_fibonacci_v4_{MAX_FIBONACCI_FOR_U64}"),
        &MAX_FIBONACCI_FOR_U64,
        |b, i| {
            b.iter(|| safe_fibonacci_v4(*i));
        },
    );
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
