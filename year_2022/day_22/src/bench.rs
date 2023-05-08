use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day_22::{self, INPUT};

fn criterion_benchmark(c: &mut Criterion) {
    let (map_regions, path) = day_22::parse_input(INPUT);

    c.bench_function("day_22::parse_input", |b| {
        b.iter(|| day_22::parse_input(black_box(INPUT)));
    });

    c.bench_function("day_22::part_one", |b| {
        b.iter(|| day_22::part_one(black_box(&map_regions), black_box(&path)));
    });

    c.bench_function("day_22::part_two", |b| {
        b.iter(|| day_22::part_two(black_box(&map_regions), black_box(&path)));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
