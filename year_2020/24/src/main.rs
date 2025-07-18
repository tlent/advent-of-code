use std::collections::HashMap;
use std::collections::HashSet;

static INPUT: &str = include_str!("../input");

fn main() {
    let directions = parse_input(INPUT);
    let black_tiles = part_one(&directions);
    println!("part one: {}", black_tiles.len());
    println!("part two: {}", part_two(&black_tiles));
}

enum Direction {
    East,
    NorthEast,
    SouthEast,
    West,
    NorthWest,
    SouthWest,
}

fn parse_input(input: &str) -> Vec<Vec<Direction>> {
    input
        .lines()
        .map(|l| {
            let mut directions = vec![];
            let mut chars = l.chars();
            while let Some(c) = chars.next() {
                directions.push(match c {
                    'n' => match chars.next() {
                        Some('e') => Direction::NorthEast,
                        Some('w') => Direction::NorthWest,
                        _ => panic!("invalid direction"),
                    },
                    's' => match chars.next() {
                        Some('e') => Direction::SouthEast,
                        Some('w') => Direction::SouthWest,
                        _ => panic!("invalid direction"),
                    },
                    'e' => Direction::East,
                    'w' => Direction::West,
                    _ => panic!("invalid direction"),
                });
            }
            directions
        })
        .collect()
}

fn part_one(directions: &[Vec<Direction>]) -> Vec<(i32, i32)> {
    let flips = directions.iter().map(|steps| {
        steps.iter().fold((0, 0), |(x, y), d| match d {
            Direction::East => (x + 2, y),
            Direction::West => (x - 2, y),
            Direction::NorthEast => (x + 1, y + 1),
            Direction::NorthWest => (x - 1, y + 1),
            Direction::SouthEast => (x + 1, y - 1),
            Direction::SouthWest => (x - 1, y - 1),
        })
    });
    let mut black_tiles = HashSet::new();
    for f in flips {
        if !black_tiles.insert(f) {
            black_tiles.remove(&f);
        }
    }
    black_tiles.iter().cloned().collect()
}

fn adjacent_coords((x, y): (i32, i32)) -> Vec<(i32, i32)> {
    vec![
        (x + 2, y),
        (x - 2, y),
        (x + 1, y + 1),
        (x - 1, y + 1),
        (x + 1, y - 1),
        (x - 1, y - 1),
    ]
}

#[derive(Debug, Clone, Copy)]
struct Tile {
    adjacent_black_count: u32,
    is_black: bool,
}

impl Tile {
    fn black() -> Self {
        Tile {
            adjacent_black_count: 0,
            is_black: true,
        }
    }

    fn white() -> Self {
        Tile {
            adjacent_black_count: 0,
            is_black: false,
        }
    }
}

fn part_two(initial_black_tiles: &[(i32, i32)]) -> usize {
    let mut tiles: HashMap<(i32, i32), Tile> = initial_black_tiles
        .iter()
        .map(|&c| (c, Tile::black()))
        .collect();
    let tile_coords: Vec<_> = tiles.keys().copied().collect();
    for c in tile_coords {
        for a in adjacent_coords(c).into_iter() {
            let is_black = tiles.get(&a).map(|t| t.is_black).unwrap_or(false);
            if is_black {
                tiles.get_mut(&c).unwrap().adjacent_black_count += 1;
            } else {
                tiles
                    .entry(a)
                    .or_insert_with(Tile::white)
                    .adjacent_black_count += 1;
            }
        }
    }
    for _ in 0..100 {
        let current_tiles = tiles.clone();
        for (c, t) in current_tiles.into_iter() {
            if t.is_black && (t.adjacent_black_count == 0 || t.adjacent_black_count > 2) {
                tiles.get_mut(&c).unwrap().is_black = false;
                for a in adjacent_coords(c).iter() {
                    tiles.get_mut(a).unwrap().adjacent_black_count -= 1;
                }
            }
            if !t.is_black && t.adjacent_black_count == 2 {
                tiles.get_mut(&c).unwrap().is_black = true;
                for a in adjacent_coords(c).into_iter() {
                    tiles
                        .entry(a)
                        .or_insert_with(Tile::white)
                        .adjacent_black_count += 1;
                }
            }
        }
        tiles.retain(|_, &mut t| t.is_black || t.adjacent_black_count > 0);
    }
    tiles.values().filter(|t| t.is_black).count()
}
