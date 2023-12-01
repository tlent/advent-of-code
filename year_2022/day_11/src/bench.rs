use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use year_2022_day_11::INPUT;

fn criterion_benchmark(c: &mut Criterion) {
    let monkeys = year_2022_day_11::parser::parse(INPUT).unwrap();

    c.bench_function("year_2022_day_11::parse_input", |b| {
        b.iter(|| year_2022_day_11::parser::parse(black_box(INPUT)).unwrap());
    });

    c.bench_function("year_2022_day_11::part_one", |b| {
        b.iter_batched(
            || monkeys.clone(),
            |mut monkeys| year_2022_day_11::part_one(black_box(&mut monkeys)),
            BatchSize::SmallInput,
        );
    });

    c.bench_function("year_2022_day_11::part_two", |b| {
        b.iter_batched(
            || monkeys.clone(),
            |mut monkeys| year_2022_day_11::part_two(black_box(&mut monkeys)),
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
