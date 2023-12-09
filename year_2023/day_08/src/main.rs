#![feature(test)]
extern crate test;

use std::env;

pub const INPUT: &str = include_str!("../input.txt");

const SIZE: usize = 2usize.pow(15);
type Map = [u32; SIZE];

pub fn parse_input(input: &str) -> (&str, Map, Vec<u16>) {
    let mut lines = input.lines();
    let turns_line = lines.next().unwrap();
    lines.next();
    let mut map = [0; SIZE];
    let mut starts = vec![];
    for line in lines {
        let id = hash(&line[..3]);
        let left = hash(&line[7..10]);
        let right = hash(&line[12..15]);
        map[id as usize] = (left as u32) << 15 | right as u32;
        if line.as_bytes()[2] == b'A' {
            starts.push(id);
        }
    }
    (turns_line, map, starts)
}

const fn hash(s: &str) -> u16 {
    let bytes = s.as_bytes();
    encode(bytes[0]) << 10 | encode(bytes[1]) << 5 | encode(bytes[2])
}

const fn encode(b: u8) -> u16 {
    (b - b'A') as u16
}

pub fn part_one(turns: &str, map: &Map) -> u64 {
    steps_to_target(turns, map, hash("AAA"), |h| h == hash("ZZZ"))
}

pub fn part_two(turns: &str, map: &Map, starts: &[u16]) -> u64 {
    let is_target = |h: u16| h & 0b11111 == encode(b'Z');
    starts
        .iter()
        .map(|&start| steps_to_target(turns, map, start, is_target))
        .reduce(|a, b| {
            let mut gcd = a;
            let mut remainder = b;
            while remainder != 0 {
                (gcd, remainder) = (remainder, gcd % remainder);
            }
            (a * b) / gcd
        })
        .unwrap()
}

fn steps_to_target<F>(turns: &str, map: &Map, start: u16, is_target: F) -> u64
where
    F: Fn(u16) -> bool,
{
    let mut turns = turns.bytes().cycle();
    let mut current = start;
    (1..)
        .find(|_| {
            let value = map[current as usize];
            let left = (value >> 15) as u16;
            let right = (value & 0x7FFF) as u16;
            current = match turns.next().unwrap() {
                b'L' => left,
                b'R' => right,
                b => panic!("invalid turn {b}"),
            };
            is_target(current)
        })
        .unwrap()
}

fn main() {
    let (turns, map, starts) = parse_input(INPUT);
    match env::args().nth(1).as_deref() {
        Some("all") => {
            let part_one = part_one(turns, &map);
            println!("{part_one}");
            let part_two = part_two(turns, &map, &starts);
            println!("{part_two}");
        }
        Some("parse") => {}
        Some("one") => {
            let part_one = part_one(turns, &map);
            println!("{part_one}");
        }
        Some("two") => {
            let part_two = part_two(turns, &map, &starts);
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
        let (turns, map, _) = parse_input(INPUT);
        assert_eq!(part_one(turns, &map), 20_093);
    }

    #[test]
    fn test_part_two() {
        let (turns, map, starts) = parse_input(INPUT);
        assert_eq!(part_two(turns, &map, &starts), 22_103_062_509_257);
    }

    #[bench]
    fn bench_parse_input(b: &mut Bencher) {
        b.iter(|| parse_input(black_box(INPUT)));
    }

    #[bench]
    fn bench_part_one(b: &mut Bencher) {
        let (turns, map, _) = parse_input(INPUT);
        b.iter(|| part_one(black_box(turns), black_box(&map)));
    }

    #[bench]
    fn bench_part_two(b: &mut Bencher) {
        let (turns, map, starts) = parse_input(INPUT);
        b.iter(|| part_two(black_box(turns), black_box(&map), black_box(&starts)));
    }
}
