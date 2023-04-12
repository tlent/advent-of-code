pub const INPUT: &str = include_str!("../input.txt");

pub fn solve(mut group_sums: Vec<u32>) -> (u32, u32) {
    group_sums.sort_unstable();
    (
        *group_sums.last().unwrap(),
        group_sums.iter().rev().take(3).sum::<u32>(),
    )
}

pub mod parser {
    use anyhow::{anyhow, Result};
    use nom::{
        character::complete::{line_ending, u32},
        multi::{fold_many1, separated_list1},
        sequence::terminated,
        Finish, IResult,
    };

    pub fn parse(input: &str) -> Result<Vec<u32>> {
        let (rest, number_group_sums) = group_sums(input)
            .finish()
            .map_err(|err| anyhow!(err.to_string()))?;
        if !rest.is_empty() {
            return Err(anyhow!("Unparsed input: {}", rest));
        }
        Ok(number_group_sums)
    }

    fn group_sum(input: &str) -> IResult<&str, u32> {
        fold_many1(terminated(u32, line_ending), || 0, |acc, item| acc + item)(input)
    }

    fn group_sums(input: &str) -> IResult<&str, Vec<u32>> {
        separated_list1(line_ending, group_sum)(input)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve() {
        let group_sums = parser::parse(INPUT).unwrap();
        let (part_one, part_two) = solve(group_sums);
        assert_eq!(part_one, 70509);
        assert_eq!(part_two, 208_567);
    }
}
