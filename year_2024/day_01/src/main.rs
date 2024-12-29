#![feature(test)]
extern crate test;

use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

struct Input {
    left_list: Vec<u32>,
    right_list: Vec<u32>,
}

fn parse_input(input: &str) -> Result<Input, &str> {
    let mut left_list = vec![];
    let mut right_list = vec![];
    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let left = parts
            .next()
            .ok_or("Missing left value")?
            .parse()
            .map_err(|_| "Invalid number")?;
        let right = parts
            .next()
            .ok_or("Missing right value")?
            .parse()
            .map_err(|_| "Invalid number")?;
        left_list.push(left);
        right_list.push(right);
    }
    Ok(Input {
        left_list,
        right_list,
    })
}

fn part_one(input: &Input) -> u32 {
    let mut sorted_left_list = input.left_list.clone();
    let mut sorted_right_list = input.right_list.clone();
    sorted_left_list.sort_unstable();
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
        *right_counts.entry(v).or_insert(0) += 1;
    }
    input
        .left_list
        .iter()
        .map(|l| l * right_counts.get(l).copied().unwrap_or(0))
        .sum()
}

fn main() {
    match parse_input(INPUT) {
        Ok(input) => {
            let run_mode = std::env::args().nth(1);
            match run_mode.as_deref() {
                Some("parse") => {}
                Some("one") => println!("{}", part_one(&input)),
                Some("two") => println!("{}", part_two(&input)),
                Some("all") => {
                    println!("{}", part_one(&input));
                    println!("{}", part_two(&input));
                }
                _ => eprintln!("Invalid argument: must be one of 'all', 'parse', 'one', or 'two'."),
            }
        }
        Err(e) => eprintln!("Parse error: {e}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::hint::black_box;
    use test::Bencher;

    #[test]
    fn test_part_one() {
        let input = parse_input(INPUT).unwrap();
        assert_eq!(part_one(&input), 1666427);
    }

    #[test]
    fn test_part_two() {
        let input = parse_input(INPUT).unwrap();
        assert_eq!(part_two(&input), 24316233);
    }

    #[bench]
    fn bench_parse_input(b: &mut Bencher) {
        b.iter(|| parse_input(black_box(INPUT)));
    }

    #[bench]
    fn bench_part_one(b: &mut Bencher) {
        let input = parse_input(INPUT).unwrap();
        b.iter(|| part_one(black_box(&input)));
    }

    #[bench]
    fn bench_part_two(b: &mut Bencher) {
        let input = parse_input(INPUT).unwrap();
        b.iter(|| part_two(black_box(&input)));
    }
}
