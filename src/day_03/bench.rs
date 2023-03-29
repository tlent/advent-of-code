use advent_of_code_2022::day_03::{self, INPUT};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let lines: Vec<_> = INPUT.lines().collect();

    c.bench_function("day_03::part_one", |b| {
        b.iter(|| day_03::part_one(black_box(&lines)));
    });

    c.bench_function("day_03::part_two", |b| {
        b.iter(|| day_03::part_two(black_box(&lines)));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
