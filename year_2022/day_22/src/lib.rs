pub const INPUT: &str = include_str!("../input.txt");

const REGION_SIZE: usize = 4;

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

impl RegionLinks {
    fn link_direction(&mut self, direction: Direction, id: usize) {
        match direction {
            Direction::Up => self.up = id,
            Direction::Left => self.left = id,
            Direction::Right => self.right = id,
            Direction::Down => self.down = id,
        }
    }
}

impl<'a> Map<'a> {
    fn link_as_wrapping(regions: &'a [MapRegion; 6]) -> Self {
        let row_count = regions.iter().map(|r| r.position.1).max().unwrap() + 1;
        let column_count = regions.iter().map(|r| r.position.0).max().unwrap() + 1;
        let mut rows = vec![vec![]; row_count];
        let mut columns = vec![vec![]; column_count];
        for (i, region) in regions.iter().enumerate() {
            let (column, row) = region.position;
            rows[row].push(i);
            columns[column].push(i);
        }
        let mut region_links: [RegionLinks; 6] = Default::default();
        for row in rows {
            let first = *row.first().unwrap();
            let last = *row.last().unwrap();
            region_links[first].left = last;
            region_links[last].right = first;
            for (left, right) in row.iter().zip(row.iter().skip(1)) {
                region_links[*left].right = *right;
                region_links[*right].left = *left;
            }
        }
        for column in columns {
            let first = *column.first().unwrap();
            let last = *column.last().unwrap();
            region_links[first].up = last;
            region_links[last].down = first;
            for (up, down) in column.iter().zip(column.iter().skip(1)) {
                region_links[*up].down = *down;
                region_links[*down].up = *up;
            }
        }
        Self {
            regions,
            region_links,
        }
    }

    fn link_as_cube(regions: &'a [MapRegion; 6]) -> Self {
        let mut region_links: [RegionLinks; 6] = Default::default();
        let mut linked_cubes = vec![vec![]; 6];
        let mut unlinked_region_edges = (0..6)
            .flat_map(|id| {
                Direction::iter().map(move |direction| {
                    let (x, y) = regions[id].position;
                    let (mut x, mut y) = (x as i32, y as i32);
                    match direction {
                        Direction::Up => y -= 1,
                        Direction::Left => x -= 1,
                        Direction::Right => x += 1,
                        Direction::Down => y += 1,
                    }
                    let position = (x, y);
                    (id, direction, position)
                })
            })
            .collect::<Vec<_>>();
        let connected_edge_pairs = unlinked_region_edges
            .iter()
            .flat_map(|&a @ (a_id, _, a_position)| {
                unlinked_region_edges
                    .iter()
                    .filter(move |&&(b_id, _, b_position)| {
                        let a_position = (a_position.0 as usize, a_position.1 as usize);
                        let b_position = (b_position.0 as usize, b_position.1 as usize);
                        a_id < b_id
                            && regions[a_id].position == b_position
                            && regions[b_id].position == a_position
                    })
                    .map(move |&b| (a, b))
            })
            .collect::<Vec<_>>();
        for (a @ (a_id, a_direction, ..), b @ (b_id, b_direction, ..)) in connected_edge_pairs {
            region_links[a_id].link_direction(a_direction, b_id);
            linked_cubes[a_id].push(b_id);
            region_links[b_id].link_direction(b_direction, a_id);
            linked_cubes[b_id].push(a_id);
            unlinked_region_edges.retain(|&e| e != a && e != b);
            dbg!((a, b));
        }
        while !unlinked_region_edges.is_empty() {
            let mut best_pair = (
                unlinked_region_edges.first().copied().unwrap(),
                unlinked_region_edges.last().copied().unwrap(),
            );
            let mut best_distance = manhattan_distance(best_pair.0 .2, best_pair.1 .2);
            for &a @ (a_id, _, a_position) in &unlinked_region_edges {
                for &b @ (b_id, _, b_position) in &unlinked_region_edges {
                    if a_id == b_id
                        || linked_cubes[a_id].contains(&b_id)
                        || linked_cubes[b_id].contains(&a_id)
                    {
                        continue;
                    }
                    let distance = manhattan_distance(a_position, b_position);
                    if distance < best_distance {
                        best_pair = (a, b);
                        best_distance = distance;
                    }
                }
            }
            dbg!(best_pair);
            let (a @ (a_id, a_direction, ..), b @ (b_id, b_direction, ..)) = best_pair;
            region_links[a_id].link_direction(a_direction, b_id);
            region_links[b_id].link_direction(b_direction, a_id);
            unlinked_region_edges.retain(|&e| e != a && e != b);
        }
        Self {
            regions,
            region_links,
        }
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
        let x = region_x * REGION_SIZE + cursor_x + 1;
        let y = region_y * REGION_SIZE + cursor_y + 1;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    fn iter() -> impl Iterator<Item = Direction> {
        [Self::Up, Self::Left, Self::Down, Self::Right].into_iter()
    }
}

fn manhattan_distance((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
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
