use rustc_hash::FxHashSet;
use std::{cmp, ops::RangeInclusive};

pub const INPUT: &str = include_str!("../input.txt");

type Coordinate = (i32, i32);

#[derive(Debug)]
pub struct Sensor {
    position: Coordinate,
    nearest_beacon_distance: i32,
}

pub fn part_one(sensors: &[Sensor]) -> i32 {
    const ROW: i32 = 2_000_000;
    let mut ranges = sensors
        .iter()
        .filter_map(|sensor| {
            let (x, y) = sensor.position;
            let max_distance = sensor.nearest_beacon_distance;
            let dy = (y - ROW).abs();
            if dy > max_distance {
                return None;
            }
            let max_dx = (max_distance - dy).abs();
            Some(x - max_dx..=x + max_dx)
        })
        .collect::<Vec<_>>();
    ranges.sort_by_key(|r| *r.start());
    let mut iter = ranges.into_iter();
    let mut merged_ranges = vec![iter.next().unwrap()];
    for range in iter {
        let last_range = merged_ranges.last_mut().unwrap();
        if range.start() <= last_range.end() {
            *last_range = *last_range.start()..=cmp::max(*last_range.end(), *range.end());
        } else {
            merged_ranges.push(range);
        }
    }
    merged_ranges.into_iter().map(|r| r.end() - r.start()).sum()
}

pub fn part_two(sensors: &[Sensor]) -> usize {
    const BOUNDS: RangeInclusive<i32> = 0..=4_000_000;
    let mut ascending_line_coefficients = FxHashSet::default();
    let mut descending_line_coefficients = FxHashSet::default();
    for sensor in sensors {
        let (x, y) = sensor.position;
        let radius = sensor.nearest_beacon_distance;
        ascending_line_coefficients.extend([y - x + radius + 1, y - x - radius - 1]);
        descending_line_coefficients.extend([x + y + radius + 1, x + y - radius - 1]);
    }
    ascending_line_coefficients
        .into_iter()
        .find_map(|ascending_coefficient| {
            for &descending_coefficient in &descending_line_coefficients {
                let x = (descending_coefficient - ascending_coefficient) / 2;
                let y = (ascending_coefficient + descending_coefficient) / 2;
                if BOUNDS.contains(&x)
                    && BOUNDS.contains(&y)
                    && sensors.iter().all(|sensor| {
                        manhattan_distance(sensor.position, (x, y)) > sensor.nearest_beacon_distance
                    })
                {
                    return Some(x as usize * 4_000_000 + y as usize);
                }
            }
            None
        })
        .unwrap()
}

fn manhattan_distance((left_x, left_y): Coordinate, (right_x, right_y): Coordinate) -> i32 {
    (left_x - right_x).abs() + (left_y - right_y).abs()
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

    fn coordinate(input: &str) -> IResult<&str, Coordinate> {
        separated_pair(preceded(tag("x="), i32), tag(", y="), i32)(input)
    }

    fn sensor(input: &str) -> IResult<&str, Sensor> {
        map(
            terminated(
                separated_pair(
                    preceded(tag("Sensor at "), coordinate),
                    tag(": closest beacon is at "),
                    coordinate,
                ),
                opt(line_ending),
            ),
            |(position, nearest_beacon_position)| {
                let nearest_beacon_distance = manhattan_distance(position, nearest_beacon_position);
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
