#![feature(test)]
extern crate test;

pub const INPUT: &str = include_str!("../input.txt");

pub struct Input<'a> {
    start: (usize, usize),
    lines: Vec<&'a [u8]>,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    West,
    South,
}

pub fn parse_input(input: &str) -> Input {
    let lines: Vec<_> = input.lines().map(str::as_bytes).collect();
    let start = lines
        .iter()
        .enumerate()
        .find_map(|(y, line)| {
            line.iter()
                .enumerate()
                .find(|(_x, &b)| b == b'S')
                .map(|(x, _b)| (x, y))
        })
        .unwrap();
    Input { start, lines }
}

pub fn part_one(input: &Input) -> u32 {
    use Direction::*;
    let (x, y) = input.start;
    let mut facing = if x > 0 && [b'-', b'L', b'F'].contains(&input.lines[y][x - 1]) {
        West
    } else if y > 0 && [b'|', b'7', b'F'].contains(&input.lines[y - 1][x]) {
        North
    } else {
        East
    };
    let (mut x, mut y) = step(input.start, facing);
    let mut steps = 1;
    loop {
        let pipe = input.lines[y][x];
        if pipe == b'S' {
            return steps / 2;
        }
        facing = match (pipe, facing) {
            (b'|', _) | (b'-', _) => facing,
            (b'L', South) => East,
            (b'L', West) => North,
            (b'J', South) => West,
            (b'J', East) => North,
            (b'7', North) => West,
            (b'7', East) => South,
            (b'F', North) => East,
            (b'F', West) => South,
            _ => panic!("invalid pipe {pipe}"),
        };
        (x, y) = step((x, y), facing);
        steps += 1;
    }
}

fn step((x, y): (usize, usize), direction: Direction) -> (usize, usize) {
    use Direction::*;
    match direction {
        North => (x, y - 1),
        East => (x + 1, y),
        West => (x - 1, y),
        South => (x, y + 1),
    }
}

pub fn part_two(input: &Input) -> u32 {
    todo!()
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
        assert_eq!(part_one(&input), 6613);
    }

    #[test]
    fn test_part_two() {
        let input = parse_input(INPUT);
        assert_eq!(part_two(&input), todo!());
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
