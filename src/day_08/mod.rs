use std::cmp;

pub const INPUT: &str = include_str!("input.txt");

pub struct Grid {
    size: usize,
    data: Vec<u8>,
}

impl Grid {
    pub fn rows(&self) -> impl Iterator<Item = &[u8]> + ExactSizeIterator + DoubleEndedIterator {
        self.data.chunks_exact(self.size)
    }
}

pub fn parse_input(input: &str) -> Grid {
    let size = input.find('\n').unwrap();
    let data: Vec<_> = input
        .bytes()
        .filter_map(|byte| {
            if byte.is_ascii_digit() {
                Some(byte - b'0')
            } else {
                None
            }
        })
        .collect();
    Grid { size, data }
}

pub fn part_one(grid: &Grid) -> usize {
    let mut visible = vec![false; grid.size * grid.size];
    let mut visible_count = 0;

    let mut max_up: Vec<Option<u8>> = vec![None; grid.size];
    for (y, row) in grid.rows().enumerate() {
        let mut max_left = None;
        for (x, &digit) in row.iter().enumerate() {
            let index = y * grid.size + x;
            if Some(digit) > max_left || Some(digit) > max_up[x] {
                visible_count += 1;
                visible[index] = true;
                max_left = cmp::max(max_left, Some(digit));
                max_up[x] = cmp::max(max_up[x], Some(digit));
            }
        }
    }

    let mut max_down: Vec<Option<u8>> = vec![None; grid.size];
    for (y, row) in grid.rows().enumerate().rev() {
        let mut max_right = None;
        for (x, &digit) in row.iter().enumerate().rev() {
            let index = y * grid.size + x;
            if Some(digit) > max_right || Some(digit) > max_down[x] {
                if !visible[index] {
                    visible_count += 1;
                    visible[index] = true;
                }
                max_right = cmp::max(max_right, Some(digit));
                max_down[x] = cmp::max(max_down[x], Some(digit));
            }
        }
    }

    visible_count
}

pub fn part_two(grid: &Grid) -> u32 {
    let mut scores = vec![1; grid.size * grid.size];

    let mut counts_up: Vec<[u32; 10]> = vec![[0; 10]; grid.size];
    for (y, row) in grid.rows().enumerate() {
        let mut counts_left = [0; 10];
        for (x, &digit) in row.iter().enumerate() {
            let index = y * grid.size + x;
            let digit_usize = digit as usize;
            scores[index] *= counts_left[digit_usize];
            scores[index] *= counts_up[x][digit_usize];
            for counts in [&mut counts_left, &mut counts_up[x]].iter_mut() {
                counts[..=digit_usize].fill(1);
                for count in &mut counts[(digit_usize + 1)..] {
                    *count += 1;
                }
            }
        }
    }

    let mut counts_down: Vec<[u32; 10]> = vec![[0; 10]; grid.size];
    for (y, row) in grid.rows().enumerate().rev() {
        let mut counts_right = [0; 10];
        for (x, &digit) in row.iter().enumerate().rev() {
            let index = y * grid.size + x;
            let digit_usize = digit as usize;
            scores[index] *= counts_right[digit_usize];
            scores[index] *= counts_down[x][digit_usize];
            for counts in [&mut counts_right, &mut counts_down[x]].iter_mut() {
                counts[..=digit_usize].fill(1);
                for count in &mut counts[(digit_usize + 1)..] {
                    *count += 1;
                }
            }
        }
    }

    *scores.iter().max().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let grid = parse_input(INPUT);
        assert_eq!(part_one(&grid), 1681);
    }

    #[test]
    fn test_part_two() {
        let grid = parse_input(INPUT);
        assert_eq!(part_two(&grid), 201684);
    }
}
