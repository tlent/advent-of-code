use advent_of_code_2022::day_14::{self, INPUT};
use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let world = day_14::parse_input(INPUT);

    c.bench_function("day_14::parse_input", |b| {
        b.iter(|| day_14::parse_input(black_box(INPUT)));
    });

    c.bench_function("day_14::part_one", |b| {
        b.iter_batched(
            || world.clone(),
            |mut world| day_14::part_one(black_box(&mut world)),
            BatchSize::SmallInput,
        );
    });

    c.bench_function("day_14::part_two", |b| {
        b.iter_batched(
            || world.clone(),
            |mut world| day_14::part_two(black_box(&mut world)),
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
