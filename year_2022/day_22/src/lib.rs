pub const INPUT: &str = include_str!("../input.txt");

pub struct Map<'a>(&'a str);

pub enum PathStep {
    Forward(usize),
    Left,
    Right,
}

pub fn parse_input(input: &str) -> (Map, Vec<PathStep>) {
    let (map_str, mut path_str) = input.trim().split_once("\n\n").unwrap();
    let map = Map(map_str);
    let mut path = vec![];
    while !path_str.is_empty() {
        let end_index = path_str.find(['L', 'R']).unwrap_or(path_str.len());
        let forward = PathStep::Forward(path_str[..end_index].parse().unwrap());
        path.push(forward);
        path_str = &path_str[end_index..];
        if !path_str.is_empty() {
            let turn = match path_str.chars().next().unwrap() {
                'L' => PathStep::Left,
                'R' => PathStep::Right,
                _ => unreachable!(),
            };
            path.push(turn);
            path_str = &path_str[1..];
        }
    }
    (map, path)
}

pub fn part_one() -> () {
    todo!()
}

pub fn part_two() -> () {
    todo!()
}

type Position = (i32, i32);

enum Direction {
    Up,
    Left,
    Right,
    Down,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        todo!()
    }

    #[test]
    fn test_part_two() {
        todo!()
    }
}
