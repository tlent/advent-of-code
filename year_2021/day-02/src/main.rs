use std::str::FromStr;

const INPUT: &str = include_str!("../input");

fn main() {
    let commands = parse_input(INPUT);
    println!("{}", part_one(&commands));
    println!("{}", part_two(&commands));
}

enum Command {
    Forward(u32),
    Up(u32),
    Down(u32),
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');
        let err = Err("failed to parse command".to_string());
        let word = match parts.next() {
            Some(s) => s,
            None => return err,
        };
        let number = match parts.next().and_then(|s| s.parse().ok()) {
            Some(x) => x,
            None => return err,
        };
        let command = match word {
            "forward" => Command::Forward(number),
            "up" => Command::Up(number),
            "down" => Command::Down(number),
            _ => return err,
        };
        Ok(command)
    }
}

fn parse_input(input: &str) -> Vec<Command> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn part_one(commands: &[Command]) -> u32 {
    let mut horizontal_position = 0;
    let mut depth = 0;
    for command in commands {
        match command {
            Command::Forward(x) => horizontal_position += x,
            Command::Up(x) => depth -= x,
            Command::Down(x) => depth += x,
        }
    }
    horizontal_position * depth
}

fn part_two(commands: &[Command]) -> u32 {
    let mut horizontal_position = 0;
    let mut depth = 0;
    let mut aim = 0;
    for command in commands {
        match command {
            Command::Forward(x) => {
                horizontal_position += x;
                depth += aim * x
            }
            Command::Up(x) => aim -= x,
            Command::Down(x) => aim += x,
        }
    }
    horizontal_position * depth
}
