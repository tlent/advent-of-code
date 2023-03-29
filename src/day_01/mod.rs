pub const INPUT: &str = include_str!("./input.txt");

pub fn parse_input(input: &str) -> Vec<u32> {
    input
        .split("\n\n")
        .map(|s| s.lines().map(|v| v.parse::<u32>().unwrap()).sum())
        .collect()
}

pub fn part_one(sorted_group_sums: &[u32]) -> u32 {
    *sorted_group_sums.last().unwrap()
}

pub fn part_two(sorted_group_sums: &[u32]) -> u32 {
    sorted_group_sums.iter().rev().take(3).sum::<u32>()
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_test_input() -> Vec<u32> {
        let mut group_sums = parse_input(INPUT);
        group_sums.sort_unstable();
        group_sums
    }

    #[test]
    fn test_part_one() {
        let input = get_test_input();
        assert_eq!(part_one(&input), 70509);
    }

    #[test]
    fn test_part_two() {
        let input = get_test_input();
        assert_eq!(part_two(&input), 208567);
    }
}
