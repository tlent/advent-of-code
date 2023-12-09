#![feature(test)]
extern crate test;

use std::mem;

pub const INPUT: &str = include_str!("../input.txt");

#[derive(Debug)]
pub struct Input {
    history_values: Vec<i32>,
    history_length: usize,
}

impl Input {
    pub fn histories(&self) -> impl Iterator<Item = &[i32]> {
        self.history_values.chunks_exact(self.history_length)
    }
}

pub fn parse_input(input: &str) -> Input {
    let first_line_end = input.find('\n').unwrap();
    let mut history_values: Vec<_> = input[..first_line_end]
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let history_length = history_values.len();
    history_values.extend(
        input[first_line_end + 1..]
            .split_ascii_whitespace()
            .map(|s| s.parse::<i32>().unwrap()),
    );
    Input {
        history_values,
        history_length,
    }
}

pub fn part_one(input: &Input) -> i32 {
    let mut sequence = Vec::with_capacity(input.history_length);
    let mut next_sequence = Vec::with_capacity(input.history_length);
    input
        .histories()
        .map(|history| {
            sequence.clear();
            sequence.extend_from_slice(history);
            let mut sum = *history.last().unwrap();
            while sequence.iter().any(|&value| value != sequence[0]) {
                next_sequence.clear();
                next_sequence.extend(sequence.windows(2).map(|window| window[1] - window[0]));
                sum += *next_sequence.last().unwrap();
                mem::swap(&mut sequence, &mut next_sequence);
            }
            sum
        })
        .sum()
}

pub fn part_two(input: &Input) -> i32 {
    let mut sequence = Vec::with_capacity(input.history_length);
    let mut next_sequence = Vec::with_capacity(input.history_length);
    let mut firsts = Vec::with_capacity(input.history_length);
    input
        .histories()
        .map(|history| {
            sequence.clear();
            sequence.extend_from_slice(history);
            firsts.clear();
            firsts.push(*history.first().unwrap());
            while sequence.iter().any(|&value| value != sequence[0]) {
                next_sequence.clear();
                next_sequence.extend(sequence.windows(2).map(|window| window[1] - window[0]));
                firsts.push(next_sequence[0]);
                mem::swap(&mut sequence, &mut next_sequence);
            }
            firsts
                .iter()
                .copied()
                .rev()
                .reduce(|accumulator, element| element - accumulator)
                .unwrap()
        })
        .sum()
}

fn main() {
    let parse_result = parse_input(INPUT);
    match std::env::args().nth(1).as_deref() {
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
        let input = parse_input(INPUT);
        assert_eq!(part_one(&input), 1_916_822_650);
    }

    #[test]
    fn test_part_two() {
        let input = parse_input(INPUT);
        assert_eq!(part_two(&input), 966);
    }

    #[bench]
    fn bench_parse_input(b: &mut Bencher) {
        b.iter(|| parse_input(black_box(INPUT)));
    }

    #[bench]
    fn bench_part_one(b: &mut Bencher) {
        let input = parse_input(INPUT);
        b.iter(|| part_one(black_box(&input)));
    }

    #[bench]
    fn bench_part_two(b: &mut Bencher) {
        let input = parse_input(INPUT);
        b.iter(|| part_two(black_box(&input)));
    }
}
