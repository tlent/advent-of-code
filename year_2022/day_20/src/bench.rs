use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day_20::{self, INPUT};

fn criterion_benchmark(c: &mut Criterion) {
    let parse_result = day_20::parse_input(INPUT);

    c.bench_function("day_20::parse_input", |b| {
        b.iter(|| day_20::parse_input(black_box(INPUT)));
    });

    c.bench_function("day_20::part_one", |b| {
        b.iter_batched(
            || parse_result.clone(),
            |list| day_20::part_one(black_box(list)),
            criterion::BatchSize::SmallInput,
        );
    });

    c.bench_function("day_20::part_two", |b| {
        b.iter_batched(
            || parse_result.clone(),
            |list| day_20::part_two(black_box(list)),
            criterion::BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
