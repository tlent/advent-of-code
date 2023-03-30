pub const INPUT: &str = include_str!("input.txt");

pub fn part_one(input: &str) -> usize {
    find_unique_window_index(input.as_bytes(), 4).unwrap()
}

pub fn part_two(input: &str) -> usize {
    find_unique_window_index(input.as_bytes(), 14).unwrap()
}

fn find_unique_window_index(bytes: &[u8], window_size: usize) -> Option<usize> {
    let mut window_set = [0; 26];
    for &byte in &bytes[..window_size] {
        window_set[byte as usize - b'a' as usize] += 1;
    }
    for index in window_size..bytes.len() {
        if window_set.iter().all(|&count| count <= 1) {
            return Some(index);
        }
        window_set[bytes[index - window_size] as usize - b'a' as usize] -= 1;
        window_set[bytes[index] as usize - b'a' as usize] += 1;
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&INPUT), 1647);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&INPUT), 2447);
    }
}
