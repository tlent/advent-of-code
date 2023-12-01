use year_2022_day_03::INPUT;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let (part_one_sets, part_two_sets) = year_2022_day_03::parse_input(INPUT);

    c.bench_function("year_2022_day_03::parse_input", |b| {
        b.iter(|| year_2022_day_03::parse_input(black_box(INPUT)));
    });

    c.bench_function("year_2022_day_03::part_one", |b| {
        b.iter(|| year_2022_day_03::part_one(black_box(&part_one_sets)));
    });

    c.bench_function("year_2022_day_03::part_two", |b| {
        b.iter(|| year_2022_day_03::part_two(black_box(&part_two_sets)));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
