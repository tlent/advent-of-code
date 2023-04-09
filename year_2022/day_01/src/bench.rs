use day_01::{self, INPUT};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let group_sums = day_01::parse_input(INPUT);

    c.bench_function("day_01::parse_input", |b| {
        b.iter(|| day_01::parse_input(black_box(INPUT)));
    });

    c.bench_function("day_01::solve", |b| {
        b.iter_batched(
            || group_sums.clone(),
            |group_sums| day_01::solve(black_box(group_sums)),
            criterion::BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
