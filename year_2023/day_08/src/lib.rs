use rustc_hash::FxHashMap as HashMap;

pub const INPUT: &str = include_str!("../input.txt");

pub struct Input<'a> {
    turns: Vec<Turn>,
    nodes: Vec<Node<'a>>,
}

pub enum Turn {
    Left,
    Right,
}

pub struct Node<'a> {
    id: &'a str,
    left: usize,
    right: usize,
}

impl<'a> Node<'a> {
    pub fn new(id: &'a str) -> Self {
        Self {
            id,
            left: 0,
            right: 0,
        }
    }
}

pub fn parse_input(input: &str) -> Input {
    let mut lines = input.lines();
    let turns = lines
        .next()
        .unwrap()
        .bytes()
        .map(|b| match b {
            b'L' => Turn::Left,
            b'R' => Turn::Right,
            _ => panic!("invalid turn {b}"),
        })
        .collect();
    lines.next();
    let lines: Vec<_> = lines.collect();
    let mut nodes: Vec<_> = lines.iter().map(|line| Node::new(&line[..3])).collect();
    let nodes_map: HashMap<&str, usize> = nodes
        .iter()
        .enumerate()
        .map(|(index, node)| (node.id, index))
        .collect();
    for (line, node) in lines.into_iter().zip(nodes.iter_mut()) {
        node.left = nodes_map[&line[7..10]];
        node.right = nodes_map[&line[12..15]];
    }
    Input { turns, nodes }
}

pub fn part_one(input: &Input) -> u32 {
    let start = input
        .nodes
        .iter()
        .position(|node| node.id == "AAA")
        .unwrap();
    steps_to_target(input, start, |id| id == "ZZZ")
}

pub fn part_two(input: &Input) -> u64 {
    let is_target = |id: &str| id.ends_with('Z');
    input
        .nodes
        .iter()
        .enumerate()
        .filter(|(_index, node)| node.id.ends_with('A'))
        .map(|(index, _node)| steps_to_target(input, index, is_target) as u64)
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

fn steps_to_target<F>(input: &Input, start: usize, is_target: F) -> u32
where
    F: Fn(&str) -> bool,
{
    let mut node = &input.nodes[start];
    let mut turns = input.turns.iter().cycle();
    (1..)
        .find(|_| {
            node = &input.nodes[match turns.next().unwrap() {
                Turn::Left => node.left,
                Turn::Right => node.right,
            }];
            is_target(node.id)
        })
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = parse_input(INPUT);
        assert_eq!(part_one(&input), 20_093);
    }

    #[test]
    fn test_part_two() {
        let input = parse_input(INPUT);
        assert_eq!(part_two(&input), 22_103_062_509_257);
    }
}
