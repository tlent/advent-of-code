pub const INPUT: &str = include_str!("input.txt");

pub struct Grid {
    size: usize,
    row_data: Vec<u8>,
    column_data: Vec<u8>,
}

impl Grid {
    pub fn from_input(input: &str) -> Self {
        let size = input.find('\n').unwrap();
        let row_data: Vec<_> = input
            .bytes()
            .filter_map(|byte| {
                if byte.is_ascii_digit() {
                    Some(byte - b'0')
                } else {
                    None
                }
            })
            .collect();
        let column_data: Vec<_> = (0..size)
            .flat_map(|column| row_data.iter().skip(column).step_by(size).copied())
            .collect();
        Self {
            size,
            row_data,
            column_data,
        }
    }

    pub fn rows(&self) -> impl Iterator<Item = &[u8]> {
        self.row_data.chunks_exact(self.size)
    }

    pub fn columns(&self) -> impl Iterator<Item = &[u8]> {
        self.column_data.chunks_exact(self.size)
    }
}

pub fn parse_input(input: &str) -> Grid {
    Grid::from_input(input)
}

pub fn part_one(grid: &Grid) -> usize {
    let mut visible = vec![false; grid.size * grid.size];
    for (y, row) in grid.rows().enumerate() {
        let mut max: Option<u8> = None;
        for (x, &digit) in row.iter().enumerate() {
            if Some(digit) > max {
                visible[y * grid.size + x] = true;
                max = Some(digit);
            }
        }
        max = None;
        for (x, &digit) in row.iter().enumerate().rev() {
            if Some(digit) > max {
                visible[y * grid.size + x] = true;
                max = Some(digit);
            }
        }
    }
    for (x, column) in grid.columns().enumerate() {
        let mut max: Option<u8> = None;
        for (y, &digit) in column.iter().enumerate() {
            if Some(digit) > max {
                visible[y * grid.size + x] = true;
                max = Some(digit);
            }
        }
        max = None;
        for (y, &digit) in column.iter().enumerate().rev() {
            if Some(digit) > max {
                visible[y * grid.size + x] = true;
                max = Some(digit);
            }
        }
    }
    visible.into_iter().filter(|&is_visible| is_visible).count()
}

pub fn part_two(grid: &Grid) -> u32 {
    let mut scores = vec![1; grid.size * grid.size];
    for (y, row) in grid.rows().enumerate() {
        let mut counts = vec![0; 10];
        for (x, &digit) in row.iter().enumerate() {
            scores[y * grid.size + x] *= counts[digit as usize];
            counts[..=digit as usize].fill(1);
            for count in &mut counts[(digit as usize + 1)..] {
                *count += 1;
            }
        }
        counts.fill(0);
        for (x, &digit) in row.iter().enumerate().rev() {
            scores[y * grid.size + x] *= counts[digit as usize];
            counts[..=digit as usize].fill(1);
            for count in &mut counts[(digit as usize + 1)..] {
                *count += 1;
            }
        }
    }
    for (x, column) in grid.columns().enumerate() {
        let mut counts = vec![0; 10];
        for (y, &digit) in column.iter().enumerate() {
            scores[y * grid.size + x] *= counts[digit as usize];
            counts[..=digit as usize].fill(1);
            for count in &mut counts[(digit as usize + 1)..] {
                *count += 1;
            }
        }
        counts.fill(0);
        for (y, &digit) in column.iter().enumerate().rev() {
            scores[y * grid.size + x] *= counts[digit as usize];
            counts[..=digit as usize].fill(1);
            for count in &mut counts[(digit as usize + 1)..] {
                *count += 1;
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
