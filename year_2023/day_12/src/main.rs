#![feature(test)]

extern crate test;

const INPUT: &str = include_str!("../input.txt");

fn parse_input(input: &str) -> Vec<(&str, Vec<usize>)> {
    input
        .lines()
        .map(|line| {
            let (row, group_sizes) = line.split_once(' ').unwrap();
            let group_sizes = group_sizes.split(',').map(|s| s.parse().unwrap()).collect();
            (row, group_sizes)
        })
        .collect()
}

fn part_one(input: &[(&str, Vec<usize>)]) -> usize {
    let mut arrangement_count = 0;
    for (row, specified_group_sizes) in input {
        let mut candidates = vec![vec![]];
        for b in row.bytes() {
            dbg!(candidates.len());
            if b == b'?' {
                let mut candidates_with_damaged = candidates.clone();
                for candidate in candidates_with_damaged.iter_mut() {
                    candidate.push(b'#');
                }
                for candidate in candidates.iter_mut() {
                    candidate.push(b'.');
                }
                candidates.extend(candidates_with_damaged);
                candidates.retain(|candidate| {
                    let candidate_group_sizes = group_sizes(candidate);
                    for (candidate_group_size, specified_group_size) in candidate_group_sizes
                        .iter()
                        .zip(specified_group_sizes.iter())
                    {
                        if candidate_group_size == specified_group_size {
                            continue;
                        }
                        return *candidate.last().unwrap() == b'#'
                            && candidate_group_size < specified_group_size;
                    }
                    return true;
                });
            } else {
                for candidate in candidates.iter_mut() {
                    candidate.push(b);
                }
            }
        }
        arrangement_count += candidates
            .iter()
            .filter(|c| &group_sizes(c) == specified_group_sizes)
            .count();
    }
    arrangement_count
}

fn group_sizes(bytes: &[u8]) -> Vec<usize> {
    let mut group_sizes = vec![];
    let mut current_group_size = 0;
    for &b in bytes {
        if b == b'#' {
            current_group_size += 1;
        } else if current_group_size > 0 {
            group_sizes.push(current_group_size);
            current_group_size = 0;
        }
    }
    if current_group_size > 0 {
        group_sizes.push(current_group_size);
    }
    group_sizes
}

fn part_two(input: &[(&str, Vec<usize>)]) -> u32 {
    todo!()
}

fn main() {
    let parse_result = parse_input(INPUT);
    match std::env::args().nth(1).as_deref() {
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
        let input = parse_input(INPUT);
        assert_eq!(part_one(&input), todo!());
    }

    #[test]
    fn test_part_two() {
        let input = parse_input(INPUT);
        assert_eq!(part_two(&input), todo!());
    }

    #[bench]
    fn bench_parse_input(b: &mut Bencher) {
        b.iter(|| parse_input(black_box(INPUT)));
    }

    #[bench]
    fn bench_part_one(b: &mut Bencher) {
        let input = parse_input(INPUT);
        b.iter(|| part_one(black_box(&input)));
    }

    #[bench]
    fn bench_part_two(b: &mut Bencher) {
        let input = parse_input(INPUT);
        b.iter(|| part_two(black_box(&input)));
    }
}
