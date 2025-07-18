const INPUT: &str = include_str!("../input");

fn main() {
    let grid = Grid::from_input(INPUT);
    println!("{}", part_one(grid.clone()));
    println!("{}", part_two(grid));
}

#[derive(Debug, Clone)]
struct Grid {
    levels: Vec<u8>,
    size: usize,
}

impl Grid {
    fn from_input(input: &str) -> Self {
        let levels: Vec<u8> = input
            .lines()
            .flat_map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8))
            .collect();
        let size: usize = input.lines().count();
        Self { levels, size }
    }

    fn step(&mut self) -> u32 {
        let mut flash_count = 0;
        let mut flashed = vec![false; self.levels.len()];
        for level in self.levels.iter_mut() {
            *level += 1;
        }
        let mut stack: Vec<_> = self
            .levels
            .iter()
            .enumerate()
            .filter_map(|(i, &l)| if l > 9 { Some(i) } else { None })
            .collect();
        while let Some(i) = stack.pop() {
            if flashed[i] {
                continue;
            }
            flashed[i] = true;
            self.levels[i] = 0;
            flash_count += 1;
            for j in adjacent_indices(i, self.size) {
                if flashed[j] {
                    continue;
                }
                self.levels[j] += 1;
                if self.levels[j] > 9 {
                    stack.push(j);
                }
            }
        }
        flash_count
    }
}

fn part_one(mut grid: Grid) -> u32 {
    let mut flash_count = 0;
    for _ in 0..100 {
        flash_count += grid.step();
    }
    flash_count
}

fn part_two(mut grid: Grid) -> u32 {
    for step in 1.. {
        if grid.step() == (grid.size * grid.size) as u32 {
            return step;
        }
    }
    unreachable!()
}

fn adjacent_indices(i: usize, grid_size: usize) -> Vec<usize> {
    let origin_x = i % grid_size;
    let origin_y = i / grid_size;
    let mut adjacents = vec![];
    for adjacent_x in origin_x.saturating_sub(1)..=(origin_x + 1).min(grid_size - 1) {
        for adjacent_y in origin_y.saturating_sub(1)..=(origin_y + 1).min(grid_size - 1) {
            if adjacent_x == origin_x && adjacent_y == origin_y {
                continue;
            }
            adjacents.push(adjacent_y * grid_size + adjacent_x);
        }
    }
    adjacents
}
