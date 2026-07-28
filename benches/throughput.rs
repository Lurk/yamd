use std::hint::black_box;

use criterion::{Criterion, Throughput, criterion_group, criterion_main};

use yamd::{deserialize, lexer::Lexer, op, to_yamd};

// cancat of all YAMD documents from https://github.com/Lurk/barhamon/tree/main/content on
// 2024-12-25
const LONG_VALID_YAMD: &str = include_str!("./human_input.yamd");
/// random tokens with long lines
/// output of yamd_utils random -m=100 352343
const RANDOM_LOW_DENSITY: &str = include_str!("./random_token_low_density.yamd");
/// random tokens with short lines
/// output of yamd_utils random -m=10 352343
const RANDOM_HIGH_DENSITY: &str = include_str!("./random_token_high_density.yamd");
/// small, hand-written document covering common node types
const SMALL_YAMD: &str = include_str!("./small.yamd");

fn datasets() -> [(&'static str, &'static str); 4] {
    [
        ("~344kb of YAMD written by humman", LONG_VALID_YAMD),
        ("~346kb with low density of tokens", RANDOM_LOW_DENSITY),
        ("~344kb with high density of tokens", RANDOM_HIGH_DENSITY),
        ("small yamd document", SMALL_YAMD),
    ]
}

fn lexer(c: &mut Criterion) {
    let mut group = c.benchmark_group("lexer");
    group.measurement_time(std::time::Duration::from_secs(10));
    for (name, input) in datasets() {
        group.throughput(Throughput::Bytes(input.len() as u64));
        group.bench_function(name, |b| {
            b.iter(|| {
                for token in Lexer::new(black_box(input)) {
                    black_box(token);
                }
            })
        });
    }
    group.finish();
}

fn parser(c: &mut Criterion) {
    let mut group = c.benchmark_group("parser");
    group.measurement_time(std::time::Duration::from_secs(10));
    for (name, input) in datasets() {
        group.throughput(Throughput::Bytes(input.len() as u64));
        group.bench_function(name, |b| b.iter(|| op::parse(black_box(input))));
    }
    group.finish();
}

fn ast(c: &mut Criterion) {
    let mut group = c.benchmark_group("ast");
    group.measurement_time(std::time::Duration::from_secs(10));
    for (name, input) in datasets() {
        group.throughput(Throughput::Bytes(input.len() as u64));
        let ops = op::parse(input);
        group.bench_function(name, |b| {
            b.iter(|| to_yamd(black_box(&ops), black_box(input)))
        });
    }
    group.finish();
}

fn roundtrip(c: &mut Criterion) {
    let mut group = c.benchmark_group("roundtrip");
    group.measurement_time(std::time::Duration::from_secs(10));
    for (name, input) in datasets() {
        group.throughput(Throughput::Bytes(input.len() as u64));
        group.bench_function(name, |b| {
            b.iter(|| deserialize(black_box(input)).to_string())
        });
    }
    group.finish();
}

criterion_group!(benches, lexer, parser, ast, roundtrip);
criterion_main!(benches);
