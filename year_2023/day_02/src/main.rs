#![feature(test)]
extern crate test;

use std::env;

pub const INPUT: &str = include_str!("../input.txt");

pub struct Game {
    id: u32,
    samples: Vec<Sample>,
}

pub struct Sample {
    red: u32,
    green: u32,
    blue: u32,
}

pub fn parse_input(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(": ").unwrap();
            let id = left.split_once(' ').unwrap().1.parse().unwrap();
            let samples = right
                .split("; ")
                .map(|round| {
                    let mut sample = Sample {
                        red: 0,
                        green: 0,
                        blue: 0,
                    };
                    for s in round.split(", ") {
                        let (count, color) = s.split_once(' ').unwrap();
                        let count: u32 = count.parse().unwrap();
                        match color {
                            "red" => sample.red += count,
                            "green" => sample.green += count,
                            "blue" => sample.blue += count,
                            _ => panic!("invalid color"),
                        }
                    }
                    sample
                })
                .collect();
            Game { id, samples }
        })
        .collect()
}

pub fn part_one(games: &[Game]) -> u32 {
    const RED_LIMIT: u32 = 12;
    const GREEN_LIMIT: u32 = 13;
    const BLUE_LIMIT: u32 = 14;
    games
        .iter()
        .filter(|game| {
            game.samples.iter().all(|sample| {
                sample.red <= RED_LIMIT && sample.green <= GREEN_LIMIT && sample.blue <= BLUE_LIMIT
            })
        })
        .map(|game| game.id)
        .sum()
}

pub fn part_two(games: &[Game]) -> u32 {
    games
        .iter()
        .map(|game| {
            let max_red = game.samples.iter().map(|sample| sample.red).max().unwrap();
            let max_green = game
                .samples
                .iter()
                .map(|sample| sample.green)
                .max()
                .unwrap();
            let max_blue = game.samples.iter().map(|sample| sample.blue).max().unwrap();
            max_red * max_green * max_blue
        })
        .sum()
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
        let games = parse_input(INPUT);
        assert_eq!(part_one(&games), 3_059);
    }

    #[test]
    fn test_part_two() {
        let games = parse_input(INPUT);
        assert_eq!(part_two(&games), 65_371);
    }

    #[bench]
    fn bench_parse_input(b: &mut Bencher) {
        b.iter(|| parse_input(black_box(INPUT)));
    }

    #[bench]
    fn bench_part_one(b: &mut Bencher) {
        let games = parse_input(INPUT);
        b.iter(|| part_one(black_box(&games)));
    }

    #[bench]
    fn bench_part_two(b: &mut Bencher) {
        let games = parse_input(INPUT);
        b.iter(|| part_two(black_box(&games)));
    }
}
