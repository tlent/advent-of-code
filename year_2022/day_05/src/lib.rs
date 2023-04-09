use std::cell::RefCell;

pub const INPUT: &str = include_str!("../input.txt");

#[derive(Debug)]
pub struct Move {
    pub count: usize,
    pub source_index: usize,
    pub destination_index: usize,
}

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
        let (stacks, moves) = parse_input(INPUT);
        assert_eq!(part_one(stacks, &moves), "PSNRGBTFT");
    }

    #[test]
    fn test_part_two() {
        let (stacks, moves) = parse_input(INPUT);
        assert_eq!(part_two(stacks, &moves), "BNTZFPMMW");
    }
}
