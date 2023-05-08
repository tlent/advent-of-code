pub const INPUT: &str = include_str!("../input.txt");

const CUBE_FACE_SIZE: usize = 50;

pub fn parse_input(input: &str) -> (MapRegions, Vec<PathStep>) {
    let (map_str, mut path_str) = input.split_once("\n\n").unwrap();
    let map_regions = MapRegions::from_input(map_str);
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
    (map_regions, path)
}

pub fn part_one(map_regions: &MapRegions, path: &[PathStep]) -> usize {
    let map = map_regions.link_as_wrapping();
    solve(&map, path)
}

pub fn part_two(map_regions: &MapRegions, path: &[PathStep]) -> usize {
    let map = map_regions.link_as_cube();
    solve(&map, path)
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
pub struct MapRegionTile {
    position: (usize, usize),
    tile: Tile,
    next_up: Option<usize>,
    next_left: Option<usize>,
    next_right: Option<usize>,
    next_down: Option<usize>,
}

pub struct MapRegion {
    position: (u8, u8),
    tiles: [[MapRegionTile; CUBE_FACE_SIZE]; CUBE_FACE_SIZE],
}

pub struct MapRegions(Vec<MapRegion>);

impl MapRegions {
    fn from_input(input: &str) -> Self {
        let lines = input
            .lines()
            .map(|l| l.bytes().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let regions = lines
            .chunks_exact(CUBE_FACE_SIZE)
            .enumerate()
            .flat_map(|(region_y, rows)| {
                let width = rows[0].len();
                (0..width / CUBE_FACE_SIZE).filter_map(move |region_x| {
                    let region_start_x = region_x * CUBE_FACE_SIZE;
                    let region_end_x = region_start_x + CUBE_FACE_SIZE;
                    if rows[0][region_start_x].is_ascii_whitespace() {
                        return None;
                    }
                    let region_position = (region_x as u8, region_y as u8);
                    let tiles = rows
                        .iter()
                        .enumerate()
                        .map(|(i, row)| {
                            row[region_start_x..region_end_x]
                                .iter()
                                .enumerate()
                                .map(|(j, &b)| {
                                    let tile = match b {
                                        b'.' => Tile::Open,
                                        b'#' => Tile::Wall,
                                        _ => unreachable!(),
                                    };
                                    let position = (j + 1, i + 1);
                                    let next_left =
                                        j.checked_sub(1).map(|j| i * CUBE_FACE_SIZE + j);
                                    let next_up = i.checked_sub(1).map(|i| i * CUBE_FACE_SIZE + j);
                                    let next_right = if j + 1 < CUBE_FACE_SIZE {
                                        Some(i * CUBE_FACE_SIZE + j + 1)
                                    } else {
                                        None
                                    };
                                    let next_down = if i + 1 < CUBE_FACE_SIZE {
                                        Some((i + 1) * CUBE_FACE_SIZE + j)
                                    } else {
                                        None
                                    };
                                    MapRegionTile {
                                        tile,
                                        position,
                                        next_left,
                                        next_up,
                                        next_right,
                                        next_down,
                                    }
                                })
                                .collect::<Vec<_>>()
                                .try_into()
                                .unwrap()
                        })
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap();
                    let region = MapRegion {
                        position: region_position,
                        tiles,
                    };
                    Some(region)
                })
            })
            .collect();
        Self(regions)
    }

    fn link_as_wrapping(&self) -> Map {
        todo!()
    }
    fn link_as_cube(&self) -> Map {
        todo!()
    }
}

pub struct Map(Vec<MapTile>);

impl Map {
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
