pub const INPUT: &str = include_str!("../input.txt");

const WORD_DIGIT_PAIRS: [(&str, u8); 9] = [
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
            let left_digit = line.bytes().find(u8::is_ascii_digit).map(|b| b - b'0');
            let right_digit = line.bytes().rfind(u8::is_ascii_digit).map(|b| b - b'0');
            (10 * left_digit.unwrap() + right_digit.unwrap()) as u32
        })
        .sum()
}

pub fn part_two(lines: &[&str]) -> u32 {
    lines
        .iter()
        .map(|line| {
            let left_digit = (0..line.len()).find_map(|i| {
                let s = &line[i..];
                if let Some(b) = s.bytes().next().filter(u8::is_ascii_digit) {
                    return Some(b - b'0');
                }
                for &(word, digit) in &WORD_DIGIT_PAIRS {
                    if s.starts_with(word) {
                        return Some(digit);
                    }
                }
                None
            });
            let right_digit = (0..line.len()).rev().find_map(|i| {
                let s = &line[..=i];
                if let Some(b) = s.bytes().next_back().filter(u8::is_ascii_digit) {
                    return Some(b - b'0');
                }
                for &(word, digit) in &WORD_DIGIT_PAIRS {
                    if s.ends_with(word) {
                        return Some(digit);
                    }
                }
                None
            });
            (10 * left_digit.unwrap() + right_digit.unwrap()) as u32
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
