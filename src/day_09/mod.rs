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
    let mut current_positions = vec![(0i32, 0i32); knot_count];
    let mut tail_positions = HashSet::default();
    for &(direction, steps) in motions {
        for _ in 0..steps {
            let head_position = &mut current_positions[0];
            match direction {
                Direction::Left => head_position.0 -= 1,
                Direction::Right => head_position.0 += 1,
                Direction::Up => head_position.1 += 1,
                Direction::Down => head_position.1 -= 1,
            }
            let mut leader = *head_position;
            for follower in &mut current_positions[1..] {
                let adjacent =
                    leader.0.abs_diff(follower.0) <= 1 && leader.1.abs_diff(follower.1) <= 1;
                if !adjacent {
                    if leader.0 > follower.0 {
                        follower.0 += 1;
                    } else if leader.0 < follower.0 {
                        follower.0 -= 1;
                    }
                    if leader.1 > follower.1 {
                        follower.1 += 1;
                    } else if leader.1 < follower.1 {
                        follower.1 -= 1;
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
