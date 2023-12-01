use criterion::{black_box, criterion_group, criterion_main, Criterion};
use year_2022_day_01::{self, INPUT};

fn criterion_benchmark(c: &mut Criterion) {
    let group_sums = year_2022_day_01::parse_input(INPUT).unwrap();

    c.bench_function("year_2022_day_01::parse_input", |b| {
        b.iter(|| year_2022_day_01::parse_input(black_box(INPUT)));
    });

    c.bench_function("year_2022_day_01::solve", |b| {
        b.iter_batched(
            || group_sums.clone(),
            |group_sums| year_2022_day_01::solve(black_box(group_sums)),
            criterion::BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
