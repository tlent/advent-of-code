use std::ops::RangeInclusive;

const INPUT: &str = include_str!("../input");

fn main() {
    let bounds = get_bounds(INPUT);
    let (solutions, max_vy) = solve(bounds);
    println!("{}", (max_vy * (max_vy + 1)) / 2);
    println!("{}", solutions.len());
}

fn get_bounds(input: &str) -> (RangeInclusive<i32>, RangeInclusive<i32>) {
    let s: String = input.trim().chars().skip(15).collect();
    let mut parts = s.split(", y=");
    let x_part = parts.next().unwrap();
    let mut x_values = x_part.split("..");
    let min_x = x_values.next().and_then(|v| v.parse().ok()).unwrap();
    let max_x = x_values.next().and_then(|v| v.parse().ok()).unwrap();
    let y_part = parts.next().unwrap();
    let mut y_values = y_part.split("..");
    let min_y = y_values.next().and_then(|v| v.parse().ok()).unwrap();
    let max_y = y_values.next().and_then(|v| v.parse().ok()).unwrap();
    (min_x..=max_x, min_y..=max_y)
}

fn solve((x_range, y_range): (RangeInclusive<i32>, RangeInclusive<i32>)) -> (Vec<(i32, i32)>, i32) {
    let mut solutions = vec![];
    let mut max_vy = -1000;
    for initial_vy in -1000..1000 {
        for initial_vx in 0..=*x_range.end() {
            let (mut x, mut y) = (0, 0);
            let mut vy = initial_vy;
            let mut vx = initial_vx;
            for _step in 0.. {
                x += vx;
                y += vy;
                vy -= 1;
                if vx > 0 {
                    vx -= 1;
                }
                if x_range.contains(&x) && y_range.contains(&y) {
                    solutions.push((initial_vx, initial_vy));
                    max_vy = std::cmp::max(max_vy, initial_vy);
                    break;
                }
                if x > *x_range.end() || (vy < 0 && y < *y_range.start()) {
                    break;
                }
            }
        }
    }
    (solutions, max_vy)
}
