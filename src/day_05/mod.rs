use std::cmp::Ordering;

pub const INPUT: &str = include_str!("./input.txt");

pub fn parse_input(input: &str) -> (Vec<Vec<u8>>, Vec<Move>) {
    let (stacks_str, moves_str) = input.split_once("\n\n").unwrap();
    let stack_lines: Vec<_> = stacks_str
        .lines()
        .rev()
        .skip(1)
        .map(str::as_bytes)
        .collect();
    let stack_line_len = stack_lines[0].len();
    let stacks = (1..stack_line_len)
        .step_by(4)
        .map(|index| {
            stack_lines
                .iter()
                .map(|line| line[index])
                .filter(|&byte| byte != b' ')
                .collect()
        })
        .collect();
    let moves = moves_str
        .lines()
        .map(|line| {
            let mut parts = line.split_ascii_whitespace();
            let count = parts.nth(1).unwrap().parse().unwrap();
            let source: usize = parts.nth(1).unwrap().parse().unwrap();
            let destination: usize = parts.nth(1).unwrap().parse().unwrap();
            Move {
                count,
                source_index: source - 1,
                destination_index: destination - 1,
            }
        })
        .collect();
    (stacks, moves)
}

pub fn part_one(mut stacks: Vec<Vec<u8>>, moves: &[Move]) -> String {
    for &Move {
        count,
        source_index,
        destination_index,
    } in moves
    {
        for _ in 0..count {
            let byte = stacks[source_index].pop().unwrap();
            stacks[destination_index].push(byte);
        }
    }
    let last_bytes: Vec<_> = stacks
        .iter()
        .filter_map(|stack| stack.last().copied())
        .collect();
    unsafe { String::from_utf8_unchecked(last_bytes) }
}

pub fn part_two(mut stacks: Vec<Vec<u8>>, moves: &[Move]) -> String {
    for &Move {
        count,
        source_index,
        destination_index,
    } in moves
    {
        let (source, destination) = stacks.get_two_mut(source_index, destination_index);
        let source_len = source.len();
        destination.extend(source.drain(source_len - count..));
    }
    let last_bytes: Vec<_> = stacks
        .iter()
        .filter_map(|stack| stack.last().copied())
        .collect();
    unsafe { String::from_utf8_unchecked(last_bytes) }
}

trait SliceExt {
    type Item;

    fn get_two_mut(&mut self, index0: usize, index1: usize) -> (&mut Self::Item, &mut Self::Item);
}

impl<T> SliceExt for [T] {
    type Item = T;

    fn get_two_mut(&mut self, index0: usize, index1: usize) -> (&mut Self::Item, &mut Self::Item) {
        match index0.cmp(&index1) {
            Ordering::Less => {
                let mut iter = self.iter_mut();
                let item0 = iter.nth(index0).unwrap();
                let item1 = iter.nth(index1 - index0 - 1).unwrap();
                (item0, item1)
            }
            Ordering::Equal => {
                panic!("[T]::get_two_mut(): received same index twice ({})", index0)
            }
            Ordering::Greater => {
                let mut iter = self.iter_mut();
                let item1 = iter.nth(index1).unwrap();
                let item0 = iter.nth(index0 - index1 - 1).unwrap();
                (item0, item1)
            }
        }
    }
}

#[derive(Debug)]
pub struct Move {
    pub count: usize,
    pub source_index: usize,
    pub destination_index: usize,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let (stacks, moves) = parse_input(INPUT);
        assert_eq!(part_one(stacks, &moves), "PSNRGBTFT");
    }

    #[test]
    fn test_part_two() {
        let (stacks, moves) = parse_input(INPUT);
        assert_eq!(part_two(stacks, &moves), "BNTZFPMMW");
    }
}
