#![feature(test)]
extern crate test;

use std::env;

pub const INPUT: &str = include_str!("../input.txt");

pub fn parse_input(input: &str) -> Vec<u8> {
    let mut winning_numbers: Vec<u8> = Vec::with_capacity(10);
    input
        .lines()
        .map(|line| {
            winning_numbers.clear();
            winning_numbers.extend(
                line[10..39]
                    .split_ascii_whitespace()
                    .map(|s| s.parse::<u8>().unwrap()),
            );
            line[42..]
                .split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .filter(|number| winning_numbers.contains(number))
                .count() as u8
        })
        .collect()
}

pub fn part_one(win_counts: &[u8]) -> u32 {
    win_counts
        .iter()
        .map(|&count| {
            if count > 0 {
                2u32.pow(count as u32 - 1)
            } else {
                0
            }
        })
        .sum()
}

pub fn part_two(win_counts: &[u8]) -> u32 {
    let mut card_counts = vec![1; win_counts.len()];
    for (index, &win_count) in win_counts.iter().enumerate() {
        let card_count = card_counts[index];
        let start = index + 1;
        let end = start + win_count as usize;
        for later_card_count in card_counts[start..end].iter_mut() {
            *later_card_count += card_count;
        }
    }
    card_counts.iter().sum()
}

fn main() {
    let parse_result = parse_input(INPUT);
    match env::args().nth(1).as_deref() {
        Some("all") => {
            let part_one = part_one(&parse_result);
            println!("{part_one}");
            let part_two = part_two(&parse_result);
            println!("{part_two}");
        }
        Some("parse") => {}
        Some("one") => {
            let part_one = part_one(&parse_result);
            println!("{part_one}");
        }
        Some("two") => {
            let part_two = part_two(&parse_result);
            println!("{part_two}");
        }
        _ => println!("Invalid argument: must be one of all, parse, one, or two"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::hint::black_box;
    use test::Bencher;

    #[test]
    fn test_part_one() {
        let match_counts = parse_input(INPUT);
        assert_eq!(part_one(&match_counts), 22_897);
    }

    #[test]
    fn test_part_two() {
        let match_counts = parse_input(INPUT);
        assert_eq!(part_two(&match_counts), 5_095_824);
    }

    #[bench]
    fn bench_parse_input(b: &mut Bencher) {
        b.iter(|| parse_input(black_box(INPUT)));
    }

    #[bench]
    fn bench_part_one(b: &mut Bencher) {
        let match_counts = parse_input(INPUT);
        b.iter(|| part_one(black_box(&match_counts)));
    }

    #[bench]
    fn bench_part_two(b: &mut Bencher) {
        let match_counts = parse_input(INPUT);
        b.iter(|| part_two(black_box(&match_counts)));
    }
}
