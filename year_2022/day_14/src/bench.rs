use year_2022_day_14::INPUT;
use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let world = year_2022_day_14::parse_input(INPUT);

    c.bench_function("year_2022_day_14::parse_input", |b| {
        b.iter(|| year_2022_day_14::parse_input(black_box(INPUT)));
    });

    c.bench_function("year_2022_day_14::part_one", |b| {
        b.iter_batched(
            || world.clone(),
            |mut world| year_2022_day_14::part_one(black_box(&mut world)),
            BatchSize::SmallInput,
        );
    });

    c.bench_function("year_2022_day_14::part_two", |b| {
        b.iter_batched(
            || world.clone(),
            |mut world| year_2022_day_14::part_two(black_box(&mut world)),
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
