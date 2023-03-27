const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Win,
    Draw,
    Lose,
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

    fn from_moves(own_move: Move, opponent_move: Move) -> Self {
        use Move::*;
        match (own_move, opponent_move) {
            (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => Outcome::Lose,
            (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => Outcome::Draw,
            (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => Outcome::Win,
        }
    }

    fn score(&self) -> u32 {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Lose => 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
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

    fn score(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn for_outcome(outcome: Outcome, opponent_move: Self) -> Self {
        use Move::*;
        match outcome {
            Outcome::Lose => match opponent_move {
                Rock => Scissors,
                Paper => Rock,
                Scissors => Paper,
            },
            Outcome::Draw => opponent_move,
            Outcome::Win => match opponent_move {
                Rock => Paper,
                Paper => Scissors,
                Scissors => Rock,
            },
        }
    }
}

fn main() {
    let part_one_score: u32 = INPUT
        .lines()
        .map(|line| {
            let opponent_move = Move::from_char(line.chars().nth(0).unwrap());
            let own_move = Move::from_char(line.chars().nth(2).unwrap());
            let outcome = Outcome::from_moves(own_move, opponent_move);
            own_move.score() + outcome.score()
        })
        .sum();
    println!("{}", part_one_score);
    let part_two_score: u32 = INPUT
        .lines()
        .map(|line| {
            let opponent_move = Move::from_char(line.chars().nth(0).unwrap());
            let outcome = Outcome::from_char(line.chars().nth(2).unwrap());
            let own_move = Move::for_outcome(outcome, opponent_move);
            own_move.score() + outcome.score()
        })
        .sum();
    println!("{}", part_two_score);
}
