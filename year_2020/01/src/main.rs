use std::collections::HashSet;
use std::num::ParseIntError;

static INPUT: &str = include_str!("../input");

fn main() {
    let target = 2020;
    let numbers = parse_input(INPUT).expect("failed to parse input");
    println!("part one: {}", part_one(&numbers, target));
    println!("part two: {}", part_two(&numbers, target));
}

fn parse_input(input: &str) -> Result<Vec<u32>, ParseIntError> {
    input.lines().map(|l| l.parse()).collect()
}

fn part_one(numbers: &[u32], target: u32) -> u32 {
    let mut seen = HashSet::new();
    for x in numbers {
        let y = target - x;
        if seen.contains(&y) {
            return x * y;
        }
        seen.insert(x);
    }
    panic!("no solution found");
}

fn part_two(numbers: &[u32], target: u32) -> u32 {
    let mut seen = HashSet::new();
    for x in numbers {
        for y in numbers {
            let z = match target.checked_sub(x + y) {
                Some(v) => v,
                None => continue,
            };
            if seen.contains(&z) {
                return x * y * z;
            }
            seen.insert(x);
            seen.insert(y);
        }
    }
    panic!("no solution found");
}
