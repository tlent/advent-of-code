use rustc_hash::FxHashMap as HashMap;

pub const INPUT: &str = include_str!("../input.txt");

#[derive(Debug)]
pub enum Value {
    Number(i64),
    BinaryOp(String, Operation, String),
}

#[derive(Debug)]
pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

type Monkeys = HashMap<String, Value>;

pub fn parse_input(input: &str) -> Monkeys {
    input
        .lines()
        .map(|line| {
            let (name_str, value_str) = line.split_once(": ").unwrap();
            let name = name_str.to_owned();
            if let Ok(number) = value_str.parse::<i64>() {
                return (name, Value::Number(number));
            }
            let mut parts = value_str.split_whitespace();
            let left = parts.next().unwrap().to_owned();
            let op_str = parts.next().unwrap();
            let op = match op_str {
                "+" => Operation::Add,
                "-" => Operation::Sub,
                "*" => Operation::Mul,
                "/" => Operation::Div,
                _ => panic!("Unknown operator {}", op_str),
            };
            let right = parts.next().unwrap().to_owned();
            let value = Value::BinaryOp(left, op, right);
            (name, value)
        })
        .collect()
}

pub fn part_one(monkeys: &Monkeys) -> i64 {
    find_value(monkeys, "root")
}

pub fn part_two(monkeys: &Monkeys) -> i64 {
    solve_for(monkeys, "root", "humn", None)
}

fn find_value(monkeys: &Monkeys, name: &str) -> i64 {
    match monkeys.get(name).unwrap() {
        Value::Number(number) => *number,
        Value::BinaryOp(left, op, right) => {
            let left_value = find_value(monkeys, left);
            let right_value = find_value(monkeys, right);
            op.apply(left_value, right_value)
        }
    }
}

fn search(monkeys: &Monkeys, root_name: &str, find_name: &str) -> bool {
    if root_name == find_name {
        return true;
    }
    match monkeys.get(root_name).unwrap() {
        Value::Number(_) => false,
        Value::BinaryOp(left, _, right) => {
            search(monkeys, left, find_name) || search(monkeys, right, find_name)
        }
    }
}

fn solve_for(
    monkeys: &Monkeys,
    root_name: &str,
    solve_name: &str,
    current_value: Option<i64>,
) -> i64 {
    if root_name == solve_name {
        return current_value.unwrap();
    }
    let value = monkeys.get(root_name).unwrap();
    let (left, op, right) = value.unwrap_binary_op();
    let (root_name, value) = if search(monkeys, left, solve_name) {
        let right_value = find_value(monkeys, right);
        let value = current_value.map_or(right_value, |v| match op {
            // y = x + z -> x = y - z
            Operation::Add => v - right_value,
            // y = x - z -> x = y + z
            Operation::Sub => v + right_value,
            // y = x * z -> x = y / z
            Operation::Mul => v / right_value,
            // y = x / z -> x = y * z
            Operation::Div => v * right_value,
        });
        (left, value)
    } else if search(monkeys, right, solve_name) {
        let left_value = find_value(monkeys, left);
        let value = current_value.map_or(left_value, |v| match op {
            // y = z + x -> x = y - z
            Operation::Add => v - left_value,
            // y = z - x -> x = z - y
            Operation::Sub => left_value - v,
            // y = z * x -> x = y / z
            Operation::Mul => v / left_value,
            // y = z / x -> x = z / y
            Operation::Div => left_value / v,
        });
        (right, value)
    } else {
        panic!("neither left nor right has solve name");
    };
    solve_for(monkeys, root_name, solve_name, Some(value))
}

impl Operation {
    fn apply(&self, left: i64, right: i64) -> i64 {
        match self {
            Self::Add => left + right,
            Self::Sub => left - right,
            Self::Mul => left * right,
            Self::Div => left / right,
        }
    }
}

impl Value {
    fn unwrap_binary_op(&self) -> (&str, &Operation, &str) {
        match self {
            Self::BinaryOp(left, op, right) => (left, op, right),
            _ => panic!("Called unwrap_binary_op on a Value that is not a BinaryOp"),
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let monkeys = parse_input(INPUT);
        assert_eq!(part_one(&monkeys), 364_367_103_397_416);
    }

    #[test]
    fn test_part_two() {
        let monkeys = parse_input(INPUT);
        assert_eq!(part_two(&monkeys), 3_782_852_515_583);
    }
}
