use std::collections::HashMap;

static INPUT: [u32; 6] = [20, 9, 11, 0, 1, 2];
const ARRAY_MAX: u32 = 5_000_000;

fn main() {
    println!("part one: {}", part_one(&INPUT));
    println!("part two: {}", part_two(&INPUT));
}

fn nth_spoken(starting_numbers: &[u32], n: u32) -> u32 {
    let array_max = std::cmp::min(n, ARRAY_MAX);
    let mut spoken_turn_array: Vec<Option<u32>> = vec![None; array_max as usize];
    let mut spoken_turn_map = HashMap::new();
    let mut last_spoken = 0;
    for (turn, &v) in starting_numbers.iter().enumerate() {
        let turn = turn as u32;
        last_spoken = v;
        if v >= array_max {
            spoken_turn_map.insert(v, turn);
        } else {
            spoken_turn_array[v as usize] = Some(turn);
        }
    }
    for turn in starting_numbers.len() as u32..n {
        let prev = if last_spoken >= array_max {
            spoken_turn_map.insert(last_spoken, turn - 1)
        } else {
            std::mem::replace(&mut spoken_turn_array[last_spoken as usize], Some(turn - 1))
        };
        last_spoken = match prev {
            Some(v) => turn - 1 - v,
            None => 0,
        };
    }
    last_spoken
}

fn part_one(numbers: &[u32]) -> u32 {
    nth_spoken(numbers, 2020)
}

fn part_two(numbers: &[u32]) -> u32 {
    nth_spoken(numbers, 30_000_000)
}
