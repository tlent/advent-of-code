pub const INPUT: &str = include_str!("../input.txt");

pub fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn part_one(lines: &[&str]) -> u32 {
    let mut part_numbers: Vec<u32> = vec![];
    for (y, line) in lines.iter().enumerate() {
        let mut number_start = None;
        let mut adjacent_to_symbol = false;
        for (x, b) in line.bytes().enumerate() {
            match b {
                b'0'..=b'9' => {
                    if number_start.is_none() {
                        number_start = Some(x);
                        adjacent_to_symbol = x
                            .checked_sub(1)
                            .map(|x_sub| {
                                is_symbol(line.as_bytes()[x_sub])
                                    || is_adjacent_to_symbol(lines, (x_sub, y))
                            })
                            .unwrap_or(false);
                    }
                    adjacent_to_symbol |= is_adjacent_to_symbol(lines, (x, y))
                }
                b'.' => {
                    if let Some(start) = number_start {
                        adjacent_to_symbol |= is_adjacent_to_symbol(lines, (x, y));
                        if adjacent_to_symbol {
                            let part_number = line[start..x].parse().unwrap();
                            part_numbers.push(part_number);
                        }
                        number_start = None;
                        adjacent_to_symbol = false;
                    }
                }
                _ => {
                    if let Some(start) = number_start {
                        let part_number = line[start..x].parse().unwrap();
                        part_numbers.push(part_number);
                        number_start = None;
                        adjacent_to_symbol = false;
                    }
                }
            }
        }
        if number_start.is_some() && adjacent_to_symbol {
            let part_number = line[number_start.unwrap()..].parse().unwrap();
            part_numbers.push(part_number);
        }
    }
    part_numbers.into_iter().sum()
}

fn is_adjacent_to_symbol(lines: &[&str], (x, y): (usize, usize)) -> bool {
    [y.checked_sub(1).map(|y_sub| (x, y_sub)), Some((x, y + 1))]
        .iter()
        .filter_map(|&coord| coord.and_then(|(x, y)| lines.get(y).map(|line| line.as_bytes()[x])))
        .any(is_symbol)
}

fn is_symbol(b: u8) -> bool {
    b != b'.' && !b.is_ascii_digit()
}

pub fn part_two(lines: &[&str]) -> u32 {
    todo!()
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
