pub const INPUT: &str = include_str!("../input.txt");

pub fn parse_input(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| {
            let winning_numbers: Vec<u32> = line[10..39]
                .split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            line[42..]
                .split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .filter(|number| winning_numbers.contains(number))
                .count() as u32
        })
        .collect()
}

pub fn part_one(win_counts: &[u32]) -> u32 {
    win_counts
        .iter()
        .map(|&count| if count > 0 { 2u32.pow(count - 1) } else { 0 })
        .sum()
}

pub fn part_two(win_counts: &[u32]) -> u32 {
    let mut card_counts = vec![1; win_counts.len()];
    for (index, &win_count) in win_counts.iter().enumerate() {
        let card_count = card_counts[index];
        let start = index + 1;
        let end = start + win_count as usize;
        for later_card_count in card_counts[start..end].iter_mut() {
            *later_card_count += card_count;
        }
    }
    card_counts.iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let match_counts = parse_input(INPUT);
        assert_eq!(part_one(&match_counts), 22_897);
    }

    #[test]
    fn test_part_two() {
        let match_counts = parse_input(INPUT);
        assert_eq!(part_two(&match_counts), 5_095_824);
    }
}
