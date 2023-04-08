use std::cmp::Ordering;

pub const INPUT: &str = include_str!("input.txt");

#[derive(Debug, PartialEq, Eq)]
pub enum Value<'a> {
    List(Values<'a>),
    Integer { value: u32, str: &'a str },
}

impl<'a> Value<'a> {
    fn new(s: &'a str) -> Self {
        if s.starts_with('[') && s.ends_with(']') {
            Value::List(Values::new(&s[1..s.len() - 1]))
        } else {
            Value::Integer {
                value: s.parse().unwrap(),
                str: s,
            }
        }
    }
}

impl<'a> PartialOrd for Value<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for Value<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Value::Integer { value: s, .. }, Value::Integer { value: o, .. }) => s.cmp(o),
            (Value::List(s), Value::List(o)) => {
                let mut self_values = s.clone();
                let mut other_values = o.clone();
                loop {
                    match (self_values.next(), other_values.next()) {
                        (Some(s), Some(o)) => match s.cmp(&o) {
                            Ordering::Equal => {}
                            ord => return ord,
                        },
                        (Some(_), None) => return Ordering::Greater,
                        (None, Some(_)) => return Ordering::Less,
                        (None, None) => return Ordering::Equal,
                    }
                }
            }
            (Value::Integer { str, .. }, Value::List(_)) => {
                let self_as_list = Value::List(Values::new(str));
                self_as_list.cmp(other)
            }
            (Value::List(_), Value::Integer { str, .. }) => {
                let other_as_list = Value::List(Values::new(str));
                self.cmp(&other_as_list)
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Values<'a> {
    value_list_str: &'a str,
}

impl<'a> Values<'a> {
    fn new(value_list_str: &'a str) -> Self {
        Self { value_list_str }
    }
}

impl<'a> Iterator for Values<'a> {
    type Item = Value<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.value_list_str.is_empty() {
            return None;
        }
        let mut level = 0;
        for (end, b) in self.value_list_str.bytes().enumerate() {
            match b {
                b',' if level == 0 => {
                    let value = Value::new(&self.value_list_str[..end]);
                    self.value_list_str = &self.value_list_str[end + 1..];
                    return Some(value);
                }
                b'[' => level += 1,
                b']' => level -= 1,
                _ => {}
            }
        }
        let value = Value::new(&self.value_list_str);
        self.value_list_str = "";
        Some(value)
    }
}

pub fn parse_input(input: &str) -> Vec<Value> {
    input
        .lines()
        .filter_map(|line| {
            if line.is_empty() {
                return None;
            }
            Some(Value::new(line))
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

pub fn part_two(packets: &[Value]) -> usize {
    let decoder_keys = [Value::new("[[2]]"), Value::new("[[6]]")];
    let mut indices = [1, 2];
    for p in packets {
        if p < &decoder_keys[0] {
            indices[0] += 1;
            indices[1] += 1;
        } else if p < &decoder_keys[1] {
            indices[1] += 1;
        }
    }
    indices.iter().product()
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
        assert_eq!(part_two(&packets), 21_922);
    }
}
