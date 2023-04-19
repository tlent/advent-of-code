use rustc_hash::FxHashMap as HashMap;
use std::{
    array, cmp,
    iter::{Cycle, Enumerate, Peekable},
    slice,
};

pub const INPUT: &str = include_str!("../input.txt");

type Point = (usize, usize);

#[derive(Debug)]
pub enum Motion {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Shape {
    HorizontalLine,
    Plus,
    ReverseL,
    VerticalLine,
    Square,
}

impl Shape {
    fn points(&self) -> Box<[Point]> {
        match self {
            Shape::HorizontalLine => Box::new([(0, 0), (1, 0), (2, 0), (3, 0)]),
            Shape::Plus => Box::new([(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)]),
            Shape::ReverseL => Box::new([(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)]),
            Shape::VerticalLine => Box::new([(0, 0), (0, 1), (0, 2), (0, 3)]),
            Shape::Square => Box::new([(0, 0), (0, 1), (1, 0), (1, 1)]),
        }
    }

    fn width(&self) -> u8 {
        match self {
            Shape::HorizontalLine => 4,
            Shape::Plus => 3,
            Shape::ReverseL => 3,
            Shape::VerticalLine => 1,
            Shape::Square => 2,
        }
    }

    fn height(&self) -> u8 {
        match self {
            Shape::HorizontalLine => 1,
            Shape::Plus => 3,
            Shape::ReverseL => 3,
            Shape::VerticalLine => 4,
            Shape::Square => 2,
        }
    }
}

const COLUMNS: usize = 7;
struct Simulation<'a> {
    columns: [Vec<bool>; COLUMNS],
    max_height: usize,
    shapes_iter: Peekable<Cycle<array::IntoIter<Shape, 5>>>,
    motions_iter: Peekable<Cycle<Enumerate<slice::Iter<'a, Motion>>>>,
}

impl<'a> Simulation<'a> {
    fn new(motions: &'a [Motion]) -> Self {
        let shapes = [
            Shape::HorizontalLine,
            Shape::Plus,
            Shape::ReverseL,
            Shape::VerticalLine,
            Shape::Square,
        ];
        Self {
            columns: Default::default(),
            max_height: 0,
            shapes_iter: shapes.into_iter().cycle().peekable(),
            motions_iter: motions.iter().enumerate().cycle().peekable(),
        }
    }

    fn peek_next_shape(&mut self) -> Shape {
        self.shapes_iter.peek().copied().unwrap()
    }

    fn peek_next_motion_index(&mut self) -> usize {
        self.motions_iter.peek().map(|(index, _)| *index).unwrap()
    }

    fn offsets(&self) -> Vec<usize> {
        self.columns
            .iter()
            .map(|column| self.max_height - column.iter().rposition(|&v| v).unwrap_or(0))
            .collect::<Vec<_>>()
    }

    fn drop_next_rock(&mut self) {
        let shape = self.shapes_iter.next().unwrap();
        let min_len = self.max_height + 3 + shape.height() as usize;
        if self.columns[0].len() < min_len {
            for column in self.columns.iter_mut() {
                column.resize(min_len, false);
            }
        }
        let (mut x, mut y) = (2usize, self.max_height + 3);
        loop {
            let (_, motion) = self.motions_iter.next().unwrap();
            let next_x = match motion {
                Motion::Left => x.saturating_sub(1),
                Motion::Right => cmp::min(x + 1, COLUMNS - shape.width() as usize),
            };
            let blocked = shape
                .points()
                .iter()
                .any(|(x_offset, y_offset)| self.columns[next_x + x_offset][y + y_offset]);
            if !blocked {
                x = next_x;
            }
            if y == 0 {
                break;
            }
            let next_y = y - 1;
            let blocked = shape
                .points()
                .iter()
                .any(|(x_offset, y_offset)| self.columns[x + x_offset][next_y + y_offset]);
            if blocked {
                break;
            }
            y = next_y;
        }
        for (x_offset, y_offset) in shape.points().iter() {
            self.columns[x + x_offset][y + y_offset] = true;
        }
        self.max_height = cmp::max(self.max_height, y + shape.height() as usize);
    }
}

pub fn parse_input(input: &str) -> Vec<Motion> {
    input
        .trim()
        .bytes()
        .map(|b| match b {
            b'<' => Motion::Left,
            b'>' => Motion::Right,
            _ => panic!("Unknown motion"),
        })
        .collect::<Vec<_>>()
}

pub fn part_one(motions: &[Motion]) -> u64 {
    let mut simulation = Simulation::new(motions);
    for _ in 0..2022 {
        simulation.drop_next_rock();
    }
    simulation.max_height as u64
}

pub fn part_two(motions: &[Motion]) -> u64 {
    const ROCK_COUNT: usize = 1_000_000_000_000;
    let mut simulation = Simulation::new(motions);
    let mut max_heights = vec![];
    let mut seen = HashMap::default();
    for i in 0..ROCK_COUNT {
        max_heights.push(simulation.max_height);
        let shape = simulation.peek_next_shape();
        let motion_index = simulation.peek_next_motion_index();
        let offsets = simulation.offsets();
        let key = (shape, motion_index, offsets);
        if let Some(&cycle_start) = seen.get(&key) {
            let cycle_end_height = simulation.max_height as u64;
            let cycle_end = i;
            let cycle_len = cycle_end - cycle_start;
            let remaining_rock_count = ROCK_COUNT - cycle_end;
            let remaining_cycles = (remaining_rock_count / cycle_len) as u64;
            let remainder = remaining_rock_count % cycle_len;
            let cycle_start_height = max_heights[cycle_start] as u64;
            let cycle_height_diff = cycle_end_height - cycle_start_height;
            let remainder_height = max_heights[cycle_start + remainder] as u64;
            let remainder_height_diff = remainder_height - cycle_start_height;
            return cycle_end_height + cycle_height_diff * remaining_cycles + remainder_height_diff;
        }
        seen.insert(key, i);
        simulation.drop_next_rock();
    }
    panic!("no cycle")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let motions = parse_input(INPUT);
        assert_eq!(part_one(&motions), 3130);
    }

    #[test]
    fn test_part_two() {
        let motions = parse_input(INPUT);
        assert_eq!(part_two(&motions), 1_556_521_739_139);
    }
}
