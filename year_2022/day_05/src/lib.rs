use std::{cell::RefCell, num::ParseIntError};

pub const INPUT: &str = include_str!("../input.txt");

#[derive(Debug)]
pub struct Move {
    pub count: usize,
    pub source_index: usize,
    pub destination_index: usize,
}

pub fn parse_input(input: &str) -> Result<(Vec<Vec<u8>>, Vec<Move>), ParseIntError> {
    let (stacks_str, moves_str) = input.split_once("\n\n").unwrap();
    let stacks = parse_stacks(stacks_str);
    let moves = parse_moves(moves_str)?;
    Ok((stacks, moves))
}

fn parse_stacks(stacks_str: &str) -> Vec<Vec<u8>> {
    let mut lines_iter = stacks_str.lines().peekable();
    let line_len = lines_iter.peek().unwrap().len();
    // 4 bytes per "[X] ", +1 because no space at end of line
    let stack_count = (line_len + 1) / 4;
    let mut stacks = vec![vec![]; stack_count];
    for line in lines_iter {
        for (i, stack) in stacks.iter_mut().enumerate() {
            let byte = line.as_bytes()[i * 4 + 1];
            if byte.is_ascii_uppercase() {
                stack.push(byte);
            }
        }
    }
    stacks.iter_mut().for_each(|stack| stack.reverse());
    stacks
}

pub fn parse_moves(moves_str: &str) -> Result<Vec<Move>, ParseIntError> {
    let mut moves = vec![];
    let mut chunk = [0; 3];
    for (i, part) in moves_str.split(['\n', ' ']).skip(1).step_by(2).enumerate() {
        let value = part.parse::<usize>()?;
        chunk[i % 3] = value;
        if i % 3 == 2 {
            moves.push(Move {
                count: chunk[0],
                source_index: chunk[1] - 1,
                destination_index: chunk[2] - 1,
            });
        }
    }
    Ok(moves)
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
    get_top_of_stacks(&stacks)
}

pub fn part_two(stacks: Vec<Vec<u8>>, moves: &[Move]) -> String {
    let stacks = stacks.into_iter().map(RefCell::new).collect::<Vec<_>>();
    for &Move {
        count,
        source_index,
        destination_index,
    } in moves
    {
        let mut source = stacks[source_index].borrow_mut();
        let mut destination = stacks[destination_index].borrow_mut();
        let source_len = source.len();
        destination.extend(source.drain(source_len - count..));
    }
    let stacks = stacks
        .into_iter()
        .map(RefCell::into_inner)
        .collect::<Vec<_>>();
    get_top_of_stacks(&stacks)
}

fn get_top_of_stacks(stacks: &[Vec<u8>]) -> String {
    let tops = stacks
        .iter()
        .filter_map(|stack| stack.last().copied())
        .collect();
    unsafe { String::from_utf8_unchecked(tops) }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let (stacks, moves) = parse_input(INPUT).unwrap();
        assert_eq!(part_one(stacks, &moves), "PSNRGBTFT");
    }

    #[test]
    fn test_part_two() {
        let (stacks, moves) = parse_input(INPUT).unwrap();
        assert_eq!(part_two(stacks, &moves), "BNTZFPMMW");
    }
}
