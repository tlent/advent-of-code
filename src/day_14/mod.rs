use crate::{HashMap, HashSet};
use std::cmp;
use std::collections::VecDeque;
use std::fmt::Display;
use std::ops::RangeInclusive;

pub const INPUT: &str = include_str!("input.txt");

const SPAWN_POINT: Point = (500, 0);

type Point = (usize, usize);

#[derive(Debug, Clone)]
pub enum Material {
    Rock,
    Sand,
}

#[derive(Debug, Clone)]
pub struct World {
    map: HashMap<Point, Material>,
    rock_bounds: (RangeInclusive<usize>, RangeInclusive<usize>),
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
                if left_x == right_x {
                    let start = cmp::min(left_y, right_y);
                    let end = cmp::max(left_y, right_y);
                    for y in start..=end {
                        map.insert((left_x, y), Material::Rock);
                    }
                } else {
                    let start = cmp::min(left_x, right_x);
                    let end = cmp::max(left_x, right_x);
                    for x in start..=end {
                        map.insert((x, left_y), Material::Rock);
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
            rock_bounds: (min_x..=max_x, min_y..=max_y),
        }
    }

    fn find_sand_bounds(&self) -> (RangeInclusive<usize>, RangeInclusive<usize>) {
        let map = &self.map;
        let min_x = cmp::min(500, *map.keys().map(|(x, _)| x).min().unwrap());
        let max_x = cmp::max(500, *map.keys().map(|(x, _)| x).max().unwrap());
        let min_y = cmp::min(0, *map.keys().map(|(_, y)| y).min().unwrap());
        let max_y = cmp::max(0, *map.keys().map(|(_, y)| y).max().unwrap());
        (min_x..=max_x, min_y..=max_y)
    }
}

impl Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (x_bounds, y_bounds) = self.find_sand_bounds();
        for y in y_bounds.clone() {
            for x in x_bounds.clone() {
                let c = match self.map.get(&(x, y)) {
                    Some(Material::Rock) => '#',
                    Some(Material::Sand) => 'o',
                    None if (x, y) == SPAWN_POINT => '+',
                    None => '.',
                };
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn parse_input(input: &str) -> World {
    World::from_input(input)
}

pub fn part_one(world: &mut World) -> usize {
    let (x_bounds, y_bounds) = &world.rock_bounds;
    let mut settled_sand_unit_count = 0;
    let mut stack = vec![SPAWN_POINT];
    let mut seen = HashSet::default();
    seen.insert(SPAWN_POINT);
    while let Some(point @ (x, y)) = stack.pop() {
        let next_points = [(x, y + 1), (x - 1, y + 1), (x + 1, y + 1)];
        if next_points
            .iter()
            .any(|(x, y)| !x_bounds.contains(&x) || !y_bounds.contains(&y))
        {
            break;
        }
        if next_points.iter().all(|p| world.map.contains_key(&p)) {
            settled_sand_unit_count += 1;
            world.map.insert(point, Material::Sand);
            continue;
        }
        stack.push(point);
        for point in next_points.into_iter().rev() {
            if !world.map.contains_key(&point) && !seen.contains(&point) {
                stack.push(point);
                seen.insert(point);
            }
        }
    }
    settled_sand_unit_count
}

pub fn part_two(world: &mut World) -> usize {
    let (_, y_bounds) = &world.rock_bounds;
    let mut settled_sand_unit_count = 0;
    let mut queue = VecDeque::from([SPAWN_POINT]);
    let mut seen = HashSet::default();
    seen.insert(SPAWN_POINT);
    while let Some(point @ (x, y)) = queue.pop_front() {
        settled_sand_unit_count += 1;
        world.map.insert(point, Material::Sand);
        let next_points = [(x, y + 1), (x - 1, y + 1), (x + 1, y + 1)];
        for next_point @ (_x, y) in next_points {
            if y != *y_bounds.end() + 2
                && !world.map.contains_key(&next_point)
                && !seen.contains(&next_point)
            {
                queue.push_back(next_point);
                seen.insert(next_point);
            }
        }
    }
    settled_sand_unit_count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let mut world = parse_input(INPUT);
        assert_eq!(part_one(&mut world), 683);
    }

    #[test]
    fn test_part_two() {
        let mut world = parse_input(INPUT);
        assert_eq!(part_two(&mut world), 28_821);
    }
}
