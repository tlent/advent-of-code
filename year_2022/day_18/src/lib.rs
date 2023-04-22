use rustc_hash::FxHashSet as HashSet;

pub const INPUT: &str = include_str!("../input.txt");

type Position = (i32, i32, i32);

pub fn parse_input(input: &str) -> HashSet<Position> {
    let values = input
        .trim()
        .split(['\n', ','])
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>();
    values.chunks_exact(3).map(|c| (c[0], c[1], c[2])).collect()
}

pub fn part_one(positions: &HashSet<Position>) -> usize {
    positions
        .iter()
        .flat_map(|&(x, y, z)| {
            [
                (x + 1, y, z),
                (x - 1, y, z),
                (x, y + 1, z),
                (x, y - 1, z),
                (x, y, z + 1),
                (x, y, z - 1),
            ]
        })
        .filter(|p| !positions.contains(p))
        .count()
}

pub fn part_two(positions: &HashSet<Position>) -> usize {
    let min_x = positions.iter().map(|&(x, _, _)| x).min().unwrap();
    let max_x = positions.iter().map(|&(x, _, _)| x).max().unwrap();
    let x_bounds = min_x - 1..=max_x + 1;
    let min_y = positions.iter().map(|&(_, y, _)| y).min().unwrap();
    let max_y = positions.iter().map(|&(_, y, _)| y).max().unwrap();
    let y_bounds = min_y - 1..=max_y + 1;
    let min_z = positions.iter().map(|&(_, _, z)| z).min().unwrap();
    let max_z = positions.iter().map(|&(_, _, z)| z).max().unwrap();
    let z_bounds = min_z - 1..=max_z + 1;
    let start_position = (max_x + 1, max_y, max_z);
    let mut surface_area = 0;
    let mut stack = vec![start_position];
    let mut seen = HashSet::default();
    while let Some((x, y, z)) = stack.pop() {
        let adjacent_positions = [
            (x + 1, y, z),
            (x - 1, y, z),
            (x, y + 1, z),
            (x, y - 1, z),
            (x, y, z + 1),
            (x, y, z - 1),
        ];
        let iter = adjacent_positions.into_iter().filter(|(x, y, z)| {
            x_bounds.contains(x) && y_bounds.contains(y) && z_bounds.contains(z)
        });
        for p in iter {
            if positions.contains(&p) {
                surface_area += 1;
            } else if !seen.contains(&p) {
                stack.push(p);
                seen.insert(p);
            }
        }
    }
    surface_area
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let positions = parse_input(INPUT);
        assert_eq!(part_one(&positions), 3586);
    }

    #[test]
    fn test_part_two() {
        let positions = parse_input(INPUT);
        assert_eq!(part_two(&positions), 2072);
    }
}
