pub const INPUT: &str = include_str!("input.txt");

pub fn part_one(input: &str) -> usize {
    find_unique_window_index(input.trim().as_bytes(), 4).unwrap()
}

pub fn part_two(input: &str) -> usize {
    find_unique_window_index(input.trim().as_bytes(), 14).unwrap()
}

fn find_unique_window_index(bytes: &[u8], window_size: usize) -> Option<usize> {
    let mut byte_counts = [0; 26];
    let mut duplicate_count = 0;

    for &byte in &bytes[..window_size] {
        let byte_count = &mut byte_counts[to_index(byte)];
        *byte_count += 1;
        if *byte_count == 2 {
            duplicate_count += 1;
        }
    }

    for index in window_size..bytes.len() {
        let removed_byte = bytes[index - window_size];
        let removed_byte_count = &mut byte_counts[to_index(removed_byte)];
        *removed_byte_count -= 1;
        if *removed_byte_count == 1 {
            duplicate_count -= 1;
        }
        let added_byte = bytes[index];
        let added_byte_count = &mut byte_counts[to_index(added_byte)];
        *added_byte_count += 1;
        if *added_byte_count == 2 {
            duplicate_count += 1;
        }
        if duplicate_count == 0 {
            return Some(index + 1);
        }
    }
    None
}

fn to_index(byte: u8) -> usize {
    (byte - b'a') as usize
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), 1647);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT), 2447);
    }
}
