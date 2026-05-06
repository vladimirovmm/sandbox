use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use exp_sort::{bubble_sort, heap_sort, insertion_sort, merge_sort, quick_sort, selection_sort};

/// Малый массив для тестирования сортировки
const SMALL_ARR: [i32; 5] = [5, 2, 8, 1, 9];
/// Размер большого массива для тестирования
const BIG_ARR_SIZE: usize = 1000;
/// Большой массив для тестирования сортировки
const BIG_ARR: [i32; BIG_ARR_SIZE] = {
    let mut arr = [0i32; BIG_ARR_SIZE];
    let mut i = 0;
    while i < BIG_ARR_SIZE {
        arr[i] = (i % 100) as i32;
        i += 1;
    }
    arr
};

fn benchmark_sort(cri: &mut Criterion) {
    cri.bench_function("bubble_sort_small", |b| {
        b.iter(|| {
            let mut arr = SMALL_ARR.clone();
            bubble_sort(black_box(&mut arr));
        })
    });
    cri.bench_function("bubble_sort_large", |b| {
        b.iter(|| {
            let mut arr = BIG_ARR.clone();
            bubble_sort(black_box(&mut arr));
        })
    });

    cri.bench_function("selection_sort_small", |b| {
        b.iter(|| {
            let mut arr = SMALL_ARR.clone();
            selection_sort(black_box(&mut arr));
        })
    });
    cri.bench_function("selection_sort_large", |b| {
        b.iter(|| {
            let mut arr = BIG_ARR.clone();
            selection_sort(black_box(&mut arr));
        })
    });

    cri.bench_function("insertion_sort_small", |b| {
        b.iter(|| {
            let mut arr = SMALL_ARR.clone();
            insertion_sort(black_box(&mut arr));
        })
    });
    cri.bench_function("insertion_sort_large", |b| {
        b.iter(|| {
            let mut arr = BIG_ARR.clone();
            insertion_sort(black_box(&mut arr));
        })
    });

    cri.bench_function("quick_sort_small", |b| {
        b.iter(|| {
            let mut arr = SMALL_ARR.clone();
            quick_sort(black_box(&mut arr));
        })
    });
    cri.bench_function("quick_sort_large", |b| {
        b.iter(|| {
            let mut arr = BIG_ARR.clone();
            quick_sort(black_box(&mut arr));
        })
    });

    cri.bench_function("merge_sort_small", |b| {
        b.iter(|| {
            let mut arr = SMALL_ARR.clone();
            merge_sort(black_box(&mut arr));
        })
    });
    cri.bench_function("merge_sort_large", |b| {
        b.iter(|| {
            let mut arr = BIG_ARR.clone();
            merge_sort(black_box(&mut arr));
        })
    });

    cri.bench_function("heap_small", |b| {
        b.iter(|| {
            let mut arr = SMALL_ARR.clone();
            heap_sort(black_box(&mut arr));
        })
    });
    cri.bench_function("heap_large", |b| {
        b.iter(|| {
            let mut arr = BIG_ARR.clone();
            heap_sort(black_box(&mut arr));
        })
    });

    cri.bench_function("base_small", |b| {
        b.iter(|| {
            let mut arr = SMALL_ARR.clone();
            arr.sort();
        })
    });
    cri.bench_function("base_large", |b| {
        b.iter(|| {
            let mut arr = BIG_ARR.clone();
            arr.sort();
        })
    });
}

criterion_group!(benches, benchmark_sort);
criterion_main!(benches);
