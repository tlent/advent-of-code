use std::collections::HashMap;
use std::collections::HashSet;

static INPUT: &str = include_str!("../input");

fn main() {
    println!("part one: {}", part_one(INPUT));
    println!("part two: {}", part_two(INPUT));
}

fn part_one(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|group| {
            let mut seen = HashSet::new();
            for c in group.chars() {
                if c != '\n' {
                    seen.insert(c);
                }
            }
            seen.len() as u32
        })
        .sum()
}

fn part_two(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|group| {
            let mut answer_count = HashMap::new();
            let mut group_size = 0;
            for person in group.lines() {
                group_size += 1;
                for c in person.chars() {
                    *answer_count.entry(c).or_insert(0) += 1;
                }
            }
            answer_count
                .iter()
                .filter(|&(_, &v)| v == group_size)
                .count() as u32
        })
        .sum()
}
