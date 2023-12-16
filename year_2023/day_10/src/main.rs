#![feature(test)]
extern crate test;

const INPUT: &str = include_str!("../input.txt");

struct Input<'a> {
    start: (usize, usize),
    lines: Vec<&'a [u8]>,
    tile_row_size: usize,
    tile_column_size: usize,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    West,
    South,
}

struct PositionSet {
    data: Vec<bool>,
    len: usize,
    row_size: usize,
}

impl PositionSet {
    fn new(row_size: usize, column_size: usize) -> Self {
        Self {
            data: vec![false; row_size * column_size],
            len: 0,
            row_size,
        }
    }

    fn contains(&self, position: (usize, usize)) -> bool {
        self.data[self.index(position)]
    }

    fn insert(&mut self, position: (usize, usize)) {
        let index = self.index(position);
        self.data[index] = true;
        self.len += 1;
    }

    fn len(&self) -> usize {
        self.len
    }

    fn index(&self, (x, y): (usize, usize)) -> usize {
        y * self.row_size + x
    }
}

fn parse_input(input: &str) -> Input {
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
    let tile_row_size = lines[0].len();
    let tile_column_size = lines.len();
    Input {
        start,
        lines,
        tile_row_size,
        tile_column_size,
    }
}

fn find_pipe_tiles(input: &Input) -> PositionSet {
    use Direction::*;
    let mut pipe_tiles = PositionSet::new(input.tile_row_size, input.tile_column_size);
    pipe_tiles.insert(input.start);
    let (mut x, mut y) = input.start;
    let mut facing = if x > 0 && [b'-', b'L', b'F'].contains(&input.lines[y][x - 1]) {
        x -= 1;
        West
    } else if y > 0 && [b'|', b'7', b'F'].contains(&input.lines[y - 1][x]) {
        y -= 1;
        North
    } else {
        x += 1;
        East
    };
    let mut tile = input.lines[y][x];
    while tile != b'S' {
        pipe_tiles.insert((x, y));
        facing = match (tile, facing) {
            (b'|', _) | (b'-', _) => facing,
            (b'L', South) => East,
            (b'L', West) => North,
            (b'J', South) => West,
            (b'J', East) => North,
            (b'7', North) => West,
            (b'7', East) => South,
            (b'F', North) => East,
            (b'F', West) => South,
            (p, _) => panic!("invalid pipe {p}"),
        };
        match facing {
            North => y -= 1,
            East => x += 1,
            West => x -= 1,
            South => y += 1,
        };
        tile = input.lines[y][x];
    }
    pipe_tiles
}

fn part_one(pipe_tiles: &PositionSet) -> usize {
    pipe_tiles.len() / 2
}

fn part_two(input: &Input, pipe_tiles: &PositionSet) -> usize {
    let (mut x, mut y) = input.start;
    if x > 0 && ![b'-', b'L', b'F'].contains(&input.lines[y][x - 1]) {
        x -= 1;
    } else if y > 0 && ![b'|', b'7', b'F'].contains(&input.lines[y - 1][x]) {
        y -= 1;
    } else {
        x += 1;
    };
    let mut unenclosed_tile_count = 0;
    let mut stack = vec![(x, y)];
    let mut seen = PositionSet::new(input.tile_row_size, input.tile_column_size);
    while let Some(position @ (x, y)) = stack.pop() {
        if seen.contains(position) {
            continue;
        }
        seen.insert(position);
        if pipe_tiles.contains(position) {
        } else {
            unenclosed_tile_count += 1;
            let adjacent = [
                x.checked_sub(1).map(|x_sub| (x_sub, y)),
                y.checked_sub(1).map(|y_sub| (x, y_sub)),
                Some(x + 1)
                    .filter(|&x_plus| x_plus < input.tile_row_size)
                    .map(|x_plus| (x_plus, y)),
                Some(y + 1)
                    .filter(|&y_plus| y_plus < input.tile_column_size)
                    .map(|y_plus| (x, y_plus)),
            ];
            stack.extend(
                adjacent
                    .into_iter()
                    .flatten()
                    .filter(|&position| !seen.contains(position)),
            );
        }
    }
    input.tile_row_size * input.tile_column_size - pipe_tiles.len() - unenclosed_tile_count
}

fn main() {
    let input = parse_input(INPUT);
    let pipe_tiles = find_pipe_tiles(&input);
    match std::env::args().nth(1).as_deref() {
        Some("all") => {
            let part_one = part_one(&pipe_tiles);
            println!("{part_one}");
            let part_two = part_two(&input, &pipe_tiles);
            println!("{part_two}");
        }
        Some("parse") => {}
        Some("one") => {
            let part_one = part_one(&pipe_tiles);
            println!("{part_one}");
        }
        Some("two") => {
            let part_two = part_two(&input, &pipe_tiles);
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
        let pipe_tiles = find_pipe_tiles(&input);
        assert_eq!(part_one(&pipe_tiles), 6613);
    }

    #[test]
    fn test_part_two() {
        let input = parse_input(INPUT);
        let pipe_tiles = find_pipe_tiles(&input);
        assert_eq!(part_two(&input, &pipe_tiles), 0);
    }

    #[bench]
    fn bench_parse_input(b: &mut Bencher) {
        b.iter(|| parse_input(black_box(INPUT)));
    }

    #[bench]
    fn bench_find_pipe_tiles(b: &mut Bencher) {
        let input = parse_input(INPUT);
        b.iter(|| find_pipe_tiles(black_box(&input)));
    }

    #[bench]
    fn bench_part_one(b: &mut Bencher) {
        let input = parse_input(INPUT);
        let pipe_tiles = find_pipe_tiles(&input);
        b.iter(|| part_one(black_box(&pipe_tiles)));
    }

    #[bench]
    fn bench_part_two(b: &mut Bencher) {
        let input = parse_input(INPUT);
        let pipe_tiles = find_pipe_tiles(&input);
        b.iter(|| part_two(black_box(&input), black_box(&pipe_tiles)));
    }
}
