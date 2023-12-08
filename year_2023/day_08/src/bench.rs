use criterion::{black_box, criterion_group, criterion_main, Criterion};
use year_2023_day_08::INPUT;

fn criterion_benchmark(c: &mut Criterion) {
    let (turns, map, starts) = year_2023_day_08::parse_input(INPUT);

    c.bench_function("year_2023_day_08::parse_input", |b| {
        b.iter(|| year_2023_day_08::parse_input(black_box(INPUT)));
    });

    c.bench_function("year_2023_day_08::part_one", |b| {
        b.iter(|| year_2023_day_08::part_one(black_box(turns), black_box(&map)));
    });

    c.bench_function("year_2023_day_08::part_two", |b| {
        b.iter(|| year_2023_day_08::part_two(turns, black_box(&map), black_box(&starts)));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
