use year_2022_day_10::{self, INPUT};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let instructions = year_2022_day_10::parse_input(INPUT);

    c.bench_function("year_2022_day_10::parse_input", |b| {
        b.iter(|| year_2022_day_10::parse_input(black_box(INPUT)));
    });

    c.bench_function("year_2022_day_10::solve", |b| {
        b.iter(|| year_2022_day_10::solve(black_box(&instructions)));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
