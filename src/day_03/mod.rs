pub const INPUT: &str = include_str!("input.txt");

pub type Set = [bool; 52];

pub fn parse_input(input: &str) -> (Vec<(Set, Set)>, Vec<Set>) {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_at(line.len() / 2);
            let mut left_set = [false; 52];
            let mut right_set = [false; 52];
            let mut line_set = [false; 52];
            for byte in left.bytes() {
                let set_index = to_set_index(byte);
                left_set[set_index] = true;
                line_set[set_index] = true;
            }
            for byte in right.bytes() {
                let set_index = to_set_index(byte);
                right_set[set_index] = true;
                line_set[set_index] = true;
            }
            ((left_set, right_set), line_set)
        })
        .unzip()
}

pub fn part_one(sets: &[(Set, Set)]) -> u32 {
    sets.iter()
        .map(|&(left, right)| {
            let common_byte = find_common_byte(&[left, right]).unwrap();
            priority(common_byte)
        })
        .sum::<u32>()
}

pub fn part_two(sets: &[Set]) -> u32 {
    sets.chunks_exact(3)
        .map(|sets| {
            let common_byte = find_common_byte(sets).unwrap();
            priority(common_byte)
        })
        .sum::<u32>()
}

fn find_common_byte(sets: &[Set]) -> Option<u8> {
    let mut common = sets[0];
    for set in &sets[1..] {
        for (is_common, set_has_byte) in common.iter_mut().zip(set.iter()) {
            *is_common &= set_has_byte;
        }
    }
    common
        .iter()
        .zip(0..)
        .find(|(&is_common, _)| is_common)
        .map(|(_, set_index)| to_byte(set_index))
}

fn priority(byte: u8) -> u32 {
    u32::from(match byte {
        b'a'..=b'z' => 1 + byte - b'a',
        b'A'..=b'Z' => 27 + byte - b'A',
        _ => panic!("Invalid byte"),
    })
}

fn to_set_index(byte: u8) -> usize {
    usize::from(match byte {
        b'a'..=b'z' => byte - b'a',
        b'A'..=b'Z' => byte - b'A' + 26,
        _ => panic!("Invalid byte"),
    })
}

fn to_byte(set_index: u8) -> u8 {
    match set_index {
        0..=25 => b'a' + set_index,
        26..=51 => b'A' + (set_index - 26),
        _ => panic!("Invalid index"),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let (part_one_sets, _) = parse_input(INPUT);
        assert_eq!(part_one(&part_one_sets), 8515);
    }

    #[test]
    fn test_part_two() {
        let (_, part_two_sets) = parse_input(INPUT);
        assert_eq!(part_two(&part_two_sets), 2434);
    }
}
