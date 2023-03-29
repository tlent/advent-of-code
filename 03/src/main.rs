use std::collections::HashSet;

const INPUT: &str = include_str!("../input.txt");

fn priority(byte: u8) -> u32 {
    (match byte {
        b'a'..=b'z' => 1 + byte - b'a',
        b'A'..=b'Z' => 27 + byte - b'A',
        _ => panic!("Invalid byte"),
    }) as u32
}

fn find_common_byte(strs: &[&str]) -> Option<u8> {
    strs.iter()
        .map(|str| str.bytes().collect::<HashSet<_>>())
        .reduce(|a, b| a.intersection(&b).copied().collect())
        .and_then(|set| set.iter().next().copied())
}

fn main() {
    let lines: Vec<_> = INPUT.lines().collect();
    let part_one_solution = lines
        .iter()
        .map(|line| {
            let (left, right) = line.split_at(line.len() / 2);
            let common_byte = find_common_byte(&[left, right]).unwrap();
            priority(common_byte)
        })
        .sum::<u32>();
    println!("{}", part_one_solution);
    let part_two_solution = lines
        .chunks_exact(3)
        .map(|chunk_lines| {
            let common_byte = find_common_byte(chunk_lines).unwrap();
            priority(common_byte)
        })
        .sum::<u32>();
    println!("{}", part_two_solution);
}
