use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion, Throughput};

use yamd::deserialize;

// cancat of all YAMD documents from https://github.com/Lurk/barhamon/tree/main/content on
// 2024-12-25
const LONG_VALID_YAMD: &str = include_str!("./human_input.yamd");
/// output of a `yamd_utils random 5000`
const LONG_RANDOM: &str = include_str!("./random_token.yamd");

fn long_valid(c: &mut Criterion) {
    let mut group = c.benchmark_group("throughput");
    group.throughput(Throughput::Bytes(LONG_VALID_YAMD.len() as u64));
    group.bench_function("~344kb lines of YAMD written by humman", |b| {
        b.iter(|| deserialize(black_box(LONG_VALID_YAMD)))
    });
    group.finish();
}

fn long_random(c: &mut Criterion) {
    let mut group = c.benchmark_group("throughput");
    group.throughput(Throughput::Bytes(LONG_RANDOM.len() as u64));
    group.bench_function("~346kb of random tokens", |b| {
        b.iter(|| deserialize(black_box(LONG_RANDOM)))
    });
    group.finish();
}

criterion_group!(benches, long_valid, long_random);
criterion_main!(benches);
