use year_2022_day_06::INPUT;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("year_2022_day_06::part_one", |b| {
        b.iter(|| year_2022_day_06::part_one(black_box(INPUT)));
    });

    c.bench_function("year_2022_day_06::part_two", |b| {
        b.iter(|| year_2022_day_06::part_two(black_box(INPUT)));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
