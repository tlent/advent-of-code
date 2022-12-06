use std::ops::RangeInclusive;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let pairs = parse_input(&INPUT);
    let full_containment_count = pairs
        .iter()
        .filter(|pair| pair.has_full_containment())
        .count();
    println!("{}", full_containment_count);
    let overlapping_count = pairs.iter().filter(|pair| pair.has_overlap()).count();
    println!("{}", overlapping_count);
}

fn parse_input(input: &str) -> Vec<Pair> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(',').collect();
            let r1: Vec<u32> = parts[0].split('-').map(|s| s.parse().unwrap()).collect();
            let r2: Vec<u32> = parts[1].split('-').map(|s| s.parse().unwrap()).collect();
            let range1 = r1[0]..=r1[1];
            let range2 = r2[0]..=r2[1];
            Pair::new([range1, range2])
        })
        .collect()
}

struct Pair([RangeInclusive<u32>; 2]);

impl Pair {
    fn new(ranges: [RangeInclusive<u32>; 2]) -> Self {
        Self(ranges)
    }

    fn has_full_containment(&self) -> bool {
        let ranges = &self.0;
        [(&ranges[0], &ranges[1]), (&ranges[1], &ranges[0])]
            .iter()
            .any(|(a, b)| a.contains(b.start()) && a.contains(b.end()))
    }

    fn has_overlap(&self) -> bool {
        let ranges = &self.0;
        [(&ranges[0], &ranges[1]), (&ranges[1], &ranges[0])]
            .iter()
            .any(|(a, b)| a.contains(b.start()) || a.contains(b.end()))
    }
}
