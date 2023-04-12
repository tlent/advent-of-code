use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day_01::{self, INPUT};

fn criterion_benchmark(c: &mut Criterion) {
    let group_sums = day_01::parser::parse(INPUT).unwrap();

    c.bench_function("day_01::parser::parse", |b| {
        b.iter(|| day_01::parser::parse(black_box(INPUT)));
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
