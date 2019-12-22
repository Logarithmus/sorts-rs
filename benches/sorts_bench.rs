use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use rand::{thread_rng, Rng};
use std::time::Duration;

fn sorts_benchmark(c: &mut Criterion) {
    use sorts_rs::sort;
    let mut group = c.benchmark_group("Sort u32");
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(10);
    let mut rng = thread_rng();

    for n in [1, 2, 3, 4].iter().map(|i| 10_usize.pow(*i)) {
        let data: Vec<u32> = (0..n).map(|_| rng.gen()).collect();
        group.bench_function(BenchmarkId::new("Bubble", n), |b| {
            b.iter_batched_ref(
                || data.clone(),
                |mut input| sort::bubble_sort(&mut input),
                BatchSize::SmallInput,
            )
        });
        group.bench_function(BenchmarkId::new("Selection", n), |b| {
            b.iter_batched_ref(
                || data.clone(),
                |mut input| sort::selection_sort(&mut input),
                BatchSize::SmallInput,
            )
        });
        group.bench_function(BenchmarkId::new("Insertion", n), |b| {
            b.iter_batched_ref(
                || data.clone(),
                |mut input| sort::insertion_sort(&mut input),
                BatchSize::SmallInput,
            )
        });
        group.bench_function(BenchmarkId::new("Shell", n), |b| {
            b.iter_batched_ref(
                || data.clone(),
                |mut input| sort::shell_sort(&mut input),
                BatchSize::SmallInput,
            )
        });
        group.bench_function(BenchmarkId::new("Qsort", n), |b| {
            b.iter_batched_ref(
                || data.clone(),
                |mut input| sort::qsort(&mut input),
                BatchSize::SmallInput,
            )
        });
        group.bench_function(BenchmarkId::new("Heap", n), |b| {
            b.iter_batched_ref(
                || data.clone(),
                |mut input| sort::heap_sort(&mut input),
                BatchSize::SmallInput,
            )
        });
        group.bench_function(BenchmarkId::new("Radix 4 unsigned", n), |b| {
            b.iter_batched_ref(
                || data.clone(),
                |mut input| sort::radix_sort_unsigned(&mut input, 8),
                BatchSize::SmallInput,
            )
        });
        group.bench_function(BenchmarkId::new("Radix 8 unsigned", n), |b| {
            b.iter_batched_ref(
                || data.clone(),
                |mut input| sort::radix_sort_unsigned(&mut input, 8),
                BatchSize::SmallInput,
            )
        });
        group.bench_function(BenchmarkId::new("Comb", n), |b| {
            b.iter_batched_ref(
                || data.clone(),
                |mut input| sort::comb_sort(&mut input),
                BatchSize::SmallInput,
            )
        });
    }
    group.finish();
}

criterion_group!(benches, sorts_benchmark);
criterion_main!(benches);
