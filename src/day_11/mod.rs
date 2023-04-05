use crate::HashMap;
use std::cell::RefCell;

pub const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
pub struct Monkey {
    starting_items: Vec<usize>,
    operation: Operation,
    test: Test,
}

#[derive(Debug)]
pub enum Operation {
    Add(usize),
    Multiply(usize),
    Square,
}

#[derive(Debug)]
pub struct Test {
    divisor: usize,
    true_destination: usize,
    false_destination: usize,
}

pub fn parse_input(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|s| {
            let mut iter = s.lines();
            let starting_items_str = &iter.nth(1).unwrap()[18..];
            let starting_items = starting_items_str
                .split(", ")
                .map(|s| s.parse().unwrap())
                .collect();
            let operation_str = &iter.next().unwrap()[23..];
            let operation = match operation_str {
                "* old" => Operation::Square,
                s if s.starts_with("*") => Operation::Multiply(s[2..].parse().unwrap()),
                s if s.starts_with("+") => Operation::Add(s[2..].parse().unwrap()),
                _ => panic!("Unknown operation: {operation_str}"),
            };
            let divisor = iter.next().unwrap()[21..].parse().unwrap();
            let true_destination = iter.next().unwrap()[29..].parse().unwrap();
            let false_destination = iter.next().unwrap()[30..].parse().unwrap();
            Monkey {
                starting_items,
                operation,
                test: Test {
                    divisor,
                    true_destination,
                    false_destination,
                },
            }
        })
        .collect()
}

pub fn part_one(monkeys: &[Monkey]) -> usize {
    const ROUNDS: usize = 20;
    let monkey_items = monkeys
        .iter()
        .map(|monkey| RefCell::new(monkey.starting_items.clone()))
        .collect::<Vec<_>>();
    let mut monkey_counts = vec![0; monkeys.len()];
    for _round in 0..ROUNDS {
        round(monkeys, &monkey_items, &mut monkey_counts, |v| v / 3);
    }
    top_two_product(monkey_counts)
}

pub fn part_two(monkeys: &[Monkey]) -> usize {
    const ROUNDS: usize = 10_000;
    let test_divisors_product = monkeys
        .iter()
        .map(|monkey| monkey.test.divisor)
        .product::<usize>();
    let monkey_items = monkeys
        .iter()
        .map(|monkey| RefCell::new(monkey.starting_items.clone()))
        .collect::<Vec<_>>();
    let mut monkey_counts = vec![0; monkeys.len()];
    let mut seen = HashMap::default();
    let mut previous_monkey_counts = vec![];
    for round_number in 0..ROUNDS {
        previous_monkey_counts.push(monkey_counts.clone());
        let inner_items = monkey_items
            .clone()
            .into_iter()
            .map(|i| i.into_inner())
            .collect::<Vec<_>>();
        if let Some(&cycle_start) = seen.get(&inner_items) {
            let cycle_len = round_number - cycle_start;
            let remaining_rounds = ROUNDS - round_number;
            let remaining_cycles = remaining_rounds / cycle_len;
            let remainder = remaining_rounds % cycle_len;
            let cycle_counts = &previous_monkey_counts[cycle_start..];
            let cycle_start_counts = &cycle_counts[0];
            let cycle_end_counts = cycle_counts.last().unwrap();
            let remainder_counts = &cycle_counts[remainder];
            for (i, count) in monkey_counts.iter_mut().enumerate() {
                let cycle_increment = cycle_end_counts[i] - cycle_start_counts[i];
                let remainder_increment = remainder_counts[i] - cycle_start_counts[i];
                *count += cycle_increment * remaining_cycles + remainder_increment;
            }
            break;
        }
        seen.insert(inner_items, round_number);
        round(monkeys, &monkey_items, &mut monkey_counts, |v| {
            v % test_divisors_product
        });
    }
    top_two_product(monkey_counts)
}

fn round<F>(
    monkeys: &[Monkey],
    monkey_items: &[RefCell<Vec<usize>>],
    monkey_counts: &mut [usize],
    map_operation_result: F,
) where
    F: Fn(usize) -> usize,
{
    for (i, monkey) in monkeys.iter().enumerate() {
        let mut items = monkey_items[i].borrow_mut();
        monkey_counts[i] += items.len();
        let Test {
            divisor,
            true_destination,
            false_destination,
        } = monkey.test;
        for item in items.drain(..) {
            let result = map_operation_result(match monkey.operation {
                Operation::Add(n) => item + n,
                Operation::Multiply(n) => item * n,
                Operation::Square => item * item,
            });
            let destination = if result % divisor == 0 {
                true_destination
            } else {
                false_destination
            };
            monkey_items[destination].borrow_mut().push(result);
        }
    }
}

fn top_two_product(mut values: Vec<usize>) -> usize {
    values.sort_unstable();
    values.into_iter().rev().take(2).product()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let monkeys = parse_input(INPUT);
        assert_eq!(part_one(&monkeys), 69918);
    }

    #[test]
    fn test_part_two() {
        let monkeys = parse_input(INPUT);
        assert_eq!(part_two(&monkeys), 19573408701);
    }
}
