#![feature(test)]
extern crate test;

use std::{cmp, env};

pub const INPUT: &str = include_str!("../input.txt");

pub struct Game {
    id: u32,
    max_red: u32,
    max_green: u32,
    max_blue: u32,
}

pub fn parse_input(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|line| {
            let mut id_end = 6;
            while line.as_bytes()[id_end].is_ascii_digit() {
                id_end += 1;
            }
            let id = line[5..id_end].parse().unwrap();
            let mut max_red = 0;
            let mut max_green = 0;
            let mut max_blue = 0;
            let color_counts = line[id_end + 2..]
                .split("; ")
                .flat_map(|s| s.split(", "))
                .map(|s| s.split_once(' ').unwrap());
            for (count, color) in color_counts {
                let count: u32 = count.parse().unwrap();
                match color {
                    "red" => max_red = cmp::max(max_red, count),
                    "green" => max_green = cmp::max(max_green, count),
                    "blue" => max_blue = cmp::max(max_blue, count),
                    c => panic!("invalid color {c}"),
                };
            }
            Game {
                id,
                max_red,
                max_green,
                max_blue,
            }
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
            game.max_red <= RED_LIMIT
                && game.max_green <= GREEN_LIMIT
                && game.max_blue <= BLUE_LIMIT
        })
        .map(|game| game.id)
        .sum()
}

pub fn part_two(games: &[Game]) -> u32 {
    games
        .iter()
        .map(|game| game.max_red * game.max_green * game.max_blue)
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
