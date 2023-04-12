pub const INPUT: &str = include_str!("../input.txt");

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn parse_input(input: &str) -> Result<Vec<u32>> {
    let mut sums = vec![];
    let mut sum = 0;
    for line in input.lines() {
        if line.is_empty() {
            sums.push(sum);
            sum = 0;
        } else {
            sum += line.parse::<u32>()?;
        }
    }
    Ok(sums)
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
    fn test_solve() {
        let group_sums = parse_input(INPUT).unwrap();
        let (part_one, part_two) = solve(group_sums);
        assert_eq!(part_one, 70_509);
        assert_eq!(part_two, 208_567);
    }
}
