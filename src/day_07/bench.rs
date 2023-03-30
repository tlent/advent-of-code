use advent_of_code_2022::day_07::{self, INPUT};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let (directory_sizes, root_size) = day_07::parse_input(INPUT);

    c.bench_function("day_07::parse_input", |b| {
        b.iter(|| day_07::parse_input(black_box(INPUT)));
    });

    c.bench_function("day_07::part_one", |b| {
        b.iter(|| day_07::part_one(black_box(&directory_sizes)));
    });

    c.bench_function("day_07::part_two", |b| {
        b.iter(|| day_07::part_two(black_box(&directory_sizes), black_box(root_size)));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
