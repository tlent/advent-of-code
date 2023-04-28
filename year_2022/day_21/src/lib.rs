use rustc_hash::FxHashMap as HashMap;

pub const INPUT: &str = include_str!("../input.txt");

pub enum Value {
    Number(i64),
    BinaryOp(String, Operation, String),
}

pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operation {
    fn inverse(&self) -> Self {
        match self {
            Self::Add => Self::Sub,
            Self::Sub => Self::Add,
            Self::Mul => Self::Div,
            Self::Div => Self::Mul,
        }
    }

    fn apply(&self, left: i64, right: i64) -> i64 {
        match self {
            Self::Add => left + right,
            Self::Sub => left - right,
            Self::Mul => left * right,
            Self::Div => left / right,
        }
    }
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
    todo!()
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        todo!()
    }

    #[test]
    fn test_part_two() {
        todo!()
    }
}
