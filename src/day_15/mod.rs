use rayon::prelude::*;
use std::{cmp, ops::RangeInclusive};

pub const INPUT: &str = include_str!("input.txt");

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn manhattan_distance(self, other: Position) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Debug)]
pub struct Sensor {
    position: Position,
    nearest_beacon_distance: i32,
}

pub fn part_one(sensors: &[Sensor]) -> i32 {
    find_covered_ranges_by_row(sensors, 2_000_000)
        .into_iter()
        .map(|r| r.end() - r.start())
        .sum()
}

pub fn part_two(sensors: &[Sensor]) -> usize {
    const MAX_ROW: i32 = 4_000_000;
    (0..=MAX_ROW)
        .into_par_iter()
        .find_map_any(|y| {
            let ranges = find_covered_ranges_by_row(sensors, y);
            if !ranges
                .iter()
                .any(|r| *r.start() <= 0 && *r.end() >= MAX_ROW)
            {
                let x = (ranges[0].end() + 1) as usize;
                Some(x * MAX_ROW as usize + y as usize)
            } else {
                None
            }
        })
        .unwrap()
}

fn find_covered_ranges_by_row(sensors: &[Sensor], row: i32) -> Vec<RangeInclusive<i32>> {
    let mut ranges = sensors
        .iter()
        .filter_map(|sensor| {
            let Position { x, y } = sensor.position;
            let max_distance = sensor.nearest_beacon_distance;
            let dy = (y - row).abs();
            if dy > max_distance {
                return None;
            }
            let max_dx = (max_distance - dy).abs();
            Some(x - max_dx..=x + max_dx)
        })
        .collect::<Vec<_>>();
    ranges.sort_by_key(|r| *r.start());
    let mut merged_ranges = vec![ranges[0].clone()];
    for range in ranges.into_iter().skip(1) {
        let last_range = merged_ranges.last_mut().unwrap();
        if range.start() <= last_range.end() {
            *last_range = *last_range.start()..=cmp::max(*last_range.end(), *range.end());
        } else {
            merged_ranges.push(range);
        }
    }
    merged_ranges
}

pub mod parser {
    use super::*;
    use anyhow::{anyhow, Result};
    use nom::{
        bytes::complete::tag,
        character::complete::{i32, line_ending},
        combinator::{map, opt},
        multi::many0,
        sequence::{preceded, separated_pair, terminated},
        Finish, IResult,
    };

    pub fn parse(input: &str) -> Result<Vec<Sensor>> {
        let (_, sensors) = sensors(input)
            .finish()
            .map_err(|err| anyhow!(err.to_string()))?;
        Ok(sensors)
    }

    fn position(input: &str) -> IResult<&str, Position> {
        map(
            separated_pair(preceded(tag("x="), i32), tag(", y="), i32),
            |(x, y)| Position { x, y },
        )(input)
    }

    fn sensor(input: &str) -> IResult<&str, Sensor> {
        map(
            terminated(
                separated_pair(
                    preceded(tag("Sensor at "), position),
                    tag(": closest beacon is at "),
                    position,
                ),
                opt(line_ending),
            ),
            |(position, nearest_beacon_position)| {
                let nearest_beacon_distance = position.manhattan_distance(nearest_beacon_position);
                Sensor {
                    position,
                    nearest_beacon_distance,
                }
            },
        )(input)
    }

    fn sensors(input: &str) -> IResult<&str, Vec<Sensor>> {
        many0(sensor)(input)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let sensors = parser::parse(INPUT).unwrap();
        assert_eq!(part_one(&sensors), 4_985_193);
    }

    #[test]
    fn test_part_two() {
        let sensors = parser::parse(INPUT).unwrap();
        assert_eq!(part_two(&sensors), 11_583_882_601_918);
    }
}
