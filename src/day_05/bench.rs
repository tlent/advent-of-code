use advent_of_code_2022::day_05::{self, INPUT};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let (stacks, moves) = day_05::parse_input(INPUT);

    c.bench_function("day_05::parse_input", |b| {
        b.iter(|| day_05::parse_input(black_box(INPUT)));
    });

    c.bench_function("day_05::part_one", |b| {
        b.iter(|| day_05::part_one(black_box(stacks.clone()), &moves));
    });

    c.bench_function("day_05::part_two", |b| {
        b.iter(|| day_05::part_two(black_box(stacks.clone()), &moves));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
