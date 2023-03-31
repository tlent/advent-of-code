pub const INPUT: &str = include_str!("input.txt");

pub fn parse_input(input: &str) -> Vec<u32> {
    input
        .split("\n\n")
        .map(|s| s.lines().map(|v| v.parse::<u32>().unwrap()).sum())
        .collect()
}

pub fn solve(mut group_sums: Vec<u32>) -> (u32, u32) {
    group_sums.sort_unstable();
    (
        *group_sums.last().unwrap(),
        group_sums.iter().rev().take(3).sum::<u32>(),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let group_sums = parse_input(INPUT);
        let (part_one, part_two) = solve(group_sums);
        assert_eq!(part_one, 70509);
        assert_eq!(part_two, 208567);
    }
}
