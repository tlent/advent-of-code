use std::{num::ParseIntError, ops::RangeInclusive};

pub const INPUT: &str = include_str!("../input.txt");

type ParseResult = Result<Vec<(RangeInclusive<u8>, RangeInclusive<u8>)>, ParseIntError>;
pub fn parse_input(input: &str) -> ParseResult {
    let parts = input.trim().split(['\n', ',', '-']);
    let numbers = parts.map(str::parse::<u8>).collect::<Result<Vec<_>, _>>()?;
    let ranges = numbers
        .chunks_exact(4)
        .map(|chunk| (chunk[0]..=chunk[1], chunk[2]..=chunk[3]))
        .collect();
    Ok(ranges)
}

pub fn part_one(pairs: &[(RangeInclusive<u8>, RangeInclusive<u8>)]) -> usize {
    pairs
        .iter()
        .filter(|(a, b)| {
            b.contains(a.start()) && b.contains(a.end())
                || a.contains(b.start()) && a.contains(b.end())
        })
        .count()
}

pub fn part_two(pairs: &[(RangeInclusive<u8>, RangeInclusive<u8>)]) -> usize {
    pairs
        .iter()
        .filter(|(a, b)| {
            b.contains(a.start())
                || b.contains(a.end())
                || a.contains(b.start())
                || a.contains(b.end())
        })
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let pairs = parse_input(INPUT).unwrap();
        assert_eq!(part_one(&pairs), 441);
    }

    #[test]
    fn test_part_two() {
        let pairs = parse_input(INPUT).unwrap();
        assert_eq!(part_two(&pairs), 861);
    }
}
