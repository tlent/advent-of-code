const INPUT: &str = include_str!("../input");

fn main() {
    let heightmap: Vec<Vec<u8>> = INPUT
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect();
    println!("{}", part_one(&heightmap));
    println!("{}", part_two(&heightmap));
}

fn part_one(heightmap: &[Vec<u8>]) -> u32 {
    let heightmap_height = heightmap.len();
    let heightmap_width = heightmap[0].len();
    let mut total_risk_level = 0;
    for (y, row) in heightmap.iter().enumerate() {
        for (x, &height) in row.iter().enumerate() {
            if (x == 0 || height < heightmap[y][x - 1])
                && (y == 0 || height < heightmap[y - 1][x])
                && (x + 1 == heightmap_width || height < heightmap[y][x + 1])
                && (y + 1 == heightmap_height || height < heightmap[y + 1][x])
            {
                total_risk_level += 1 + height as u32;
            }
        }
    }
    total_risk_level
}

fn part_two(heightmap: &[Vec<u8>]) -> u32 {
    let heightmap_height = heightmap.len();
    let heightmap_width = heightmap[0].len();
    let mut visited = vec![vec![false; heightmap_width]; heightmap_height];
    let mut basin_sizes = vec![];
    for y in 0..heightmap_height {
        for x in 0..heightmap_width {
            if visited[y][x] || heightmap[y][x] == 9 {
                continue;
            }
            let mut basin_size = 0;
            let mut stack = vec![(y, x)];
            while let Some((y, x)) = stack.pop() {
                if visited[y][x] {
                    continue;
                }
                basin_size += 1;
                visited[y][x] = true;
                if x > 0 && !visited[y][x - 1] && heightmap[y][x - 1] < 9 {
                    stack.push((y, x - 1));
                }
                if y > 0 && !visited[y - 1][x] && heightmap[y - 1][x] < 9 {
                    stack.push((y - 1, x));
                }
                if x + 1 < heightmap_width && !visited[y][x + 1] && heightmap[y][x + 1] < 9 {
                    stack.push((y, x + 1));
                }
                if y + 1 < heightmap_height && !visited[y + 1][x] && heightmap[y + 1][x] < 9 {
                    stack.push((y + 1, x));
                }
            }
            basin_sizes.push(basin_size);
        }
    }
    basin_sizes.sort_unstable();
    basin_sizes[basin_sizes.len() - 3..].iter().product()
}
