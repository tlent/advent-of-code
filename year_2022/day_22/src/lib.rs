pub const INPUT: &str = include_str!("../input.txt");

pub struct Map(Vec<Vec<Tile>>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    None,
    Open,
    Wall,
}

#[derive(Debug)]
pub enum PathStep {
    Forward(usize),
    Left,
    Right,
}

pub fn parse_input(input: &str) -> (Map, Vec<PathStep>) {
    let (map_str, mut path_str) = input.split_once("\n\n").unwrap();
    let tiles = map_str
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    ' ' => Tile::None,
                    '.' => Tile::Open,
                    '#' => Tile::Wall,
                    _ => panic!("invalid map char"),
                })
                .collect()
        })
        .collect();
    let map = Map(tiles);
    path_str = path_str.trim();
    let mut path = vec![];
    while !path_str.is_empty() {
        let end_index = path_str.find(['L', 'R']).unwrap_or(path_str.len());
        let forward = PathStep::Forward(path_str[..end_index].parse().unwrap());
        path.push(forward);
        path_str = &path_str[end_index..];
        if !path_str.is_empty() {
            let turn = match path_str.chars().next().unwrap() {
                'L' => PathStep::Left,
                'R' => PathStep::Right,
                _ => unreachable!(),
            };
            path.push(turn);
            path_str = &path_str[1..];
        }
    }
    (map, path)
}

pub fn part_one(map: &Map, path: &[PathStep]) -> i32 {
    let start_x = map.first_open_tile_position(1) as i32;
    let mut position = (start_x, 1);
    dbg!(position);
    let mut facing = Direction::Right;
    for step in path {
        match dbg!(step) {
            PathStep::Forward(n) => position = dbg!(map.find_new_position(position, facing, *n)),
            PathStep::Left => facing = dbg!(facing.turn_left()),
            PathStep::Right => facing = dbg!(facing.turn_right()),
        }
    }
    let (column, row) = position;
    dbg!(row, column, facing);
    1000 * row
        + 4 * column
        + match facing {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        }
}

pub fn part_two(map: &Map, path: &[PathStep]) -> i32 {
    todo!()
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Left,
    Right,
    Down,
}

impl Map {
    fn first_open_tile_position(&self, row: usize) -> usize {
        self.0[row - 1]
            .iter()
            .position(|&t| t == Tile::Open)
            .unwrap()
            + 1
    }

    fn find_new_position(
        &self,
        (mut x, mut y): (i32, i32),
        facing: Direction,
        steps: usize,
    ) -> (i32, i32) {
        let rows = &self.0;
        let (dx, dy) = match facing {
            Direction::Up => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
        };
        for _ in 0..steps {
            let next_x = x + dx;
            let next_y = y + dy;
            let next_tile = if self.in_bounds((next_x, next_y)) {
                rows[next_y as usize][next_x as usize]
            } else {
                Tile::None
            };
            dbg!((next_x, next_y));
            match next_tile {
                Tile::Open => {
                    x = next_x;
                    y = next_y;
                }
                Tile::Wall => break,
                Tile::None => {
                    // TODO: Move this outside of match, only reach match with correct next values
                    let (dx, dy) = match facing.turn_around() {
                        Direction::Up => (0, -1),
                        Direction::Left => (-1, 0),
                        Direction::Right => (1, 0),
                        Direction::Down => (0, 1),
                    };
                    let mut next_x = x + dx;
                    let mut next_y = y + dy;
                    while self.in_bounds((next_x, next_y))
                        && rows[next_y as usize][next_x as usize] != Tile::None
                    {
                        x = next_x;
                        y = next_y;
                        next_x = x + dx;
                        next_y = y + dy;
                    }
                }
            }
        }
        (x, y)
    }

    fn in_bounds(&self, (x, y): (i32, i32)) -> bool {
        let rows = &self.0;
        (0..rows.len() as i32).contains(&y) && (0..rows[y as usize].len() as i32).contains(&x)
    }
}

impl Direction {
    fn turn_left(&self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Right => Self::Up,
            Self::Down => Self::Right,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
        }
    }

    fn turn_around(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Down => Self::Up,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        todo!()
    }

    #[test]
    fn test_part_two() {
        todo!()
    }
}
