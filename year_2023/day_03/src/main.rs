#![feature(test)]
extern crate test;

use std::env;

pub const INPUT: &str = include_str!("../input.txt");

pub fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn part_one(lines: &[&str]) -> u32 {
    let mut sum = 0;
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
                            let part_number: u32 = line[start..x].parse().unwrap();
                            sum += part_number;
                        }
                        number_start = None;
                        adjacent_to_symbol = false;
                    }
                }
                _ => {
                    if let Some(start) = number_start {
                        let part_number: u32 = line[start..x].parse().unwrap();
                        sum += part_number;
                        number_start = None;
                        adjacent_to_symbol = false;
                    }
                }
            }
        }
        if number_start.is_some() && adjacent_to_symbol {
            let part_number: u32 = line[number_start.unwrap()..].parse().unwrap();
            sum += part_number;
        }
    }
    sum
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
    let mut sum = 0;
    for (y, line) in lines.iter().enumerate() {
        let mut start = 0;
        while let Some(i) = line[start..].find('*') {
            if let Some(product) = two_adjacents_product(lines, (start + i, y)) {
                sum += product;
            }
            start += i + 1;
        }
    }
    sum
}

fn two_adjacents_product(lines: &[&str], (x, y): (usize, usize)) -> Option<u32> {
    let x_sub = x.checked_sub(1);
    let y_sub = y.checked_sub(1);
    let adjacent_coords = [
        x_sub.and_then(|x_sub| y_sub.map(|y_sub| (x_sub, y_sub))),
        y_sub.map(|y_sub| (x, y_sub)),
        y_sub.map(|y_sub| (x + 1, y_sub)),
        x_sub.map(|x_sub| (x_sub, y)),
        Some((x + 1, y)),
        x_sub.map(|x_sub| (x_sub, y + 1)),
        Some((x, y + 1)),
        Some((x + 1, y + 1)),
    ];
    let mut coord_used = [false; 8];
    let mut adjacent_number_count = 0;
    let mut product = 1;
    let iter = adjacent_coords.iter().flatten().enumerate();
    for (i, &(x, y)) in iter {
        if coord_used[i] || y >= lines.len() || x >= lines[0].len() {
            continue;
        }
        let line = lines[y];
        let bytes = line.as_bytes();
        if !bytes[x].is_ascii_digit() {
            continue;
        }
        adjacent_number_count += 1;
        if adjacent_number_count > 2 {
            return None;
        }
        let start = bytes[..x]
            .iter()
            .rposition(|b| !b.is_ascii_digit())
            .map(|i| i + 1)
            .unwrap_or(0);
        let end = bytes[x + 1..]
            .iter()
            .position(|b| !b.is_ascii_digit())
            .map(|i| i + x + 1)
            .unwrap_or(bytes.len());
        let number: u32 = line[start..end].parse().unwrap();
        product *= number;
        for x in start..end {
            let mut iter = adjacent_coords.iter().flatten();
            if let Some(i) = iter.position(|&c| c == (x, y)) {
                coord_used[i] = true;
            }
        }
    }
    if adjacent_number_count == 2 {
        Some(product)
    } else {
        None
    }
}

fn main() {
    let parse_result = parse_input(INPUT);
    match env::args().nth(1).as_deref() {
        Some("all") => {
            let part_one = part_one(&parse_result);
            println!("{part_one}");
            let part_two = part_two(&parse_result);
            println!("{part_two}");
        }
        Some("parse") => {}
        Some("one") => {
            let part_one = part_one(&parse_result);
            println!("{part_one}");
        }
        Some("two") => {
            let part_two = part_two(&parse_result);
            println!("{part_two}");
        }
        _ => println!("Invalid argument: must be one of all, parse, one, or two"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::hint::black_box;
    use test::Bencher;

    #[test]
    fn test_part_one() {
        let lines = parse_input(INPUT);
        assert_eq!(part_one(&lines), 544_664);
    }

    #[test]
    fn test_part_two() {
        let lines = parse_input(INPUT);
        assert_eq!(part_two(&lines), 84_495_585);
    }

    #[bench]
    fn bench_parse_input(b: &mut Bencher) {
        b.iter(|| parse_input(black_box(INPUT)));
    }

    #[bench]
    fn bench_part_one(b: &mut Bencher) {
        let lines = parse_input(INPUT);
        b.iter(|| part_one(black_box(&lines)));
    }

    #[bench]
    fn bench_part_two(b: &mut Bencher) {
        let lines = parse_input(INPUT);
        b.iter(|| part_two(black_box(&lines)));
    }
}
