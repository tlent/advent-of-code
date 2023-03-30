pub const INPUT: &str = include_str!("./input.txt");

pub fn part_one(lines: &[&str]) -> u32 {
    lines
        .iter()
        .map(|line| {
            let (left, right) = line.split_at(line.len() / 2);
            let common_byte = find_common_byte(&[left, right]).unwrap();
            priority(common_byte)
        })
        .sum::<u32>()
}

pub fn part_two(lines: &[&str]) -> u32 {
    lines
        .chunks_exact(3)
        .map(|chunk_lines| {
            let common_byte = find_common_byte(chunk_lines).unwrap();
            priority(common_byte)
        })
        .sum::<u32>()
}

fn find_common_byte(strs: &[&str]) -> Option<u8> {
    strs[0]
        .bytes()
        .find(|&byte| strs[1..].iter().all(|str| str.bytes().any(|b| b == byte)))
}

fn priority(byte: u8) -> u32 {
    u32::from(match byte {
        b'a'..=b'z' => 1 + byte - b'a',
        b'A'..=b'Z' => 27 + byte - b'A',
        _ => panic!("Invalid byte"),
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let lines: Vec<_> = INPUT.lines().collect();
        assert_eq!(part_one(&lines), 8515);
    }

    #[test]
    fn test_part_two() {
        let lines: Vec<_> = INPUT.lines().collect();
        assert_eq!(part_two(&lines), 2434);
    }
}
