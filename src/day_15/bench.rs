use advent_of_code_2022::day_15::{self, INPUT};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let sensors = day_15::parser::parse(INPUT).unwrap();

    c.bench_function("day_15::parser::parse", |b| {
        b.iter(|| day_15::parser::parse(black_box(INPUT)));
    });

    c.bench_function("day_15::part_one", |b| {
        b.iter(|| day_15::part_one(black_box(&sensors)));
    });

    c.bench_function("day_15::part_two", |b| {
        b.iter(|| day_15::part_two(black_box(&sensors)));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
