use std::cmp;
use std::ops::RangeInclusive;

use crate::HashMap;

pub const INPUT: &str = include_str!("input.txt");

const SPAWN_POINT: Point = (500, 0);

#[derive(Debug, Clone)]
pub enum Material {
    Rock,
    Sand,
}

type Point = (usize, usize);

#[derive(Debug, Clone)]
pub struct World {
    map: HashMap<Point, Material>,
    x_bounds: RangeInclusive<usize>,
    y_bounds: RangeInclusive<usize>,
}

impl World {
    fn from_input(input: &str) -> Self {
        let mut map = HashMap::default();
        for line in input.lines() {
            let points: Vec<_> = line
                .split(" -> ")
                .map(|point_str| {
                    let (left, right) = point_str.split_once(',').unwrap();
                    let x: usize = left.parse().unwrap();
                    let y: usize = right.parse().unwrap();
                    (x, y)
                })
                .collect();
            for (&(left_x, left_y), &(right_x, right_y)) in points.iter().zip(points.iter().skip(1))
            {
                if left_x != right_x {
                    let start = cmp::min(left_x, right_x);
                    let end = cmp::max(left_x, right_x);
                    for x in start..=end {
                        map.insert((x, left_y), Material::Rock);
                    }
                } else {
                    let start = cmp::min(left_y, right_y);
                    let end = cmp::max(left_y, right_y);
                    for y in start..=end {
                        map.insert((left_x, y), Material::Rock);
                    }
                }
            }
        }
        let min_x = cmp::min(500, *map.keys().map(|(x, _)| x).min().unwrap());
        let max_x = cmp::max(500, *map.keys().map(|(x, _)| x).max().unwrap());
        let min_y = cmp::min(0, *map.keys().map(|(_, y)| y).min().unwrap());
        let max_y = cmp::max(0, *map.keys().map(|(_, y)| y).max().unwrap());
        World {
            map,
            x_bounds: min_x..=max_x,
            y_bounds: min_y..=max_y,
        }
    }
}

pub fn parse_input(input: &str) -> World {
    World::from_input(input)
}

pub fn part_one(world: &mut World) -> usize {
    solve(
        world,
        |world, p| !world.map.contains_key(&p),
        |world, (x, y)| !world.x_bounds.contains(&x) || !world.y_bounds.contains(&y),
    )
}

pub fn part_two(world: &mut World) -> usize {
    solve(
        world,
        |world, (x, y)| !world.map.contains_key(&(x, y)) && y < *world.y_bounds.end() + 2,
        |world, _| world.map.contains_key(&SPAWN_POINT),
    )
}

fn solve<F1, F2>(world: &mut World, is_valid_position: F1, should_return: F2) -> usize
where
    F1: Fn(&World, Point) -> bool,
    F2: Fn(&World, Point) -> bool,
{
    let mut settled_sand_unit_count = 0;
    loop {
        let (mut x, mut y) = SPAWN_POINT;
        loop {
            if should_return(&world, (x, y)) {
                return settled_sand_unit_count;
            }
            let candidate_positions = [(x, y + 1), (x - 1, y + 1), (x + 1, y + 1)];
            let next_position = candidate_positions
                .into_iter()
                .find(|&p| is_valid_position(&world, p));
            match next_position {
                Some(p) => (x, y) = p,
                None => {
                    world.map.insert((x, y), Material::Sand);
                    settled_sand_unit_count += 1;
                    break;
                }
            }
        }
    }
}
