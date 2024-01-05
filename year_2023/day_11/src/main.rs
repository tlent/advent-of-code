#![feature(test)]
extern crate test;

const INPUT: &str = include_str!("../input.txt");

type Position = (usize, usize);

fn parse_input(input: &str) -> Vec<Position> {
    let mut positions = vec![];
    for (y, line) in input.lines().enumerate() {
        for (x, b) in line.bytes().enumerate() {
            if b == b'#' {
                positions.push((x, y));
            }
        }
    }
    positions
}

fn part_one(positions: &[Position]) -> usize {
    let mut sum = 0;
    for &position @ (x, y) in positions {
        for &other_position @ (other_x, other_y) in positions {
            if position < other_position {
                sum += x.abs_diff(other_x) + y.abs_diff(other_y);
            }
        }
    }
    sum
}

fn part_two(positions: &[Position]) -> usize {
    let mut sum = 0;
    for &position @ (x, y) in positions {
        for &other_position @ (other_x, other_y) in positions {
            if position < other_position {
                sum += x.abs_diff(other_x) + y.abs_diff(other_y);
            }
        }
    }
    sum
}

fn expand_positions(positions: &[(usize, usize)]) -> (Vec<Position>, Vec<Position>) {
    let max_x = *positions.iter().map(|(x, _)| x).max().unwrap();
    let max_y = *positions.iter().map(|(_, y)| y).max().unwrap();
    let mut column_has_galaxy = vec![false; max_x + 1];
    let mut row_has_galaxy = vec![false; max_y + 1];
    for &(x, y) in positions {
        column_has_galaxy[x] = true;
        row_has_galaxy[y] = true;
    }
    let mut column_expansion_amounts = Vec::with_capacity(max_x + 1);
    let mut expansion_amount = 0;
    for has_galaxy in column_has_galaxy {
        if !has_galaxy {
            expansion_amount += 1;
        }
        column_expansion_amounts.push(expansion_amount);
    }
    let mut row_expansion_amounts = Vec::with_capacity(max_y + 1);
    expansion_amount = 0;
    for has_galaxy in row_has_galaxy {
        if !has_galaxy {
            expansion_amount += 1;
        }
        row_expansion_amounts.push(expansion_amount);
    }
    let part_one_positions = positions
        .iter()
        .map(|&(x, y)| {
            (
                x + column_expansion_amounts[x],
                y + row_expansion_amounts[y],
            )
        })
        .collect();
    for amount in column_expansion_amounts.iter_mut() {
        *amount *= 999_999;
    }
    for amount in row_expansion_amounts.iter_mut() {
        *amount *= 999_999;
    }
    let part_two_positions = positions
        .iter()
        .map(|&(x, y)| {
            (
                x + column_expansion_amounts[x],
                y + row_expansion_amounts[y],
            )
        })
        .collect();
    (part_one_positions, part_two_positions)
}

fn main() {
    let base_positions = parse_input(INPUT);
    let (part_one_positions, part_two_positions) = expand_positions(&base_positions);
    match std::env::args().nth(1).as_deref() {
        Some("all") => {
            let part_one = part_one(&part_one_positions);
            println!("{part_one}");
            let part_two = part_two(&part_two_positions);
            println!("{part_two}");
        }
        Some("parse") => {}
        Some("one") => {
            let part_one = part_one(&part_one_positions);
            println!("{part_one}");
        }
        Some("two") => {
            let part_two = part_two(&part_two_positions);
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
        let base_positions = parse_input(INPUT);
        let (positions, _) = expand_positions(&base_positions);
        assert_eq!(part_one(&positions), 9_563_821);
    }

    #[test]
    fn test_part_two() {
        let base_positions = parse_input(INPUT);
        let (_, positions) = expand_positions(&base_positions);
        assert_eq!(part_two(&positions), 827_009_909_817);
    }

    #[bench]
    fn bench_parse_input(b: &mut Bencher) {
        b.iter(|| parse_input(black_box(INPUT)));
    }

    #[bench]
    fn bench_expand_positions(b: &mut Bencher) {
        let base_positions = parse_input(INPUT);
        b.iter(|| expand_positions(black_box(&base_positions)));
    }

    #[bench]
    fn bench_part_one(b: &mut Bencher) {
        let base_positions = parse_input(INPUT);
        let (positions, _) = expand_positions(&base_positions);
        b.iter(|| part_one(black_box(&positions)));
    }

    #[bench]
    fn bench_part_two(b: &mut Bencher) {
        let base_positions = parse_input(INPUT);
        let (_, positions) = expand_positions(&base_positions);
        b.iter(|| part_two(black_box(&positions)));
    }
}
