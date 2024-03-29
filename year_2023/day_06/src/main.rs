#![feature(test)]
extern crate test;

use std::env;

pub const INPUT: &str = include_str!("../input.txt");

pub struct Input {
    part_one_races: Vec<Race>,
    part_two_race: Race,
}

pub struct Race {
    time: u64,
    distance: u64,
}

pub fn parse_input(input: &str) -> Input {
    let mut lines = input.lines();
    let time_line = lines.next().unwrap();
    let distance_line = lines.next().unwrap();
    let times = time_line[12..]
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap());
    let distances = distance_line[12..]
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap());
    let part_one_races = times
        .zip(distances)
        .map(|(time, distance)| Race { time, distance })
        .collect();
    let part_two_race = Race {
        time: parse_digits(&time_line[12..]),
        distance: parse_digits(&distance_line[12..]),
    };
    Input {
        part_one_races,
        part_two_race,
    }
}

fn parse_digits(s: &str) -> u64 {
    let mut value = 0;
    for b in s.bytes() {
        if b.is_ascii_digit() {
            value = value * 10 + (b - b'0') as u64;
        }
    }
    value
}

pub fn part_one(input: &Input) -> u64 {
    input
        .part_one_races
        .iter()
        .map(|race| {
            let min_winning_time = min_winning_time(race).unwrap();
            race.time - (2 * min_winning_time - 1)
        })
        .product()
}

pub fn part_two(input: &Input) -> u64 {
    let race = &input.part_two_race;
    let min_winning_time = min_winning_time(race).unwrap();
    race.time - (2 * min_winning_time - 1)
}

fn min_winning_time(race: &Race) -> Option<u64> {
    let t = race.time as f64;
    let d = race.distance as f64;
    let discriminant = t * t - 4.0 * d;
    if discriminant >= 0.0 {
        [
            (t + discriminant.sqrt()) / 2.0,
            (t - discriminant.sqrt()) / 2.0,
        ]
        .into_iter()
        .filter_map(|root| {
            if root >= 0.0 && root <= t {
                Some(root.ceil() as u64)
            } else {
                None
            }
        })
        .min()
    } else {
        None
    }
}

fn main() {
    let input = parse_input(INPUT);
    match env::args().nth(1).as_deref() {
        Some("all") => {
            let part_one = part_one(&input);
            println!("{part_one}");
            let part_two = part_two(&input);
            println!("{part_two}");
        }
        Some("parse") => {}
        Some("one") => {
            let part_one = part_one(&input);
            println!("{part_one}");
        }
        Some("two") => {
            let part_two = part_two(&input);
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
        assert_eq!(part_one(&input), 1_413_720);
    }

    #[test]
    fn test_part_two() {
        let input = parse_input(INPUT);
        assert_eq!(part_two(&input), 30_565_288);
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
