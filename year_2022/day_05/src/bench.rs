use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day_05::{self, INPUT};

fn criterion_benchmark(c: &mut Criterion) {
    let (stacks, moves) = day_05::parse_input(INPUT).unwrap();

    c.bench_function("day_05::parse_input", |b| {
        b.iter(|| day_05::parse_input(black_box(INPUT)));
    });

    c.bench_function("day_05::part_one", |b| {
        b.iter_batched(
            || stacks.clone(),
            |stacks| day_05::part_one(black_box(stacks), &moves),
            criterion::BatchSize::SmallInput,
        );
    });

    c.bench_function("day_05::part_two", |b| {
        b.iter_batched(
            || stacks.clone(),
            |stacks| day_05::part_two(black_box(stacks), &moves),
            criterion::BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
