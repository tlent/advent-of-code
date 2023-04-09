use std::ops::RangeInclusive;

pub const INPUT: &str = include_str!("../input.txt");

pub fn parse_input(input: &str) -> Vec<(RangeInclusive<u32>, RangeInclusive<u32>)> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split([',', '-']).map(|s| s.parse::<u32>().unwrap());
            (
                parts.next().unwrap()..=parts.next().unwrap(),
                parts.next().unwrap()..=parts.next().unwrap(),
            )
        })
        .collect()
}

pub fn part_one(pairs: &[(RangeInclusive<u32>, RangeInclusive<u32>)]) -> usize {
    pairs
        .iter()
        .filter(|(a, b)| {
            b.contains(a.start()) && b.contains(a.end())
                || a.contains(b.start()) && a.contains(b.end())
        })
        .count()
}

pub fn part_two(pairs: &[(RangeInclusive<u32>, RangeInclusive<u32>)]) -> usize {
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
        let pairs = parse_input(INPUT);
        assert_eq!(part_one(&pairs), 441);
    }

    #[test]
    fn test_part_two() {
        let pairs = parse_input(INPUT);
        assert_eq!(part_two(&pairs), 861);
    }
}
