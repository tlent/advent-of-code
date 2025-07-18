const INPUT: &str = include_str!("../input");

fn main() {
    let positions: Vec<u32> = INPUT
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    println!("{}", part_one(&positions));
    println!("{}", part_two(&positions));
}

fn part_one(positions: &[u32]) -> u32 {
    find_min_fuel_cost(positions, fuel_cost_part_one)
}

fn part_two(positions: &[u32]) -> u32 {
    find_min_fuel_cost(positions, fuel_cost_part_two)
}

fn find_min_fuel_cost<F>(positions: &[u32], cost_function: F) -> u32
where
    F: Fn(&[u32], u32) -> u32,
{
    let average = positions.iter().sum::<u32>() / positions.len() as u32;
    let mut min_cost = cost_function(positions, average);
    let mut stack = vec![average + 1];
    if average > 0 {
        stack.push(average - 1);
    }
    while let Some(p) = stack.pop() {
        let cost = cost_function(positions, p);
        if cost < min_cost {
            if p < average && p > 0 {
                stack.push(p - 1);
            } else if p > average {
                stack.push(p + 1);
            }
            min_cost = cost;
        }
    }
    min_cost
}

fn fuel_cost_part_one(positions: &[u32], target_position: u32) -> u32 {
    positions
        .iter()
        .map(|&p| {
            if p > target_position {
                p - target_position
            } else {
                target_position - p
            }
        })
        .sum()
}

fn fuel_cost_part_two(positions: &[u32], target_position: u32) -> u32 {
    positions
        .iter()
        .map(|&p| {
            let abs_diff = if p > target_position {
                p - target_position
            } else {
                target_position - p
            };
            (abs_diff * (abs_diff + 1)) / 2
        })
        .sum()
}
