use day_08::{self, INPUT};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let grid = day_08::parse_input(INPUT);

    c.bench_function("day_08::parse_input", |b| {
        b.iter(|| day_08::parse_input(black_box(INPUT)));
    });

    c.bench_function("day_08::part_one", |b| {
        b.iter(|| day_08::part_one(black_box(&grid)));
    });

    c.bench_function("day_08::part_two", |b| {
        b.iter(|| day_08::part_two(black_box(&grid)));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
