static INPUT: &str = include_str!("../input");

fn main() {
    let map = parse_map(INPUT);
    println!("part one: {}", part_one(&map));
    println!("part two: {}", part_two(&map));
}

#[derive(PartialEq, Eq)]
enum AreaStatus {
    Open,
    Tree,
}

struct Map {
    lines: usize,
    line_width: usize,
    data: Vec<AreaStatus>,
}

fn parse_map(input: &str) -> Map {
    let lines = input.lines().count();
    let line_width = input.lines().next().unwrap().len();
    let data = input
        .chars()
        .filter_map(|c| match c {
            '.' => Some(AreaStatus::Open),
            '#' => Some(AreaStatus::Tree),
            '\n' => None,
            _ => panic!("invalid char in input"),
        })
        .collect();
    Map {
        lines,
        line_width,
        data,
    }
}

fn count_collisions(map: &Map, slope: (usize, usize)) -> u32 {
    let (dx, dy) = slope;
    let (mut current_x, mut current_y) = slope;
    let mut collisions = 0;
    while current_y < map.lines {
        let index = current_y * map.line_width + current_x % map.line_width;
        if map.data[index] == AreaStatus::Tree {
            collisions += 1;
        }
        current_x += dx;
        current_y += dy;
    }
    collisions
}

fn part_one(map: &Map) -> u32 {
    count_collisions(map, (3, 1))
}

fn part_two(map: &Map) -> u32 {
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut product = 1;
    for &s in &slopes {
        product *= count_collisions(map, s);
    }
    product
}
