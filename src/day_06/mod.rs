pub const INPUT: &str = include_str!("input.txt");

pub fn part_one(input: &str) -> usize {
    find_unique_window_index(input.trim().as_bytes(), 4).unwrap()
}

pub fn part_two(input: &str) -> usize {
    find_unique_window_index(input.trim().as_bytes(), 14).unwrap()
}

fn find_unique_window_index(bytes: &[u8], window_size: usize) -> Option<usize> {
    let mut previous_index_by_letter = [None; 26];
    let mut previous_index = vec![None; bytes.len()];
    for index in 0..bytes.len() {
        if index >= window_size {
            let window_start = index - window_size;
            let window = &previous_index[window_start..index];
            if window.iter().all(|&i| i < Some(window_start)) {
                return Some(index);
            }
        }
        let index_by_letter = &mut previous_index_by_letter[bytes[index] as usize - b'a' as usize];
        previous_index[index] = *index_by_letter;
        *index_by_letter = Some(index);
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
