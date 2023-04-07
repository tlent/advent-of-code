use advent_of_code_2022::day_13::{self, INPUT};
use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let packets = day_13::parse_input(INPUT);

    c.bench_function("day_13::parse_input", |b| {
        b.iter(|| day_13::parse_input(black_box(INPUT)));
    });

    c.bench_function("day_13::part_one", |b| {
        b.iter(|| day_13::part_one(black_box(&packets)));
    });

    c.bench_function("day_13::part_two", |b| {
        b.iter_batched(
            || packets.clone(),
            |packets| day_13::part_two(black_box(packets)),
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
