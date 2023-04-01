use advent_of_code_2022::day_10::{self, INPUT};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let instructions = day_10::parse_input(INPUT);

    c.bench_function("day_10::parse_input", |b| {
        b.iter(|| day_10::parse_input(black_box(INPUT)));
    });

    c.bench_function("day_10::solve", |b| {
        b.iter(|| day_10::solve(black_box(&instructions)));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);