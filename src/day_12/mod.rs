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

pub fn part_one(grid: &HeightMap) -> usize {
    find_shortest_path_len(grid, [grid.initial_position]).unwrap()
}

pub fn part_two(grid: &HeightMap) -> usize {
    let starting_positions =
        grid.heights
            .iter()
            .enumerate()
            .filter_map(|(i, &h)| if h == 0 { Some(i) } else { None });
    find_shortest_path_len(grid, starting_positions).unwrap()
}

fn find_shortest_path_len<I>(grid: &HeightMap, starting_positions: I) -> Option<usize>
where
    I: IntoIterator<Item = usize>,
{
    let mut queue = starting_positions
        .into_iter()
        .map(|p| (p, 0))
        .collect::<VecDeque<_>>();
    let mut seen = vec![false; grid.width * grid.height];
    while let Some((position, steps)) = queue.pop_front() {
        if position == grid.target_position {
            return Some(steps);
        }
        let seen = &mut seen[position];
        if *seen {
            continue;
        }
        *seen = true;
        let x = position % grid.width;
        let y = position / grid.width;
        let adjacent_positions = [
            x.checked_sub(1).map(|x| x + y * grid.width),
            x.checked_add(1)
                .filter(|&x| x < grid.width)
                .map(|x| x + y * grid.width),
            y.checked_sub(1).map(|y| x + y * grid.width),
            y.checked_add(1)
                .filter(|&y| y < grid.height)
                .map(|y| x + y * grid.width),
        ]
        .into_iter()
        .filter_map(|p| {
            p.filter(|&p| grid.heights[p] <= grid.heights[position] + 1)
                .map(|p| (p, steps + 1))
        });
        queue.extend(adjacent_positions);
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let grid = parse_input(INPUT);
        assert_eq!(part_one(&grid), 472);
    }

    #[test]
    fn test_part_two() {
        let grid = parse_input(INPUT);
        assert_eq!(part_two(&grid), 465);
    }
}
