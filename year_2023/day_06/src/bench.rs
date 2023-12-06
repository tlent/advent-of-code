use criterion::{black_box, criterion_group, criterion_main, Criterion};
use year_2023_day_06::INPUT;

fn criterion_benchmark(c: &mut Criterion) {
    let (part_one_races, part_two_race) = year_2023_day_06::parse_input(INPUT);

    c.bench_function("year_2023_day_06::parse_input", |b| {
        b.iter(|| year_2023_day_06::parse_input(black_box(INPUT)));
    });

    c.bench_function("year_2023_day_06::part_one", |b| {
        b.iter(|| year_2023_day_06::part_one(black_box(&part_one_races)));
    });

    c.bench_function("year_2023_day_06::part_two", |b| {
        b.iter(|| year_2023_day_06::part_two(black_box(&part_two_race)));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
