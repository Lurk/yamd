use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion, Throughput};

use yamd::deserialize;

// cancat of all YAMD documents from https://github.com/Lurk/barhamon/tree/main/content on
// 2024-12-25
const LONG_VALID_YAMD: &str = include_str!("./human_input.yamd");
/// random tokens with long lines
/// output of yamd_utils random -m=100 352343
const RANDOM_LOW_DENSITY: &str = include_str!("./random_token_low_density.yamd");
/// random tokens with short lines
/// output of yamd_utils random -m=10 352343
const RANDOM_HIGH_DENSITY: &str = include_str!("./random_token_high_density.yamd");

fn long_valid(c: &mut Criterion) {
    let mut group = c.benchmark_group("throughput");
    group.throughput(Throughput::Bytes(LONG_VALID_YAMD.len() as u64));
    group.bench_function("~344kb of YAMD written by humman", |b| {
        b.iter(|| deserialize(black_box(LONG_VALID_YAMD)))
    });
    group.finish();
}

fn random_long_lines(c: &mut Criterion) {
    let mut group = c.benchmark_group("throughput");
    group.throughput(Throughput::Bytes(RANDOM_LOW_DENSITY.len() as u64));
    group.bench_function("~346kb with low density of tokens", |b| {
        b.iter(|| deserialize(black_box(RANDOM_LOW_DENSITY)))
    });
    group.finish();
}

fn random_short_lines(c: &mut Criterion) {
    let mut group = c.benchmark_group("throughput");
    group.throughput(Throughput::Bytes(RANDOM_HIGH_DENSITY.len() as u64));
    group.bench_function("~344kb with high density of tokens", |b| {
        b.iter(|| deserialize(black_box(RANDOM_HIGH_DENSITY)))
    });
    group.finish();
}

criterion_group!(benches, long_valid, random_short_lines, random_long_lines);
criterion_main!(benches);
