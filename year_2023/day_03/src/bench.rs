use criterion::{black_box, criterion_group, criterion_main, Criterion};
use year_2023_day_03::INPUT;

fn criterion_benchmark(c: &mut Criterion) {
    let parse_result = year_2023_day_03::parse_input(INPUT);

    c.bench_function("year_2023_day_03::parse_input", |b| {
        b.iter(|| year_2023_day_03::parse_input(black_box(INPUT)));
    });

    c.bench_function("year_2023_day_03::part_one", |b| {
        b.iter(|| year_2023_day_03::part_one(black_box(&parse_result)));
    });

    c.bench_function("year_2023_day_03::part_two", |b| {
        b.iter(|| year_2023_day_03::part_two(black_box(&parse_result)));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
