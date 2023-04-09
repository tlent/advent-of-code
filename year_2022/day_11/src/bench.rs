use day_11::{self, INPUT};
use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let monkeys = day_11::parse_input(INPUT);

    c.bench_function("day_11::parse_input", |b| {
        b.iter(|| day_11::parse_input(black_box(INPUT)));
    });

    c.bench_function("day_11::part_one", |b| {
        b.iter_batched(
            || monkeys.clone(),
            |mut monkeys| day_11::part_one(black_box(&mut monkeys)),
            BatchSize::SmallInput,
        );
    });

    c.bench_function("day_11::part_two", |b| {
        b.iter_batched(
            || monkeys.clone(),
            |mut monkeys| day_11::part_two(black_box(&mut monkeys)),
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
