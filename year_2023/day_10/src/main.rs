#![feature(test)]
extern crate test;

pub const INPUT: &str = include_str!("../input.txt");

pub struct Input<'a> {
    start: (usize, usize),
    lines: Vec<&'a [u8]>,
}

pub struct PositionSet {
    positions: Vec<bool>,
    size: usize,
    len: usize,
}

impl PositionSet {
    pub fn new(size: usize) -> Self {
        Self {
            positions: vec![false; size * size],
            size,
            len: 0,
        }
    }

    pub fn insert(&mut self, position: (usize, usize)) {
        let index = self.index(position);
        self.positions[index] = true;
        self.len += 1;
    }

    pub fn contains(&self, position: (usize, usize)) -> bool {
        self.positions[self.index(position)]
    }

    pub fn len(&self) -> usize {
        self.len
    }

    fn index(&self, (x, y): (usize, usize)) -> usize {
        y * self.size + x
    }
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

pub fn part_one(input: &Input) -> (usize, PositionSet) {
    use Direction::*;
    let (x, y) = input.start;
    let (mut position, mut facing) = if x > 0 && [b'-', b'L', b'F'].contains(&input.lines[y][x - 1])
    {
        ((x - 1, y), West)
    } else if y > 0 && [b'|', b'7', b'F'].contains(&input.lines[y - 1][x]) {
        ((x, y - 1), North)
    } else {
        ((x + 1, y), East)
    };
    let mut steps = 1;
    let mut loop_positions = PositionSet::new(input.lines.len());
    loop_positions.insert(input.start);
    loop {
        let (x, y) = position;
        let pipe = input.lines[y][x];
        if pipe == b'S' {
            return (steps / 2, loop_positions);
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
        position = match facing {
            North => (x, y - 1),
            East => (x + 1, y),
            West => (x - 1, y),
            South => (x, y + 1),
        };
        loop_positions.insert(position);
        steps += 1;
    }
}

pub fn part_two(input: &Input, loop_positions: &PositionSet) -> usize {
    let total_positions = input.lines.len() * input.lines.len();
    let mut outside_position_count = 0;
    let mut start = (0, 0);
    while loop_positions.contains(start) {
        let (x, y) = start;
        start = if x + 1 == loop_positions.size {
            (0, y + 1)
        } else {
            (x + 1, y)
        }
    }
    let mut seen = PositionSet::new(loop_positions.size);
    let mut stack = vec![start];
    while let Some(position) = stack.pop() {
        if seen.contains(position) {
            continue;
        }
        seen.insert(position);
        outside_position_count += 1;
        let (x, y) = position;
        let adjacent_positions = [
            y.checked_sub(1).map(|y_sub| (x, y_sub)),
            x.checked_sub(1).map(|x_sub| (x_sub, y)),
            Some((x + 1, y)).filter(|&(x, _)| x < loop_positions.size),
            Some((x, y + 1)).filter(|&(_, y)| y < loop_positions.size),
        ];
        stack.extend(
            adjacent_positions
                .into_iter()
                .flatten()
                .filter(|&p| !seen.contains(p) && !loop_positions.contains(p)),
        );
    }
    total_positions - loop_positions.len() - outside_position_count
}

fn main() {
    let parse_result = parse_input(INPUT);
    match std::env::args().nth(1).as_deref() {
        Some("all") => {
            let (part_one, loop_positions) = part_one(&parse_result);
            println!("{part_one}");
            let part_two = part_two(&parse_result, &loop_positions);
            println!("{part_two}");
        }
        Some("parse") => {}
        Some("one") => {
            let (part_one, _) = part_one(&parse_result);
            println!("{part_one}");
        }
        Some("two") => {
            let (_, loop_positions) = part_one(&parse_result);
            let part_two = part_two(&parse_result, &loop_positions);
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
        let (part_one, _) = part_one(&input);
        assert_eq!(part_one, 6613);
    }

    #[test]
    fn test_part_two() {
        let input = parse_input(INPUT);
        let (_, loop_positions) = part_one(&input);
        assert_eq!(part_two(&input, &loop_positions), todo!());
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
        let (_, loop_positions) = part_one(&input);
        b.iter(|| part_two(black_box(&input), black_box(&loop_positions)));
    }
}
