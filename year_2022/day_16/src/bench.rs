use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day_16::{self, INPUT};

fn criterion_benchmark(c: &mut Criterion) {
    let parse_result = day_16::parser::parse(INPUT).unwrap();

    c.bench_function("day_16::parser::parse", |b| {
        b.iter(|| day_16::parser::parse(black_box(INPUT)));
    });

    // c.bench_function("day_16::part_one", |b| {
    //     b.iter(|| day_16::part_one(black_box(&parse_result)));
    // });
    //
    // c.bench_function("day_16::part_two", |b| {
    //     b.iter(|| day_16::part_two(black_box(&parse_result)));
    // });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
