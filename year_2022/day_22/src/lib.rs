use std::cmp::Ordering;

pub const INPUT: &str = include_str!("../input.txt");

pub fn parse_input(input: &str) -> (Map, Vec<PathStep>) {
    let (map_str, mut path_str) = input.split_once("\n\n").unwrap();
    let map = Map::from_input(map_str);
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

pub fn part_one(map: &Map, path: &[PathStep]) -> usize {
    let mut cursor = map.cursor();
    let mut facing = Direction::Right;
    for step in path {
        match step {
            PathStep::Forward(n) => {
                for _ in 0..*n {
                    let next = cursor.next(facing);
                    if next.tile() == Tile::Wall {
                        break;
                    }
                    cursor = next;
                }
            }
            PathStep::Left => facing = facing.turn_left(),
            PathStep::Right => facing = facing.turn_right(),
        }
    }
    let (column, row) = cursor.position();
    1000 * row
        + 4 * column
        + match facing {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        }
}

pub fn part_two(map: &Map, path: &[PathStep]) -> usize {
    todo!()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Open,
    Wall,
}

#[derive(Debug)]
pub enum PathStep {
    Forward(usize),
    Left,
    Right,
}

pub struct Map(Vec<MapTile>);

impl Map {
    fn from_input(input: &str) -> Self {
        let lines = input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut tiles: Vec<MapTile> = vec![];
        let mut column_prevs: Vec<Option<usize>> = vec![];
        let mut column_segment_starts: Vec<Option<usize>> = vec![];
        for (i, line) in lines.iter().enumerate() {
            match line.len().cmp(&column_prevs.len()) {
                Ordering::Less => {
                    let iter = column_segment_starts[line.len()..]
                        .iter_mut()
                        .zip(column_prevs[line.len()..].iter_mut());
                    for (start_mut, prev_mut) in iter {
                        if let Some(start) = start_mut.take() {
                            let prev = prev_mut.take().unwrap();
                            tiles[start].next_up = prev;
                            tiles[prev].next_down = start;
                        }
                    }
                }
                Ordering::Greater => {
                    column_prevs.resize_with(line.len(), Default::default);
                    column_segment_starts.resize_with(line.len(), Default::default);
                }
                Ordering::Equal => {}
            }
            let mut row_prev: Option<usize> = None;
            let mut row_segment_start: Option<usize> = None;
            for (j, &c) in line.iter().enumerate() {
                let column_prev = &mut column_prevs[j];
                let column_segment_start = &mut column_segment_starts[j];
                if c == ' ' {
                    if let Some(start) = row_segment_start.take() {
                        let prev = row_prev.take().unwrap();
                        tiles[start].next_left = prev;
                        tiles[prev].next_right = start;
                    }
                    if let Some(start) = column_segment_start.take() {
                        let prev = column_prev.take().unwrap();
                        tiles[start].next_up = prev;
                        tiles[prev].next_down = start;
                    }
                    continue;
                }
                let tile = match c {
                    '.' => Tile::Open,
                    '#' => Tile::Wall,
                    _ => panic!("invalid map char {c}"),
                };
                let map_tile = MapTile {
                    position: (j + 1, i + 1),
                    tile,
                    next_left: row_prev.unwrap_or_default(),
                    next_up: column_prev.unwrap_or_default(),
                    next_right: Default::default(),
                    next_down: Default::default(),
                };
                let index = tiles.len();
                tiles.push(map_tile);
                if let Some(p) = row_prev {
                    tiles[p].next_right = index;
                }
                if let Some(p) = *column_prev {
                    tiles[p].next_down = index;
                }
                row_prev = Some(index);
                if row_segment_start.is_none() {
                    row_segment_start = Some(index);
                }
                *column_prev = Some(index);
                if column_segment_start.is_none() {
                    *column_segment_start = Some(index);
                }
            }
            let start = row_segment_start.unwrap();
            let prev = row_prev.unwrap();
            tiles[start].next_left = prev;
            tiles[prev].next_right = start;
        }
        for (start, prev) in column_segment_starts.into_iter().zip(column_prevs) {
            if let Some(start) = start {
                let prev = prev.unwrap();
                tiles[start].next_up = prev;
                tiles[prev].next_down = start;
            }
        }

        Self(tiles)
    }

    fn cursor(&self) -> MapCursor {
        MapCursor {
            map: self,
            current_tile: &self.0[0],
        }
    }
}

pub struct MapTile {
    position: (usize, usize),
    tile: Tile,
    next_up: usize,
    next_left: usize,
    next_right: usize,
    next_down: usize,
}

pub struct MapCursor<'a> {
    map: &'a Map,
    current_tile: &'a MapTile,
}

impl<'a> MapCursor<'a> {
    fn position(&self) -> (usize, usize) {
        self.current_tile.position
    }

    fn tile(&self) -> Tile {
        self.current_tile.tile
    }

    fn next(&self, direction: Direction) -> Self {
        let index = match direction {
            Direction::Up => self.current_tile.next_up,
            Direction::Left => self.current_tile.next_left,
            Direction::Right => self.current_tile.next_right,
            Direction::Down => self.current_tile.next_down,
        };
        Self {
            map: self.map,
            current_tile: &self.map.0[index],
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Left,
    Right,
    Down,
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let (map, path) = parse_input(INPUT);
        assert_eq!(part_one(&map, &path), 65368);
    }

    #[test]
    fn test_part_two() {
        todo!()
    }
}
