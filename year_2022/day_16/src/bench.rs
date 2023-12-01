use criterion::{black_box, criterion_group, criterion_main, Criterion};
use year_2022_day_16::{self, INPUT};

fn criterion_benchmark(c: &mut Criterion) {
    let valves = year_2022_day_16::parser::parse(INPUT).unwrap();
    let (processed_valves, initial_distances) = year_2022_day_16::preprocess(valves.clone());

    c.bench_function("year_2022_day_16::parser::parse", |b| {
        b.iter(|| year_2022_day_16::parser::parse(black_box(INPUT)));
    });

    c.bench_function("year_2022_day_16::preprocess", |b| {
        b.iter_batched(
            || valves.clone(),
            |valves| year_2022_day_16::preprocess(black_box(valves)),
            criterion::BatchSize::SmallInput,
        );
    });

    c.bench_function("year_2022_day_16::part_one", |b| {
        b.iter(|| year_2022_day_16::part_one(black_box(&processed_valves), black_box(&initial_distances)));
    });

    c.bench_function("year_2022_day_16::part_two", |b| {
        b.iter(|| year_2022_day_16::part_two(black_box(&processed_valves), black_box(&initial_distances)));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
