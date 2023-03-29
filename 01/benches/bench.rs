use advent_of_code_2022_day_01::{parse_input, part_one, part_two};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const INPUT: &str = include_str!("../input.txt");

fn criterion_benchmark(c: &mut Criterion) {
    let mut group_sums = parse_input(INPUT);
    group_sums.sort_unstable();

    c.bench_function("01::parse_input", |b| {
        b.iter(|| parse_input(black_box(INPUT)));
    });

    c.bench_function("01::part_one", |b| {
        b.iter(|| part_one(&group_sums));
    });

    c.bench_function("01::part_two", |b| {
        b.iter(|| part_two(&group_sums));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
