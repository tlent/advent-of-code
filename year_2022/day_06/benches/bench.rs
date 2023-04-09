use day_06::{self, INPUT};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day_06::part_one", |b| {
        b.iter(|| day_06::part_one(black_box(INPUT)));
    });

    c.bench_function("day_06::part_two", |b| {
        b.iter(|| day_06::part_two(black_box(INPUT)));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
