use criterion::{black_box, criterion_group, criterion_main, Criterion};
use year_2023_day_07::INPUT;

fn criterion_benchmark(c: &mut Criterion) {
    let mut parse_result = year_2023_day_07::parse_input(INPUT);

    c.bench_function("year_2023_day_07::parse_input", |b| {
        b.iter(|| year_2023_day_07::parse_input(black_box(INPUT)));
    });

    c.bench_function("year_2023_day_07::part_one", |b| {
        b.iter(|| year_2023_day_07::part_one(black_box(&mut parse_result)));
    });

    c.bench_function("year_2023_day_07::part_two", |b| {
        b.iter(|| year_2023_day_07::part_two(black_box(&mut parse_result)));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
