const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Clone, Copy)]
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

    fn from_moves(own_move: Move, opponent_move: Move) -> Self {
        if own_move == opponent_move {
            Self::Draw
        } else if own_move.wins_against() == opponent_move {
            Self::Win
        } else {
            Self::Lose
        }
    }

    fn score(&self) -> u32 {
        match self {
            Self::Lose => 0,
            Self::Draw => 3,
            Self::Win => 6,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    
    fn wins_against(&self) -> Self {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper
        }
    }
    
    fn loses_against(&self) -> Self {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock
        }
    }

    fn for_outcome(outcome: Outcome, opponent_move: Self) -> Self {
        match outcome {
            Outcome::Lose => opponent_move.wins_against(),
            Outcome::Draw => opponent_move,
            Outcome::Win => opponent_move.loses_against()
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
