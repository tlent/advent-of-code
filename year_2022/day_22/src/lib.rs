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
    for step in path {
        match step {
            PathStep::Forward(n) => {
                for _ in 0..*n {
                    let next = cursor.next();
                    if next.tile() == Tile::Wall {
                        break;
                    }
                    cursor = next;
                }
            }
            PathStep::Left => cursor.turn_left(),
            PathStep::Right => cursor.turn_right(),
        }
    }
    let (column, row, facing) = cursor.position();
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

#[derive(Debug, Clone, Default)]
pub struct RegionLinks {
    up: (usize, Direction),
    left: (usize, Direction),
    right: (usize, Direction),
    down: (usize, Direction),
}

impl RegionLinks {
    fn link_edge(&mut self, edge: Direction, other_id: usize, other_edge: Direction) {
        match edge {
            Direction::Up => self.up = (other_id, other_edge),
            Direction::Left => self.left = (other_id, other_edge),
            Direction::Right => self.right = (other_id, other_edge),
            Direction::Down => self.down = (other_id, other_edge),
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
            region_links[first].link_edge(Direction::Left, last, Direction::Right);
            region_links[last].link_edge(Direction::Right, first, Direction::Left);
            for (&left, &right) in row.iter().zip(row.iter().skip(1)) {
                region_links[left].link_edge(Direction::Right, right, Direction::Left);
                region_links[right].link_edge(Direction::Left, left, Direction::Right);
            }
        }
        for column in columns {
            let first = *column.first().unwrap();
            let last = *column.last().unwrap();
            region_links[first].link_edge(Direction::Up, last, Direction::Down);
            region_links[last].link_edge(Direction::Down, first, Direction::Up);
            for (&up, &down) in column.iter().zip(column.iter().skip(1)) {
                region_links[up].link_edge(Direction::Down, down, Direction::Up);
                region_links[down].link_edge(Direction::Up, first, Direction::Down);
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
            region_links[a_id].link_edge(a_direction, b_id, b_direction);
            linked_cubes[a_id].push(b_id);
            region_links[b_id].link_edge(b_direction, a_id, a_direction);
            linked_cubes[b_id].push(a_id);
            unlinked_region_edges.retain(|&e| e != a && e != b);
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
            let (a @ (a_id, a_direction, ..), b @ (b_id, b_direction, ..)) = best_pair;
            region_links[a_id].link_edge(a_direction, b_id, b_direction);
            linked_cubes[a_id].push(b_id);
            region_links[b_id].link_edge(b_direction, a_id, a_direction);
            linked_cubes[b_id].push(a_id);
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
            facing: Direction::Right,
        }
    }
}

pub struct MapCursor<'a> {
    map: &'a Map<'a>,
    region_index: usize,
    position: (usize, usize),
    facing: Direction,
}

impl<'a> MapCursor<'a> {
    fn position(&self) -> (usize, usize, Direction) {
        let region = &self.map.regions[self.region_index];
        let (region_x, region_y) = region.position;
        let (cursor_x, cursor_y) = self.position;
        let x = region_x * REGION_SIZE + cursor_x + 1;
        let y = region_y * REGION_SIZE + cursor_y + 1;
        (x, y, self.facing)
    }

    fn tile(&self) -> Tile {
        let region = &self.map.regions[self.region_index];
        let (x, y) = self.position;
        region.tiles[y][x]
    }

    fn turn_left(&mut self) {
        self.facing = self.facing.turn_left();
    }

    fn turn_right(&mut self) {
        self.facing = self.facing.turn_right();
    }

    fn next(&self) -> Self {
        let region_links = &self.map.region_links[self.region_index];
        let Self {
            mut region_index,
            position: (mut x, mut y),
            mut facing,
            ..
        } = *self;
        match facing {
            Direction::Up => {
                if y == 0 {
                    let (linked_region, edge) = region_links.up;
                    region_index = linked_region;
                    facing = edge.reverse();
                    (x, y) = match edge {
                        Direction::Down => (x, REGION_SIZE - 1),
                        Direction::Up => (REGION_SIZE - x - 1, 0),
                        Direction::Left => (0, x),
                        Direction::Right => (REGION_SIZE - 1, REGION_SIZE - x - 1),
                    };
                } else {
                    y -= 1;
                }
            }
            Direction::Left => {
                if x == 0 {
                    let (linked_region, edge) = region_links.left;
                    region_index = linked_region;
                    facing = edge.reverse();
                    (x, y) = match edge {
                        Direction::Down => (REGION_SIZE - y - 1, REGION_SIZE - 1),
                        Direction::Up => (y, 0),
                        Direction::Left => (0, y),
                        Direction::Right => (REGION_SIZE - 1, y),
                    };
                } else {
                    x -= 1;
                }
            }
            Direction::Right => {
                if x == REGION_SIZE - 1 {
                    let (linked_region, edge) = region_links.right;
                    region_index = linked_region;
                    facing = edge.reverse();
                    (x, y) = match edge {
                        Direction::Down => (y, REGION_SIZE - 1),
                        Direction::Up => (REGION_SIZE - y - 1, 0),
                        Direction::Left => (0, y),
                        Direction::Right => (REGION_SIZE - 1, y),
                    };
                } else {
                    x += 1;
                }
            }
            Direction::Down => {
                if y == REGION_SIZE - 1 {
                    let (linked_region, edge) = region_links.down;
                    region_index = linked_region;
                    facing = edge.reverse();
                    (x, y) = match edge {
                        Direction::Down => (REGION_SIZE - x - 1, REGION_SIZE - 1),
                        Direction::Up => (x, 0),
                        Direction::Left => (0, REGION_SIZE - x - 1),
                        Direction::Right => (REGION_SIZE - 1, x),
                    };
                } else {
                    y += 1;
                }
            }
        };
        Self {
            map: self.map,
            position: (x, y),
            region_index,
            facing,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Direction {
    #[default]
    Up,
    Left,
    Right,
    Down,
}

impl Direction {
    fn turn_left(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn reverse(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
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
