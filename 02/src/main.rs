const INPUT: &str = include_str!("../input.txt");

enum Outcome {
    Lose,
    Draw,
    Win,
}

impl Outcome {
    fn from_char(c: char) -> Self {
        match c {
            'X' => Self::Lose,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            _ => panic!("Invalid char"),
        }
    }
}

enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn from_char(c: char) -> Self {
        match c {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            _ => panic!("Invalid char"),
        }
    }
}

fn main() {
    let part_one_score: u32 = INPUT
        .lines()
        .map(|line| {
            let opponent_move = Move::from_char(line.chars().nth(0).unwrap());
            let own_move = Move::from_char(line.chars().nth(2).unwrap());
            match (own_move, opponent_move) {
                (Move::Rock, Move::Paper) => 1,
                (Move::Paper, Move::Scissors) => 2,
                (Move::Scissors, Move::Rock) => 3,
                (Move::Rock, Move::Rock) => 4,
                (Move::Paper, Move::Paper) => 5,
                (Move::Scissors, Move::Scissors) => 6,
                (Move::Rock, Move::Scissors) => 7,
                (Move::Paper, Move::Rock) => 8,
                (Move::Scissors, Move::Paper) => 9,
            }
        })
        .sum();
    println!("{}", part_one_score);
    let part_two_score: u32 = INPUT
        .lines()
        .map(|line| {
            let opponent_move = Move::from_char(line.chars().nth(0).unwrap());
            let outcome = Outcome::from_char(line.chars().nth(2).unwrap());
            match (outcome, opponent_move) {
                (Outcome::Lose, Move::Paper) => 1,
                (Outcome::Lose, Move::Scissors) => 2,
                (Outcome::Lose, Move::Rock) => 3,
                (Outcome::Draw, Move::Rock) => 4,
                (Outcome::Draw, Move::Paper) => 5,
                (Outcome::Draw, Move::Scissors) => 6,
                (Outcome::Win, Move::Scissors) => 7,
                (Outcome::Win, Move::Rock) => 8,
                (Outcome::Win, Move::Paper) => 9,
            }
        })
        .sum();
    println!("{}", part_two_score);
}
