use rayon::prelude::*;
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

    pub fn par_rows(&self) -> impl ParallelIterator<Item = &[u8]> + IndexedParallelIterator {
        self.data.par_chunks_exact(self.size)
    }

    pub fn get(&self, x: usize, y: usize) -> u8 {
        self.data[y * self.size + x]
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

pub fn part_two(grid: &Grid) -> usize {
    grid.par_rows()
        .enumerate()
        .flat_map_iter(|(y, row)| {
            row.iter().enumerate().map(move |(x, &digit)| {
                let left_count = (0..x)
                    .rev()
                    .enumerate()
                    .find_map(|(i, closure_x)| {
                        if grid.get(closure_x, y) >= digit {
                            Some(i + 1)
                        } else {
                            None
                        }
                    })
                    .unwrap_or(x);
                let right_count = (x + 1..grid.size)
                    .enumerate()
                    .find_map(|(i, closure_x)| {
                        if grid.get(closure_x, y) >= digit {
                            Some(i + 1)
                        } else {
                            None
                        }
                    })
                    .unwrap_or_else(|| grid.size - x - 1);
                let up_count = (0..y)
                    .rev()
                    .enumerate()
                    .find_map(|(i, closure_y)| {
                        if grid.get(x, closure_y) >= digit {
                            Some(i + 1)
                        } else {
                            None
                        }
                    })
                    .unwrap_or(y);
                let down_count = (y + 1..grid.size)
                    .enumerate()
                    .find_map(|(i, closure_y)| {
                        if grid.get(x, closure_y) >= digit {
                            Some(i + 1)
                        } else {
                            None
                        }
                    })
                    .unwrap_or_else(|| grid.size - y - 1);

                left_count * right_count * up_count * down_count
            })
        })
        .max()
        .unwrap()
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
