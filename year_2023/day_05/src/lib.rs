use std::ops::Range;
use std::{cmp, mem};

pub const INPUT: &str = include_str!("../input.txt");

pub struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<Vec<(Range<u64>, Range<u64>)>>,
}

pub fn parse_input(input: &str) -> Almanac {
    let mut lines = input.lines();
    let seeds = lines.next().unwrap()[7..]
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    lines.next();
    let maps = (0..7)
        .map(|_| {
            lines
                .by_ref()
                .skip(1)
                .take_while(|line| !line.is_empty())
                .map(|line| {
                    let mut numbers = line.split_ascii_whitespace().map(|s| s.parse().unwrap());
                    let destination_range_start = numbers.next().unwrap();
                    let source_range_start = numbers.next().unwrap();
                    let range_length = numbers.next().unwrap();
                    let source_range = source_range_start..source_range_start + range_length;
                    let destination_range =
                        destination_range_start..destination_range_start + range_length;
                    (source_range, destination_range)
                })
                .collect()
        })
        .collect();
    Almanac { seeds, maps }
}

pub fn part_one(almanac: &Almanac) -> u64 {
    almanac
        .seeds
        .iter()
        .map(|&seed| {
            let mut value = seed;
            for map in almanac.maps.iter() {
                for (source_range, destination_range) in map {
                    if source_range.contains(&value) {
                        if source_range.start <= destination_range.start {
                            value += destination_range.start - source_range.start;
                        } else {
                            value -= source_range.start - destination_range.start;
                        }
                        break;
                    }
                }
            }
            value
        })
        .min()
        .unwrap()
}

pub fn part_two(almanac: &Almanac) -> u64 {
    let mut ranges: Vec<_> = almanac
        .seeds
        .chunks_exact(2)
        .map(|chunk| {
            let seed_range_start = chunk[0];
            let seed_range_length = chunk[1];
            seed_range_start..seed_range_start + seed_range_length
        })
        .collect();
    let mut mapped_ranges = vec![];
    let mut unmapped_ranges = vec![];
    for map in almanac.maps.iter() {
        mapped_ranges.clear();
        for (source, destination) in map {
            unmapped_ranges.clear();
            for range in ranges.drain(..) {
                let mut overlap =
                    cmp::max(source.start, range.start)..cmp::min(source.end, range.end);
                if overlap.is_empty() {
                    unmapped_ranges.push(range);
                } else {
                    let left = range.start..source.start;
                    if !left.is_empty() {
                        unmapped_ranges.push(left);
                    }
                    let right = source.end..range.end;
                    if !right.is_empty() {
                        unmapped_ranges.push(right);
                    }
                    let offset = source.start.abs_diff(destination.start);
                    if source.start <= destination.start {
                        overlap.start += offset;
                        overlap.end += offset;
                    } else {
                        overlap.start -= offset;
                        overlap.end -= offset;
                    }
                    mapped_ranges.push(overlap);
                }
            }
            mem::swap(&mut ranges, &mut unmapped_ranges);
        }
        mapped_ranges.append(&mut ranges);
        mem::swap(&mut ranges, &mut mapped_ranges);
    }
    ranges.into_iter().map(|range| range.start).min().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let almanac = parse_input(INPUT);
        assert_eq!(part_one(&almanac), 324_724_204);
    }

    #[test]
    fn test_part_two() {
        let almanac = parse_input(INPUT);
        assert_eq!(part_two(&almanac), 104_070_862);
    }
}
