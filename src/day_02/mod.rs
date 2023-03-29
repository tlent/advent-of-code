pub const INPUT: &str = include_str!("./input.txt");

pub fn parse_input(input: &str) -> (Vec<(Move, Move)>, Vec<(Move, Outcome)>) {
    input
        .lines()
        .map(|line| {
            let opponent_move = Move::from_byte(line.bytes().nth(0).unwrap());
            let second_byte = line.bytes().nth(2).unwrap();
            let own_move = Move::from_byte(second_byte);
            let outcome = Outcome::from_byte(second_byte);
            ((opponent_move, own_move), (opponent_move, outcome))
        })
        .unzip()
}

pub fn part_one(rounds: &[(Move, Move)]) -> u32 {
    rounds
        .iter()
        .map(
            |(opponent_move, own_move)| match (own_move, opponent_move) {
                (Move::Rock, Move::Paper) => 1,
                (Move::Paper, Move::Scissors) => 2,
                (Move::Scissors, Move::Rock) => 3,
                (Move::Rock, Move::Rock) => 4,
                (Move::Paper, Move::Paper) => 5,
                (Move::Scissors, Move::Scissors) => 6,
                (Move::Rock, Move::Scissors) => 7,
                (Move::Paper, Move::Rock) => 8,
                (Move::Scissors, Move::Paper) => 9,
            },
        )
        .sum()
}

pub fn part_two(rounds: &[(Move, Outcome)]) -> u32 {
    rounds
        .iter()
        .map(|(opponent_move, outcome)| match (outcome, opponent_move) {
            (Outcome::Lose, Move::Paper) => 1,
            (Outcome::Lose, Move::Scissors) => 2,
            (Outcome::Lose, Move::Rock) => 3,
            (Outcome::Draw, Move::Rock) => 4,
            (Outcome::Draw, Move::Paper) => 5,
            (Outcome::Draw, Move::Scissors) => 6,
            (Outcome::Win, Move::Scissors) => 7,
            (Outcome::Win, Move::Rock) => 8,
            (Outcome::Win, Move::Paper) => 9,
        })
        .sum()
}

pub enum Outcome {
    Lose,
    Draw,
    Win,
}

impl Outcome {
    fn from_byte(byte: u8) -> Self {
        match byte {
            b'X' => Self::Lose,
            b'Y' => Self::Draw,
            b'Z' => Self::Win,
            _ => panic!("Invalid byte"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn from_byte(byte: u8) -> Self {
        match byte {
            b'A' | b'X' => Self::Rock,
            b'B' | b'Y' => Self::Paper,
            b'C' | b'Z' => Self::Scissors,
            _ => panic!("Invalid byte"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let (rounds, _) = parse_input(INPUT);
        assert_eq!(part_one(&rounds), 14531);
    }

    #[test]
    fn test_part_two() {
        let (_, rounds) = parse_input(INPUT);
        assert_eq!(part_two(&rounds), 11258);
    }
}
