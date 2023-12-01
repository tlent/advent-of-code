pub const INPUT: &str = include_str!("../input.txt");

const DIGITS: [(&str, u32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

pub fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn part_one(lines: &[&str]) -> u32 {
    lines
        .iter()
        .map(|line| {
            let left_digit = line
                .chars()
                .find(char::is_ascii_digit)
                .and_then(|c| c.to_digit(10))
                .unwrap();
            let right_digit = line
                .chars()
                .rfind(char::is_ascii_digit)
                .and_then(|c| c.to_digit(10))
                .unwrap();
            10 * left_digit + right_digit
        })
        .sum()
}

pub fn part_two(lines: &[&str]) -> u32 {
    lines
        .iter()
        .map(|line| {
            let mut left_digit = None;
            'outer: for i in 0..line.len() {
                let l = &line[i..];
                if l.starts_with(|c: char| c.is_ascii_digit()) {
                    left_digit = l.chars().next().and_then(|c| c.to_digit(10));
                    break;
                }
                for &(word, digit) in &DIGITS {
                    if l.starts_with(word) {
                        left_digit = Some(digit);
                        break 'outer;
                    }
                }
            }
            let mut right_digit = None;
            'outer: for i in 0..line.len() {
                let l = &line[..line.len() - i];
                if l.ends_with(|c: char| c.is_ascii_digit()) {
                    right_digit = l.chars().next_back().and_then(|c| c.to_digit(10));
                    break;
                }
                for &(word, digit) in &DIGITS {
                    if l.ends_with(word) {
                        right_digit = Some(digit);
                        break 'outer;
                    }
                }
            }
            10 * left_digit.unwrap() + right_digit.unwrap()
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let lines = parse_input(INPUT);
        assert_eq!(part_one(&lines), 54_632);
    }

    #[test]
    fn test_part_two() {
        let lines = parse_input(INPUT);
        assert_eq!(part_two(&lines), 54_019);
    }
}
