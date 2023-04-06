use std::collections::VecDeque;

pub const INPUT: &str = include_str!("input.txt");

pub struct HeightMap {
    width: usize,
    height: usize,
    heights: Vec<u8>,
    initial_position: usize,
    target_position: usize,
}

impl HeightMap {
    fn from_input(input: &str) -> Self {
        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();
        let bytes = input.lines().flat_map(str::bytes).collect::<Vec<_>>();
        let initial_position = bytes.iter().position(|&b| b == b'S').unwrap();
        let target_position = bytes.iter().position(|&b| b == b'E').unwrap();
        let heights = bytes
            .into_iter()
            .map(|b| match b {
                b'S' => 0,
                b'E' => 25,
                b if b.is_ascii_lowercase() => b - b'a',
                _ => panic!("invalid height {b}"),
            })
            .collect::<Vec<_>>();
        HeightMap {
            width,
            height,
            heights,
            initial_position,
            target_position,
        }
    }
}

pub fn parse_input(input: &str) -> HeightMap {
    HeightMap::from_input(input)
}

pub fn part_one(height_map: &HeightMap) -> usize {
    find_shortest_path_len(height_map, [height_map.initial_position]).unwrap()
}

pub fn part_two(height_map: &HeightMap) -> usize {
    let starting_positions =
        height_map
            .heights
            .iter()
            .enumerate()
            .filter_map(|(i, &h)| if h == 0 { Some(i) } else { None });
    find_shortest_path_len(height_map, starting_positions).unwrap()
}

fn find_shortest_path_len<I>(height_map: &HeightMap, starting_positions: I) -> Option<usize>
where
    I: IntoIterator<Item = usize>,
{
    let mut queue = starting_positions
        .into_iter()
        .map(|p| (p, 0))
        .collect::<VecDeque<_>>();
    let mut seen = vec![false; height_map.width * height_map.height];
    while let Some((position, steps)) = queue.pop_front() {
        if position == height_map.target_position {
            return Some(steps);
        }
        if seen[position] {
            continue;
        }
        seen[position] = true;
        let x = position % height_map.width;
        let y = position / height_map.width;
        let adjacent_positions = [
            x.checked_sub(1).map(|x| x + y * height_map.width),
            x.checked_add(1)
                .filter(|&x| x < height_map.width)
                .map(|x| x + y * height_map.width),
            y.checked_sub(1).map(|y| x + y * height_map.width),
            y.checked_add(1)
                .filter(|&y| y < height_map.height)
                .map(|y| x + y * height_map.width),
        ]
        .into_iter()
        .filter_map(|p| {
            p.filter(|&p| !seen[p] && height_map.heights[p] <= height_map.heights[position] + 1)
                .map(|p| (p, steps + 1))
        });
        queue.extend(adjacent_positions);
        dbg!(queue.len());
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let height_map = parse_input(INPUT);
        assert_eq!(part_one(&height_map), 472);
    }

    #[test]
    fn test_part_two() {
        let height_map = parse_input(INPUT);
        assert_eq!(part_two(&height_map), 465);
    }
}
