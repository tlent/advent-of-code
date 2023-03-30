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
    fn to_index(byte: u8) -> usize {
        usize::from(match byte {
            b'a'..=b'z' => byte - b'a',
            b'A'..=b'Z' => 26 + byte - b'A',
            _ => panic!("Invalid byte"),
        })
    }

    fn to_byte(index: usize) -> u8 {
        match index {
            0..=25 => b'a' + index as u8,
            26..=51 => b'A' + (index - 26) as u8,
            _ => panic!("Invalid index"),
        }
    }

    let mut common_bytes = [true; 52];
    for str in strs {
        let mut bytes = [false; 52];
        for byte in str.bytes() {
            bytes[to_index(byte)] = true;
        }
        for (common, byte) in common_bytes.iter_mut().zip(bytes.iter()) {
            *common &= *byte;
        }
    }
    common_bytes
        .into_iter()
        .enumerate()
        .find(|&(_, common)| common)
        .map(|(index, _)| to_byte(index))
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
