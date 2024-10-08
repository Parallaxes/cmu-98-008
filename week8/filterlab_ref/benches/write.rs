use criterion::{black_box, criterion_group, criterion_main, Criterion};
use filterlab_ref::BloomFilter;
use rand::distributions::Standard;
use rand::Rng;

/// Hack: this should be enough operations so that the bencher doesn't run out of work.
const MEGABYTE: usize = 1 << 20;
const GIGABYTE: usize = 1 << 30;

pub fn bloom_filter_write_benchmark(c: &mut Criterion) {
    // Generate 1 million random integers.
    let list: Vec<i32> = rand::thread_rng()
        .sample_iter(Standard)
        .take(MEGABYTE)
        .collect();

    // Allocate the bloom filter.
    let mut bf = BloomFilter::new(GIGABYTE, 64);

    let mut index = 0;
    c.bench_function("write", |b| {
        b.iter(|| {
            bf.insert(black_box(&list[index % list.len()]));
            index += 1;
        })
    });

    black_box(bf);
}

criterion_main!(benches);
criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = bloom_filter_write_benchmark
}
