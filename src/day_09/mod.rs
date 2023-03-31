use crate::HashSet;

pub const INPUT: &str = include_str!("input.txt");

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
    fn is_adjacent_to(&self, other: &Self) -> bool {
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
                _ => panic!("invalid direction {}", d),
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
    let mut tail_positions = HashSet::default();
    for &(direction, steps) in motions {
        for _ in 0..steps {
            let head_position = &mut current_positions[0];
            match direction {
                Direction::Left => head_position.x -= 1,
                Direction::Right => head_position.x += 1,
                Direction::Up => head_position.y += 1,
                Direction::Down => head_position.y -= 1,
            }
            let mut leader = *head_position;
            for follower in &mut current_positions[1..] {
                if !leader.is_adjacent_to(follower) {
                    if leader.x > follower.x {
                        follower.x += 1;
                    } else if leader.x < follower.x {
                        follower.x -= 1;
                    }
                    if leader.y > follower.y {
                        follower.y += 1;
                    } else if leader.y < follower.y {
                        follower.y -= 1;
                    }
                }
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
