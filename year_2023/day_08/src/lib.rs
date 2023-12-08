pub const INPUT: &str = include_str!("../input.txt");

const SIZE: usize = 2usize.pow(15);
type Map = [(u16, u16); SIZE];

pub fn parse_input(input: &str) -> (&str, Map, Vec<u16>) {
    let mut lines = input.lines();
    let turns_line = lines.next().unwrap();
    lines.next();
    let mut map = [(0, 0); SIZE];
    let mut starts = vec![];
    for line in lines {
        let id = hash(&line[..3]);
        let left = hash(&line[7..10]);
        let right = hash(&line[12..15]);
        map[id as usize] = (left, right);
        if line.as_bytes()[2] == b'A' {
            starts.push(id);
        }
    }
    (turns_line, map, starts)
}

fn hash(s: &str) -> u16 {
    let mut hash = 0;
    for b in s.bytes().take(3) {
        hash <<= 5;
        hash |= encode(b) as u16;
    }
    hash
}

fn encode(b: u8) -> u8 {
    b - b'A'
}

pub fn part_one(turns: &str, map: &Map) -> u64 {
    steps_to_target(turns, map, hash("AAA"), |h| h == hash("ZZZ"))
}

pub fn part_two(turns: &str, map: &Map, starts: &[u16]) -> u64 {
    let is_target = |h: u16| h & 0b11111 == encode(b'Z') as u16;
    starts
        .iter()
        .map(|&start| steps_to_target(turns, map, start, is_target))
        .reduce(|a, b| {
            let mut gcd = a;
            let mut remainder = b;
            while remainder != 0 {
                (gcd, remainder) = (remainder, gcd % remainder);
            }
            (a * b) / gcd
        })
        .unwrap()
}

fn steps_to_target<F>(turns: &str, map: &Map, start: u16, is_target: F) -> u64
where
    F: Fn(u16) -> bool,
{
    let mut turns = turns.bytes().cycle();
    let mut current = start;
    (1..)
        .find(|_| {
            let (left, right) = map[current as usize];
            current = match turns.next().unwrap() {
                b'L' => left,
                b'R' => right,
                b => panic!("invalid turn {b}"),
            };
            is_target(current)
        })
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let (turns, map, _) = parse_input(INPUT);
        assert_eq!(part_one(turns, &map), 20_093);
    }

    #[test]
    fn test_part_two() {
        let (turns, map, starts) = parse_input(INPUT);
        assert_eq!(part_two(turns, &map, &starts), 22_103_062_509_257);
    }
}
