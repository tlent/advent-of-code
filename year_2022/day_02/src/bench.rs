use criterion::{black_box, criterion_group, criterion_main, Criterion};
use year_2022_day_02::{self, INPUT};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("year_2022_day_02::solve", |b| {
        b.iter(|| year_2022_day_02::solve(black_box(INPUT)));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
