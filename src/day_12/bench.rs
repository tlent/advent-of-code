use advent_of_code_2022::day_12::{self, INPUT};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let grid = day_12::parse_input(INPUT);

    c.bench_function("day_12::parse_input", |b| {
        b.iter(|| day_12::parse_input(black_box(INPUT)));
    });

    c.bench_function("day_12::part_one", |b| {
        b.iter(|| day_12::part_one(black_box(&grid)));
    });

    c.bench_function("day_12::part_two", |b| {
        b.iter(|| day_12::part_two(black_box(&grid)));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
