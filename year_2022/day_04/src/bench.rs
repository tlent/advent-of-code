use criterion::{black_box, criterion_group, criterion_main, Criterion};
use year_2022_day_04::{self, INPUT};

fn criterion_benchmark(c: &mut Criterion) {
    let pairs = year_2022_day_04::parse_input(INPUT).unwrap();

    c.bench_function("year_2022_day_04::parse_input", |b| {
        b.iter(|| year_2022_day_04::parse_input(black_box(INPUT)));
    });

    c.bench_function("year_2022_day_04::part_one", |b| {
        b.iter(|| year_2022_day_04::part_one(black_box(&pairs)));
    });

    c.bench_function("year_2022_day_04::part_two", |b| {
        b.iter(|| year_2022_day_04::part_two(black_box(&pairs)));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
