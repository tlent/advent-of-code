use std::{num::ParseIntError, str::FromStr};

pub const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Integer(u32),
    List(Vec<Value>),
}

impl FromStr for Value {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('[') && s.ends_with(']') {
            let mut strs = vec![];
            let mut level = 0;
            let mut start = 1;
            for (end, b) in s.bytes().enumerate().skip(1) {
                match b {
                    b']' if level == 0 => {
                        if start < end {
                            strs.push(&s[start..end]);
                        }
                    }
                    b',' if level == 0 => {
                        strs.push(&s[start..end]);
                        start = end + 1;
                    }
                    b'[' => level += 1,
                    b']' => level -= 1,
                    _ => {}
                }
            }
            let values = strs
                .into_iter()
                .map(|s| s.parse())
                .collect::<Result<_, _>>()?;
            Ok(Value::List(values))
        } else {
            Ok(Value::Integer(s.parse()?))
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Value::Integer(s), Value::Integer(o)) => s.cmp(&o),
            (Value::List(s), Value::List(o)) => s.cmp(&o),
            (Value::Integer(_), Value::List(_)) => Value::List(vec![self.clone()]).cmp(other),
            (Value::List(_), Value::Integer(_)) => self.cmp(&Value::List(vec![other.clone()])),
        }
    }
}

pub fn parse_input(input: &str) -> Vec<Value> {
    input
        .lines()
        .filter_map(|line| {
            if line.is_empty() {
                return None;
            }
            Some(line.parse().unwrap())
        })
        .collect()
}

pub fn part_one(packets: &[Value]) -> usize {
    packets
        .chunks_exact(2)
        .enumerate()
        .filter_map(|(i, pair)| if pair[0] < pair[1] { Some(i + 1) } else { None })
        .sum()
}

pub fn part_two(mut packets: Vec<Value>) -> usize {
    let decoder_keys = [
        Value::List(vec![Value::List(vec![Value::Integer(2)])]),
        Value::List(vec![Value::List(vec![Value::Integer(6)])]),
    ];
    packets.extend(decoder_keys.clone());
    packets.sort_unstable();
    (packets.iter().position(|p| p == &decoder_keys[0]).unwrap() + 1)
        * (packets.iter().position(|p| p == &decoder_keys[1]).unwrap() + 1)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let packets = parse_input(INPUT);
        assert_eq!(part_one(&packets), 6478);
    }

    #[test]
    fn test_part_two() {
        let packets = parse_input(INPUT);
        assert_eq!(part_two(packets), 21_922);
    }
}
