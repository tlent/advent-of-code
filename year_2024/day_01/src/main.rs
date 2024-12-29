#![feature(test)]

use std::collections::HashMap;
extern crate test;

const INPUT: &str = include_str!("../input.txt");

struct Input {
    left_list: Vec<u32>,
    right_list: Vec<u32>,
}

fn parse_input(input: &str) -> Input {
    let (left_list, right_list) = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let left: u32 = parts.next().unwrap().parse().unwrap();
            let right: u32 = parts.next().unwrap().parse().unwrap();
            (left, right)
        })
        .unzip();
    Input {
        left_list,
        right_list,
    }
}

fn part_one(input: &Input) -> u32 {
    let mut sorted_left_list = input.left_list.clone();
    sorted_left_list.sort_unstable();
    let mut sorted_right_list = input.right_list.clone();
    sorted_right_list.sort_unstable();
    sorted_left_list
        .into_iter()
        .zip(sorted_right_list.into_iter())
        .map(|(l, r)| l.abs_diff(r))
        .sum()
}

fn part_two(input: &Input) -> u32 {
    let mut right_counts: HashMap<u32, u32> = HashMap::new();
    for &v in &input.right_list {
        *right_counts.entry(v).or_default() += 1;
    }
    input
        .left_list
        .iter()
        .map(|l| l * right_counts.get(l).copied().unwrap_or_default())
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
        assert_eq!(part_one(&input), 1666427);
    }

    #[test]
    fn test_part_two() {
        let input = parse_input(INPUT);
        assert_eq!(part_two(&input), 24316233);
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
