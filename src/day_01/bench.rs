use advent_of_code_2022::day_01::{self, INPUT};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let mut group_sums = day_01::parse_input(INPUT);
    group_sums.sort_unstable();

    c.bench_function("day_01::parse_input", |b| {
        b.iter(|| day_01::parse_input(black_box(INPUT)));
    });

    c.bench_function("day_01::part_one", |b| {
        b.iter(|| day_01::part_one(black_box(&group_sums)));
    });

    c.bench_function("day_01::part_two", |b| {
        b.iter(|| day_01::part_two(black_box(&group_sums)));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
