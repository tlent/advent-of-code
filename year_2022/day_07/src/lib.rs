pub const INPUT: &str = include_str!("../input.txt");

pub fn parse_input(input: &str) -> (Vec<usize>, usize) {
    let mut sizes = vec![];
    let mut root_size = 0;
    let mut stack: Vec<usize> = vec![];
    for line in input.lines() {
        match line {
            "$ cd .." => {
                let size = stack.pop().unwrap();
                sizes.push(size);
            }
            s if s.starts_with("$ cd") => {
                stack.push(0);
            }
            s => {
                let file_size = s
                    .split_whitespace()
                    .next()
                    .and_then(|s| s.parse::<usize>().ok());
                if let Some(size) = file_size {
                    for directory_size in &mut stack {
                        *directory_size += size;
                    }
                    root_size += size;
                }
            }
        }
    }
    sizes.append(&mut stack);
    (sizes, root_size)
}

pub fn part_one(directory_sizes: &[usize]) -> usize {
    const MAX_SIZE: usize = 100_000;
    directory_sizes
        .iter()
        .filter(|&&size| size <= MAX_SIZE)
        .sum()
}

pub fn part_two(directory_sizes: &[usize], root_size: usize) -> usize {
    const DISK_SIZE: usize = 70_000_000;
    const TOTAL_REQUIRED_SPACE: usize = 30_000_000;
    let available_space = DISK_SIZE - root_size;
    let required_space = TOTAL_REQUIRED_SPACE - available_space;
    *directory_sizes
        .iter()
        .filter(|&&size| size >= required_space)
        .min()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let (directory_sizes, _) = parse_input(INPUT);
        assert_eq!(part_one(&directory_sizes), 1_743_217);
    }

    #[test]
    fn test_part_two() {
        let (directory_sizes, root_size) = parse_input(INPUT);
        assert_eq!(part_two(&directory_sizes, root_size), 8_319_096);
    }
}
