use std::collections::VecDeque;

const INPUT: [u8; 9] = [3, 1, 8, 9, 4, 6, 5, 7, 2];
const SAMPLE_INPUT: [u8; 9] = [3, 8, 9, 1, 2, 5, 4, 6, 7];

fn main() {
    println!("part one: {}", part_one(&INPUT));
    println!("part two: {}", part_two(&INPUT));
}

fn part_one(cups: &[u8]) -> String {
    let mut cups: VecDeque<_> = cups.iter().cloned().collect();
    let min = *cups.iter().min().unwrap();
    let max = *cups.iter().max().unwrap();
    for _ in 0..100 {
        let current_cup = cups[0];
        cups.rotate_left(1);
        let mut picked_cups = [0; 3];
        for c in picked_cups.iter_mut() {
            *c = cups.pop_front().unwrap();
        }
        let mut destination_cup = if current_cup > min {
            current_cup - 1
        } else {
            max
        };
        while picked_cups.contains(&destination_cup) {
            destination_cup -= 1;
            if destination_cup < min {
                destination_cup = max;
            }
        }
        let rotate_size = cups.iter().position(|&c| c == destination_cup).unwrap() + 1;
        cups.rotate_left(rotate_size);
        for &c in picked_cups.iter().rev() {
            cups.push_front(c);
        }
        cups.rotate_right(rotate_size);
    }
    cups.rotate_left(cups.iter().position(|&c| c == 1).unwrap());
    cups.make_contiguous()[1..]
        .iter()
        .map(u8::to_string)
        .collect()
}

fn part_two(cups: &[u8]) -> u64 {
    let cups: Vec<_> = cups.iter().map(|&v| v as u32).collect();
    let min = *cups.iter().min().unwrap();
    let max = *cups.iter().max().unwrap();
    let mut current_cup = cups[0];
    let mut next_cup: Vec<_> = (1..=1_000_000).collect();
    for (&c, &next_c) in cups.iter().zip(cups.iter().skip(1)) {
        next_cup[c as usize] = next_c;
    }
    next_cup[cups[cups.len() - 1] as usize] = max + 1;
    next_cup.push(current_cup);
    let max = 1_000_000;
    for _ in 0..10_000_000 {
        let a = next_cup[current_cup as usize];
        let b = next_cup[a as usize];
        let c = next_cup[b as usize];
        let picked_cups = [a, b, c];
        next_cup[current_cup as usize] = next_cup[c as usize];
        let mut destination_cup = if current_cup > min {
            current_cup - 1
        } else {
            max
        };
        while picked_cups.contains(&destination_cup) {
            destination_cup -= 1;
            if destination_cup < min {
                destination_cup = max;
            }
        }
        next_cup[c as usize] = next_cup[destination_cup as usize];
        next_cup[destination_cup as usize] = a;
        current_cup = next_cup[current_cup as usize];
    }
    let a = next_cup[1];
    let b = next_cup[a as usize];
    a as u64 * b as u64
}
