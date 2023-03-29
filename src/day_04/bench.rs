use advent_of_code_2022::day_04::{self, INPUT};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let pairs = day_04::parse_input(INPUT);

    c.bench_function("day_04::parse_input", |b| {
        b.iter(|| day_04::parse_input(black_box(INPUT)));
    });

    c.bench_function("day_04::part_one", |b| {
        b.iter(|| day_04::part_one(black_box(&pairs)));
    });

    c.bench_function("day_04::part_two", |b| {
        b.iter(|| day_04::part_two(black_box(&pairs)));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
