pub const INPUT: &str = include_str!("./input.txt");

pub fn part_one(lines: &[&str]) -> u32 {
    lines
        .iter()
        .map(|line| {
            let (left, right) = line.split_at(line.len() / 2);
            let common_byte = find_common_byte(&[left, right]).unwrap();
            priority(common_byte)
        })
        .sum::<u32>()
}

pub fn part_two(lines: &[&str]) -> u32 {
    lines
        .chunks_exact(3)
        .map(|chunk_lines| {
            let common_byte = find_common_byte(chunk_lines).unwrap();
            priority(common_byte)
        })
        .sum::<u32>()
}

fn find_common_byte(strs: &[&str]) -> Option<u8> {
    for byte in strs[0].bytes() {
        if strs[1..].iter().all(|str| str.bytes().any(|b| b == byte)) {
            return Some(byte);
        }
    }
    None
}

fn priority(byte: u8) -> u32 {
    (match byte {
        b'a'..=b'z' => 1 + byte - b'a',
        b'A'..=b'Z' => 27 + byte - b'A',
        _ => panic!("Invalid byte"),
    }) as u32
}
