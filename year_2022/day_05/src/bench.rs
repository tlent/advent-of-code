use criterion::{black_box, criterion_group, criterion_main, Criterion};
use year_2022_day_05::INPUT;

fn criterion_benchmark(c: &mut Criterion) {
    let (stacks, moves) = year_2022_day_05::parse_input(INPUT).unwrap();

    c.bench_function("year_2022_day_05::parse_input", |b| {
        b.iter(|| year_2022_day_05::parse_input(black_box(INPUT)));
    });

    c.bench_function("year_2022_day_05::part_one", |b| {
        b.iter_batched(
            || stacks.clone(),
            |stacks| year_2022_day_05::part_one(black_box(stacks), &moves),
            criterion::BatchSize::SmallInput,
        );
    });

    c.bench_function("year_2022_day_05::part_two", |b| {
        b.iter_batched(
            || stacks.clone(),
            |stacks| year_2022_day_05::part_two(black_box(stacks), &moves),
            criterion::BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
