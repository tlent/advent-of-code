use std::collections::{btree_map::Entry, BTreeMap};

pub const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Clone)]
pub struct Monkey {
    held_items: Vec<u64>,
    inspection_count: u64,
    operation: Operation,
    test: Test,
}

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Add(u64),
    Multiply(u64),
    Square,
}

#[derive(Debug, Clone, Copy)]
pub struct Test {
    divisor: u64,
    true_destination: usize,
    false_destination: usize,
}

type Monkeys = Vec<Monkey>;

pub fn part_one(monkeys: &mut Monkeys) -> u64 {
    const ROUNDS: usize = 20;
    for _ in 0..ROUNDS {
        round(monkeys, |v| v / 3);
    }
    monkey_business_level(monkeys)
}

pub fn part_two(monkeys: &mut Monkeys) -> u64 {
    const ROUNDS: u64 = 10_000;
    let test_divisors_product = monkeys
        .iter()
        .map(|monkey| monkey.test.divisor)
        .product::<u64>();
    let mut seen = BTreeMap::default();
    let mut previous_inspection_counts = vec![];
    for round_number in 0..ROUNDS {
        previous_inspection_counts.push(get_inspection_counts(monkeys));
        let monkeys_held_items = monkeys
            .iter()
            .map(|m| m.held_items.clone())
            .collect::<Vec<_>>();
        match seen.entry(monkeys_held_items) {
            Entry::Vacant(v) => v.insert(round_number),
            Entry::Occupied(o) => {
                let cycle_start = *o.get();
                let cycle_len = round_number - cycle_start;
                let remaining_rounds = ROUNDS - round_number;
                let remaining_cycles = remaining_rounds / cycle_len;
                let remainder = remaining_rounds % cycle_len;
                let cycle_counts = &previous_inspection_counts[cycle_start as usize..];
                let cycle_start_counts = &cycle_counts[0];
                let cycle_end_counts = cycle_counts.last().unwrap();
                let remainder_counts = &cycle_counts[remainder as usize];
                for (i, monkey) in monkeys.iter_mut().enumerate() {
                    let cycle_increment = cycle_end_counts[i] - cycle_start_counts[i];
                    let remainder_increment = remainder_counts[i] - cycle_start_counts[i];
                    monkey.inspection_count +=
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
    F: Fn(u64) -> u64,
{
    for monkey_id in 0..monkeys.len() {
        let monkey = &mut monkeys[monkey_id];
        monkey.inspection_count += monkey.held_items.len() as u64;
        let Test {
            divisor,
            true_destination,
            false_destination,
        } = monkey.test;
        let operation = monkey.operation;
        while let Some(item) = monkeys[monkey_id].held_items.pop() {
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
            monkeys[destination].held_items.push(result);
        }
    }
}

fn monkey_business_level(monkeys: &Monkeys) -> u64 {
    let mut inspection_counts = get_inspection_counts(monkeys);
    inspection_counts.sort_unstable();
    inspection_counts.into_iter().rev().take(2).product()
}

fn get_inspection_counts(monkeys: &Monkeys) -> Vec<u64> {
    monkeys.iter().map(|m| m.inspection_count).collect()
}

pub mod parser {
    use super::{Monkey, Monkeys, Operation, Test};
    use anyhow::{anyhow, Result};
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{char, line_ending, u64},
        combinator::map,
        multi::separated_list0,
        sequence::{pair, preceded, terminated, tuple},
        Finish, IResult,
    };

    pub fn parse(input: &str) -> Result<Monkeys> {
        let (rest, monkeys) = monkeys(input)
            .finish()
            .map_err(|err| anyhow!(err.to_string()))?;
        if !rest.is_empty() {
            return Err(anyhow!("Unparsed input: {}", rest));
        }
        Ok(monkeys)
    }

    fn starting_items(input: &str) -> IResult<&str, Vec<u64>> {
        let start_tag = tag("  Starting items: ");
        let items = separated_list0(tag(", "), u64);
        let line = preceded(start_tag, items);
        terminated(line, line_ending)(input)
    }

    fn operation(input: &str) -> IResult<&str, Operation> {
        let start_tag = tag("  Operation: new = old ");
        let square = map(tag("* old"), |_| Operation::Square);
        let multiply = map(preceded(tag("* "), u64), Operation::Multiply);
        let add = map(preceded(tag("+ "), u64), Operation::Add);
        let operation = alt((square, multiply, add));
        let line = preceded(start_tag, operation);
        terminated(line, line_ending)(input)
    }

    fn test(input: &str) -> IResult<&str, Test> {
        let divisible_by_line =
            terminated(preceded(tag("  Test: divisible by "), u64), line_ending);
        let if_true_line = terminated(
            preceded(tag("    If true: throw to monkey "), u64),
            line_ending,
        );
        let if_false_line = terminated(
            preceded(tag("    If false: throw to monkey "), u64),
            line_ending,
        );
        let lines = tuple((divisible_by_line, if_true_line, if_false_line));
        let map_fn = |(divisor, true_destination, false_destination)| Test {
            divisor,
            true_destination: true_destination as usize,
            false_destination: false_destination as usize,
        };
        map(lines, map_fn)(input)
    }

    fn monkey(input: &str) -> IResult<&str, Monkey> {
        let monkey_line = terminated(preceded(tag("Monkey "), u64), pair(char(':'), line_ending));
        let lines = preceded(monkey_line, tuple((starting_items, operation, test)));
        let map_fn = |(starting_items, operation, test)| Monkey {
            held_items: starting_items,
            operation,
            test,
            inspection_count: 0,
        };
        map(lines, map_fn)(input)
    }

    fn monkeys(input: &str) -> IResult<&str, Monkeys> {
        separated_list0(line_ending, monkey)(input)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let mut monkeys = parser::parse(INPUT).unwrap();
        assert_eq!(part_one(&mut monkeys), 69918);
    }

    #[test]
    fn test_part_two() {
        let mut monkeys = parser::parse(INPUT).unwrap();
        assert_eq!(part_two(&mut monkeys), 19_573_408_701);
    }
}
