use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day_04::{self, INPUT};

fn criterion_benchmark(c: &mut Criterion) {
    let pairs = day_04::parse_input(INPUT).unwrap();

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
