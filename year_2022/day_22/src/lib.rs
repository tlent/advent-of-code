pub const INPUT: &str = include_str!("../input.txt");

const REGION_SIZE: usize = 50;

pub fn parse_input(input: &str) -> ([MapRegion; 6], Vec<PathStep>) {
    let (map_str, path_str) = input.split_once("\n\n").unwrap();
    let map_regions = parse_map_regions(map_str);
    let path = parse_path(path_str);
    (map_regions, path)
}

pub fn part_one(map_regions: &[MapRegion; 6], path: &[PathStep]) -> usize {
    let map = Map::link_as_wrapping(map_regions);
    solve(&map, path)
}

pub fn part_two(map_regions: &[MapRegion; 6], path: &[PathStep]) -> usize {
    let map = Map::link_as_cube(map_regions);
    solve(&map, path)
}

fn parse_map_regions(map_str: &str) -> [MapRegion; 6] {
    let lines = map_str
        .lines()
        .map(|l| l.bytes().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    lines
        .chunks_exact(REGION_SIZE)
        .enumerate()
        .flat_map(|(region_y, chunk)| {
            let width = chunk[0].len();
            (0..width / REGION_SIZE)
                .filter(|&region_x| {
                    let region_start_x = region_x * REGION_SIZE;
                    !chunk[0][region_start_x].is_ascii_whitespace()
                })
                .map(move |region_x| {
                    let position = (region_x, region_y);
                    let region_start_x = region_x * REGION_SIZE;
                    let region_end_x = region_start_x + REGION_SIZE;
                    let tiles = chunk
                        .iter()
                        .map(|bytes| {
                            bytes[region_start_x..region_end_x]
                                .iter()
                                .map(|b| match b {
                                    b'.' => Tile::Open,
                                    b'#' => Tile::Wall,
                                    _ => unreachable!(),
                                })
                                .collect::<Vec<_>>()
                                .try_into()
                                .unwrap()
                        })
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap();
                    MapRegion { position, tiles }
                })
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

fn parse_path(mut path_str: &str) -> Vec<PathStep> {
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
    path
}

fn solve(map: &Map, path: &[PathStep]) -> usize {
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

#[derive(Debug)]
pub struct Map<'a> {
    regions: &'a [MapRegion; 6],
    region_links: [RegionLinks; 6],
}

#[derive(Debug)]
pub struct MapRegion {
    position: (usize, usize),
    tiles: [[Tile; REGION_SIZE]; REGION_SIZE],
}

#[derive(Debug, Default)]
pub struct RegionLinks {
    up: usize,
    left: usize,
    right: usize,
    down: usize,
}

impl<'a> Map<'a> {
    fn link_as_wrapping(regions: &'a [MapRegion; 6]) -> Self {
        let mut region_links = Default::default();
        todo!();
        Self {
            regions,
            region_links,
        }
    }

    fn link_as_cube(regions: &[MapRegion; 6]) -> Self {
        todo!()
    }

    fn cursor(&self) -> MapCursor {
        MapCursor {
            map: self,
            region_index: 0,
            position: (0, 0),
        }
    }
}

pub struct MapCursor<'a> {
    map: &'a Map<'a>,
    region_index: usize,
    position: (usize, usize),
}

impl<'a> MapCursor<'a> {
    fn position(&self) -> (usize, usize) {
        let region = &self.map.regions[self.region_index];
        let (region_x, region_y) = region.position;
        let (cursor_x, cursor_y) = self.position;
        let x = region_x * REGION_SIZE + cursor_x;
        let y = region_y * REGION_SIZE + cursor_y;
        (x, y)
    }

    fn tile(&self) -> Tile {
        let region = &self.map.regions[self.region_index];
        let (x, y) = self.position;
        region.tiles[y][x]
    }

    fn next(&self, direction: Direction) -> Self {
        let region_links = &self.map.region_links[self.region_index];
        let (x, y) = self.position;
        let (region_index, position) = match direction {
            Direction::Up => {
                if y == 0 {
                    (region_links.up, (x, REGION_SIZE - 1))
                } else {
                    (self.region_index, (x, y - 1))
                }
            }
            Direction::Left => {
                if x == 0 {
                    (region_links.left, (REGION_SIZE - 1, y))
                } else {
                    (self.region_index, (x - 1, y))
                }
            }
            Direction::Right => {
                if x == REGION_SIZE - 1 {
                    (region_links.right, (0, y))
                } else {
                    (self.region_index, (x + 1, y))
                }
            }
            Direction::Down => {
                if y == REGION_SIZE - 1 {
                    (region_links.down, (x, 0))
                } else {
                    (self.region_index, (x, y + 1))
                }
            }
        };
        Self {
            map: self.map,
            region_index,
            position,
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
