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
    solve(monkeys, 20, |item| item / 3)
}

pub fn part_two(monkeys: &[Monkey]) -> usize {
    let modulo = monkeys
        .iter()
        .map(|monkey| monkey.test.divisor)
        .product::<usize>();
    solve(monkeys, 10_000, |item| item % modulo)
}

fn solve<F>(monkeys: &[Monkey], rounds: usize, after_operation: F) -> usize
where
    F: Fn(usize) -> usize,
{
    let monkey_items = monkeys
        .iter()
        .map(|monkey| RefCell::new(monkey.starting_items.clone()))
        .collect::<Vec<_>>();
    let mut monkey_counts = vec![0; monkeys.len()];
    for _ in 0..rounds {
        for (i, monkey) in monkeys.iter().enumerate() {
            let mut items = monkey_items[i].borrow_mut();
            monkey_counts[i] += items.len();
            let Test {
                divisor,
                true_destination,
                false_destination,
            } = monkey.test;
            for item in items.drain(..) {
                let result = after_operation(match monkey.operation {
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
    monkey_counts.sort_unstable();
    monkey_counts.into_iter().rev().take(2).product()
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
