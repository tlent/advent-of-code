use std::ops::RangeInclusive;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let pairs = parse_input(INPUT);
    println!("{}", part_one(&pairs));
    println!("{}", part_two(&pairs));
}

fn parse_input(input: &str) -> Vec<(RangeInclusive<u32>, RangeInclusive<u32>)> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<_> = line
                .split([',', '-'])
                .map(|s| s.parse::<u32>().unwrap())
                .collect();
            (parts[0]..=parts[1], parts[2]..=parts[3])
        })
        .collect()
}

fn part_one(pairs: &[(RangeInclusive<u32>, RangeInclusive<u32>)]) -> usize {
    pairs
        .iter()
        .filter(|(a, b)| {
            b.contains(a.start()) && b.contains(a.end())
                || a.contains(b.start()) && a.contains(b.end())
        })
        .count()
}

fn part_two(pairs: &[(RangeInclusive<u32>, RangeInclusive<u32>)]) -> usize {
    pairs
        .iter()
        .filter(|(a, b)| {
            b.contains(a.start())
                || b.contains(a.end())
                || a.contains(b.start())
                || a.contains(b.end())
        })
        .count()
}
