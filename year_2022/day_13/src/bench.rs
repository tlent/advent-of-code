use year_2022_day_13::INPUT;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let packets = year_2022_day_13::parse_input(INPUT);

    c.bench_function("year_2022_day_13::parse_input", |b| {
        b.iter(|| year_2022_day_13::parse_input(black_box(INPUT)));
    });

    c.bench_function("year_2022_day_13::part_one", |b| {
        b.iter(|| year_2022_day_13::part_one(black_box(&packets)));
    });

    c.bench_function("year_2022_day_13::part_two", |b| {
        b.iter(|| year_2022_day_13::part_two(black_box(&packets)));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
