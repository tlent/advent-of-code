use advent_of_code_2022::day_02::{self, INPUT};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let (part_one_rounds, part_two_rounds) = day_02::parse_input(INPUT);

    c.bench_function("day_02::parse_input", |b| {
        b.iter(|| day_02::parse_input(black_box(INPUT)));
    });

    c.bench_function("day_02::part_one", |b| {
        b.iter(|| day_02::part_one(black_box(&part_one_rounds)));
    });

    c.bench_function("day_02::part_two", |b| {
        b.iter(|| day_02::part_two(black_box(&part_two_rounds)));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
