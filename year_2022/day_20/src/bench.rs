use day_20::{self, INPUT};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let parse_result = day_20::parse_input(INPUT);

    c.bench_function("day_20::parse_input", |b| {
        b.iter(|| day_20::parse_input(black_box(INPUT)));
    });

    c.bench_function("day_20::part_one", |b| {
        b.iter(|| day_20::part_one(black_box(&parse_result)));
    });

    c.bench_function("day_20::part_two", |b| {
        b.iter(|| day_20::part_two(black_box(&parse_result)));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
