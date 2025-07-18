use std::collections::HashMap;

static INPUT: &str = include_str!("../input");

fn main() {
    let commands = parse_input(INPUT);
    println!("part one: {}", part_one(&commands));
    println!("part two: {}", part_two(&commands));
}

#[derive(Debug, Clone, Copy)]
enum Command {
    Mask { mask: u64, value: u64 },
    Mem { address: usize, value: u64 },
}

fn parse_input(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|l| {
            let mut words = l.split_whitespace();
            let left = words.next().unwrap();
            let right = words.nth(1).unwrap();
            if left == "mask" {
                let mut mask = 0u64;
                let mut value = 0u64;
                for c in right.chars() {
                    mask *= 2;
                    value *= 2;
                    match c {
                        'X' => mask += 1,
                        '1' => value += 1,
                        '0' => {}
                        _ => panic!("invalid char: {}", c),
                    }
                }
                Command::Mask { mask, value }
            } else {
                let end = left.rfind(']').unwrap();
                let address = left[4..end].parse().unwrap();
                let value = right.parse().unwrap();
                Command::Mem { address, value }
            }
        })
        .collect()
}

fn part_one(commands: &[Command]) -> u64 {
    let mut mem = HashMap::new();
    let mut mask = 0;
    let mut mask_value = 0;
    use Command::*;
    for c in commands {
        match c {
            Mask { mask: m, value: v } => {
                mask = *m;
                mask_value = *v;
            }
            Mem {
                address: a,
                value: v,
            } => {
                mem.insert(a, (v & mask) | mask_value);
            }
        }
    }
    mem.values().sum()
}

fn part_two(commands: &[Command]) -> u64 {
    let mut mem = HashMap::new();
    let mut mask = 0;
    let mut mask_value = 0;
    use Command::*;
    for &c in commands {
        match c {
            Mask { mask: m, value: v } => {
                mask = m;
                mask_value = v;
            }
            Mem { address, value: v } => {
                let address = address as u64 | mask_value;
                let mut addresses = vec![address];
                let mut b = 1;
                let mut m = mask;
                while m > 0 {
                    if m % 2 == 1 {
                        let mut new_addresses = Vec::with_capacity(2 * addresses.len());
                        for a in addresses {
                            new_addresses.push(a | b);
                            new_addresses.push(a & !b);
                        }
                        addresses = new_addresses;
                    }
                    m /= 2;
                    b *= 2;
                }
                for a in addresses {
                    mem.insert(a, v);
                }
            }
        }
    }
    mem.values().sum()
}
