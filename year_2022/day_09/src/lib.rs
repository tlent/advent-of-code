use rustc_hash::FxHashSet;

pub const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

type Motion = (Direction, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    const fn is_adjacent_to(self, other: Self) -> bool {
        self.x.abs_diff(other.x) <= 1 && self.y.abs_diff(other.y) <= 1
    }
}

pub fn parse_input(input: &str) -> Vec<Motion> {
    input
        .lines()
        .map(|line| {
            let (d, s) = line.split_once(' ').unwrap();
            let direction = match d {
                "L" => Direction::Left,
                "R" => Direction::Right,
                "U" => Direction::Up,
                "D" => Direction::Down,
                _ => panic!("invalid direction {d}"),
            };
            let steps = s.parse().unwrap();
            (direction, steps)
        })
        .collect()
}

pub fn part_one(motions: &[Motion]) -> usize {
    count_tail_positions(motions, 2)
}

pub fn part_two(motions: &[Motion]) -> usize {
    count_tail_positions(motions, 10)
}

fn count_tail_positions(motions: &[Motion], knot_count: usize) -> usize {
    let mut current_positions = vec![Position::default(); knot_count];
    let mut tail_positions = FxHashSet::default();
    for &(direction, step_count) in motions {
        let (dx, dy) = match direction {
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
        };
        for _ in 0..step_count {
            let head_position = &mut current_positions[0];
            head_position.x += dx;
            head_position.y += dy;
            let mut leader = *head_position;
            for follower in &mut current_positions[1..] {
                if leader.is_adjacent_to(*follower) {
                    break;
                }
                follower.x += (leader.x - follower.x).signum();
                follower.y += (leader.y - follower.y).signum();
                leader = *follower;
            }
            tail_positions.insert(current_positions[knot_count - 1]);
        }
    }
    tail_positions.len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let motions = parse_input(INPUT);
        assert_eq!(part_one(&motions), 6087);
    }

    #[test]
    fn test_part_two() {
        let motions = parse_input(INPUT);
        assert_eq!(part_two(&motions), 2493);
    }
}
