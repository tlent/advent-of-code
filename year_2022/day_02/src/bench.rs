use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day_02::{self, INPUT};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day_02::solve", |b| {
        b.iter(|| day_02::solve(black_box(INPUT)));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
