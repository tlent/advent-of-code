use std::collections::VecDeque;

pub const INPUT: &str = include_str!("input.txt");

pub struct Grid {
    size: usize,
    heights: Vec<u8>,
    initial_position: Position,
    target_position: Position,
}

impl Grid {
    fn from_input(input: &str) -> Self {
        let size = input.lines().nth(0).unwrap().len();
        let bytes = input
            .lines()
            .flat_map(|line| line.bytes())
            .collect::<Vec<_>>();
        let initial_position = to_position(bytes.iter().position(|&b| b == b'S').unwrap(), size);
        let target_position = to_position(bytes.iter().position(|&b| b == b'E').unwrap(), size);
        let heights = bytes
            .into_iter()
            .map(|b| match b {
                b'S' => 0,
                b'E' => 25,
                b if b.is_ascii_lowercase() => b - b'a',
                _ => panic!("invalid height {b}"),
            })
            .collect();
        Grid {
            size,
            heights,
            initial_position,
            target_position,
        }
    }

    fn get(&self, position: Position) -> Option<u8> {
        if position.x >= self.size {
            return None;
        }
        self.heights.get(to_index(position, self.size)).copied()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn neighbors(&self) -> impl Iterator<Item = Position> {
        let x = self.x;
        let y = self.y;
        vec![
            x.checked_sub(1).map(|x| Position { x, y }),
            Some(Position { x: x + 1, y }),
            y.checked_sub(1).map(|y| Position { x, y }),
            Some(Position { x, y: y + 1 }),
        ]
        .into_iter()
        .filter_map(|option| option)
    }
}

pub fn parse_input(input: &str) -> Grid {
    Grid::from_input(input)
}

pub fn part_one(grid: &Grid) -> usize {
    find_shortest_path_len(grid, [grid.initial_position]).unwrap()
}

pub fn part_two(grid: &Grid) -> usize {
    let starting_positions = grid.heights.iter().enumerate().filter_map(|(i, &h)| {
        if h == 0 {
            Some(to_position(i, grid.size))
        } else {
            None
        }
    });
    find_shortest_path_len(grid, starting_positions).unwrap()
}

fn find_shortest_path_len<I>(grid: &Grid, starting_positions: I) -> Option<usize>
where
    I: IntoIterator<Item = Position>,
{
    let mut queue = starting_positions
        .into_iter()
        .map(|p| (p, 0))
        .collect::<VecDeque<_>>();
    let mut seen = vec![false; grid.size * grid.size];
    while let Some((position, steps)) = queue.pop_front() {
        if position == grid.target_position {
            return Some(steps);
        }
        let seen = &mut seen[to_index(position, grid.size)];
        if *seen {
            continue;
        }
        *seen = true;
        let height = grid.get(position).unwrap();
        let valid_neighbors = position.neighbors().filter_map(|n| {
            grid.get(n)
                .filter(|&h| h <= height + 1)
                .map(|_| (n, steps + 1))
        });
        queue.extend(valid_neighbors);
    }
    None
}

fn to_position(i: usize, size: usize) -> Position {
    Position {
        x: i % size,
        y: i / size,
    }
}

fn to_index(position: Position, size: usize) -> usize {
    position.y * size + position.x
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let grid = parse_input(INPUT);
        assert_eq!(part_one(&grid), 472);
    }

    #[test]
    fn test_part_two() {
        let grid = parse_input(INPUT);
        assert_eq!(part_two(&grid), 465);
    }
}
