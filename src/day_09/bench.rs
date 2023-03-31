use advent_of_code_2022::day_09::{self, INPUT};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let motions = day_09::parse_input(INPUT);

    c.bench_function("day_09::parse_input", |b| {
        b.iter(|| day_09::parse_input(black_box(INPUT)));
    });

    c.bench_function("day_09::part_one", |b| {
        b.iter(|| day_09::part_one(black_box(&motions)));
    });

    c.bench_function("day_09::part_two", |b| {
        b.iter(|| day_09::part_two(black_box(&motions)));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
