static INPUT: &str = include_str!("../input");

fn main() {
    let motions: Vec<_> = INPUT.lines().map(|l| Motion::parse(l)).collect();
    println!("part one: {}", part_one(&motions));
    println!("part two: {}", part_two(&motions));
}

#[derive(Clone, Copy)]
enum Motion {
    North(u32),
    East(u32),
    West(u32),
    South(u32),
    Forward(u32),
    Right(u32),
    Left(u32),
}

impl Motion {
    fn parse(s: &str) -> Self {
        let value = s[1..].parse().unwrap();
        match s.chars().next().unwrap() {
            'N' => Self::North(value),
            'E' => Self::East(value),
            'W' => Self::West(value),
            'S' => Self::South(value),
            'F' => Self::Forward(value),
            'R' => Self::Right(value),
            'L' => Self::Left(value),
            _ => panic!("invalid motion"),
        }
    }
}

fn part_one(motions: &[Motion]) -> u32 {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut facing: i32 = 0;
    use Motion::*;
    for &motion in motions {
        let mut m = motion;
        if let Forward(v) = motion {
            m = match facing {
                0 => East(v),
                90 => North(v),
                180 => West(v),
                270 => South(v),
                _ => panic!("invalid facing: {}", facing),
            }
        }
        match m {
            North(v) => y += v as i32,
            East(v) => x += v as i32,
            West(v) => x -= v as i32,
            South(v) => y -= v as i32,
            Right(v) => facing = facing - v as i32,
            Left(v) => facing = facing + v as i32,
            Forward(_) => unreachable!(),
        }
        if facing < 0 {
            facing += 360;
        } else if facing >= 360 {
            facing -= 360;
        }
    }
    (x.abs() + y.abs()) as u32
}

fn part_two(motions: &[Motion]) -> u32 {
    let (mut waypoint_x, mut waypoint_y) = (10i32, 1i32);
    let (mut x, mut y) = (0i32, 0i32);
    use Motion::*;
    for &motion in motions {
        match motion {
            North(v) => waypoint_y += v as i32,
            East(v) => waypoint_x += v as i32,
            West(v) => waypoint_x -= v as i32,
            South(v) => waypoint_y -= v as i32,
            Forward(v) => {
                x += waypoint_x * v as i32;
                y += waypoint_y * v as i32;
            }
            Left(v) => match v {
                90 => {
                    let t = waypoint_x;
                    waypoint_x = -waypoint_y;
                    waypoint_y = t;
                }
                180 => {
                    waypoint_x = -waypoint_x;
                    waypoint_y = -waypoint_y;
                }
                270 => {
                    let t = waypoint_x;
                    waypoint_x = waypoint_y;
                    waypoint_y = -t;
                }
                v => panic!("invalid turn degrees: {}", v),
            },
            Right(v) => match v {
                90 => {
                    let t = waypoint_x;
                    waypoint_x = waypoint_y;
                    waypoint_y = -t;
                }
                180 => {
                    waypoint_x = -waypoint_x;
                    waypoint_y = -waypoint_y;
                }
                270 => {
                    let t = waypoint_x;
                    waypoint_x = -waypoint_y;
                    waypoint_y = t;
                }
                v => panic!("invalid turn degrees: {}", v),
            },
        }
    }
    (x.abs() + y.abs()) as u32
}
