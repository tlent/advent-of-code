use std::{
    cell::RefCell,
    collections::{btree_map::Entry, BTreeMap},
};

pub const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Clone)]
pub struct Monkey {
    held_items: Vec<usize>,
    inspection_count: usize,
    operation: Operation,
    test: Test,
}

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Add(usize),
    Multiply(usize),
    Square,
}

#[derive(Debug, Clone, Copy)]
pub struct Test {
    divisor: usize,
    true_destination: usize,
    false_destination: usize,
}

// RefCell is necessary for the `round()` function to move items
// from one monkey's `held_items` to another monkey's `held_items`
type Monkeys = Vec<RefCell<Monkey>>;

pub fn parse_input(input: &str) -> Monkeys {
    input
        .split("\n\n")
        .map(|s| {
            let mut iter = s.lines();
            let held_items_str = &iter.nth(1).unwrap()[18..];
            let held_items = held_items_str
                .split(", ")
                .map(|s| s.parse().unwrap())
                .collect();
            let operation_str = &iter.next().unwrap()[23..];
            let operation = match operation_str {
                "* old" => Operation::Square,
                s if s.starts_with('*') => Operation::Multiply(s[2..].parse().unwrap()),
                s if s.starts_with('+') => Operation::Add(s[2..].parse().unwrap()),
                _ => panic!("Unknown operation: {operation_str}"),
            };
            let divisor = iter.next().unwrap()[21..].parse().unwrap();
            let true_destination = iter.next().unwrap()[29..].parse().unwrap();
            let false_destination = iter.next().unwrap()[30..].parse().unwrap();
            RefCell::new(Monkey {
                held_items,
                inspection_count: 0,
                operation,
                test: Test {
                    divisor,
                    true_destination,
                    false_destination,
                },
            })
        })
        .collect()
}

pub fn part_one(monkeys: &mut Monkeys) -> usize {
    const ROUNDS: usize = 20;
    for _ in 0..ROUNDS {
        round(monkeys, |v| v / 3);
    }
    monkey_business_level(monkeys)
}

pub fn part_two(monkeys: &mut Monkeys) -> usize {
    const ROUNDS: usize = 10_000;
    let test_divisors_product = monkeys
        .iter()
        .map(|monkey| monkey.borrow().test.divisor)
        .product::<usize>();
    let mut seen = BTreeMap::default();
    let mut previous_inspection_counts = vec![];
    for round_number in 0..ROUNDS {
        previous_inspection_counts.push(get_inspection_counts(monkeys));
        let monkeys_held_items = monkeys
            .iter()
            .map(|m| m.borrow().held_items.clone())
            .collect::<Vec<_>>();
        match seen.entry(monkeys_held_items) {
            Entry::Vacant(v) => v.insert(round_number),
            Entry::Occupied(o) => {
                let cycle_start = *o.get();
                let cycle_len = round_number - cycle_start;
                let remaining_rounds = ROUNDS - round_number;
                let remaining_cycles = remaining_rounds / cycle_len;
                let remainder = remaining_rounds % cycle_len;
                let cycle_counts = &previous_inspection_counts[cycle_start..];
                let cycle_start_counts = &cycle_counts[0];
                let cycle_end_counts = cycle_counts.last().unwrap();
                let remainder_counts = &cycle_counts[remainder];
                for (i, monkey) in monkeys.iter_mut().enumerate() {
                    let cycle_increment = cycle_end_counts[i] - cycle_start_counts[i];
                    let remainder_increment = remainder_counts[i] - cycle_start_counts[i];
                    monkey.get_mut().inspection_count +=
                        cycle_increment * remaining_cycles + remainder_increment;
                }
                break;
            }
        };
        round(monkeys, |v| v % test_divisors_product);
    }
    monkey_business_level(monkeys)
}

fn round<F>(monkeys: &mut Monkeys, map_operation_result: F)
where
    F: Fn(usize) -> usize,
{
    for monkey_refcell in monkeys.iter() {
        let mut monkey = monkey_refcell.borrow_mut();
        monkey.inspection_count += monkey.held_items.len();
        let Test {
            divisor,
            true_destination,
            false_destination,
        } = monkey.test;
        let operation = monkey.operation;
        for item in monkey.held_items.drain(..) {
            let result = map_operation_result(match operation {
                Operation::Add(n) => item + n,
                Operation::Multiply(n) => item * n,
                Operation::Square => item * item,
            });
            let destination = if result % divisor == 0 {
                true_destination
            } else {
                false_destination
            };
            monkeys[destination].borrow_mut().held_items.push(result);
        }
    }
}

fn monkey_business_level(monkeys: &Monkeys) -> usize {
    let mut inspection_counts = get_inspection_counts(monkeys);
    inspection_counts.sort_unstable();
    inspection_counts.into_iter().rev().take(2).product()
}

fn get_inspection_counts(monkeys: &Monkeys) -> Vec<usize> {
    monkeys
        .iter()
        .map(|m| m.borrow().inspection_count)
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let mut monkeys = parse_input(INPUT);
        assert_eq!(part_one(&mut monkeys), 69918);
    }

    #[test]
    fn test_part_two() {
        let mut monkeys = parse_input(INPUT);
        assert_eq!(part_two(&mut monkeys), 19_573_408_701);
    }
}
