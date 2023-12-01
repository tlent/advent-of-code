use year_2022_day_15::INPUT;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let sensors = year_2022_day_15::parser::parse(INPUT).unwrap();

    c.bench_function("year_2022_day_15::parser::parse", |b| {
        b.iter(|| year_2022_day_15::parser::parse(black_box(INPUT)));
    });

    c.bench_function("year_2022_day_15::part_one", |b| {
        b.iter(|| year_2022_day_15::part_one(black_box(&sensors)));
    });

    c.bench_function("year_2022_day_15::part_two", |b| {
        b.iter(|| year_2022_day_15::part_two(black_box(&sensors)));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
