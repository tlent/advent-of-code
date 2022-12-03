const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn from_char(c: char) -> Self {
        use Shape::*;
        match c {
            'A' | 'X' => Rock,
            'B' | 'Y' => Paper,
            'C' | 'Z' => Scissors,
            _ => panic!("invalid char"),
        }
    }

    fn score(&self) -> u32 {
        use Shape::*;
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    fn win_shape(&self) -> Self {
        use Shape::*;
        match self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }

    fn draw_shape(&self) -> Self {
        *self
    }

    fn loss_shape(&self) -> Self {
        use Shape::*;
        match self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }
}

enum Outcome {
    Win,
    Loss,
    Draw,
}

impl Outcome {
    fn from_char(c: char) -> Self {
        use Outcome::*;
        match c {
            'X' => Loss,
            'Y' => Draw,
            'Z' => Win,
            _ => panic!("invalid char"),
        }
    }

    fn from_shapes(opponent_shape: Shape, player_shape: Shape) -> Self {
        use Outcome::*;
        if player_shape == opponent_shape.win_shape() {
            Win
        } else if player_shape == opponent_shape.loss_shape() {
            Loss
        } else {
            Draw
        }
    }

    fn score(&self) -> u32 {
        use Outcome::*;
        match self {
            Win => 6,
            Loss => 0,
            Draw => 3,
        }
    }
}

struct Round {
    player_shape: Shape,
    opponent_shape: Shape,
    outcome: Outcome,
}

impl Round {
    fn from_shapes(opponent_shape: Shape, player_shape: Shape) -> Self {
        let outcome = Outcome::from_shapes(opponent_shape, player_shape);
        Round {
            opponent_shape,
            player_shape,
            outcome,
        }
    }

    fn from_outcome(opponent_shape: Shape, outcome: Outcome) -> Self {
        use Outcome::*;
        let player_shape = match outcome {
            Win => opponent_shape.win_shape(),
            Loss => opponent_shape.loss_shape(),
            Draw => opponent_shape,
        };
        Round {
            opponent_shape,
            player_shape,
            outcome,
        }
    }

    fn score(&self) -> u32 {
        let shape_score = self.player_shape.score();
        let outcome_score = self.outcome.score();
        outcome_score + shape_score
    }
}

fn main() {
    let (part_one_rounds, part_two_rounds) = parse_input(&INPUT);
    let part_one_score: u32 = part_one_rounds.iter().map(|round| round.score()).sum();
    println!("{}", part_one_score);
    let part_two_score: u32 = part_two_rounds.iter().map(|round| round.score()).sum();
    println!("{}", part_two_score);
}

fn parse_input(input: &str) -> (Vec<Round>, Vec<Round>) {
    input
        .lines()
        .map(|line| {
            let mut chars = line.chars().filter(|c| c.is_alphabetic());
            let opponent_shape = Shape::from_char(chars.next().unwrap());
            let second_char = chars.next().unwrap();
            let player_shape = Shape::from_char(second_char);
            let outcome = Outcome::from_char(second_char);
            (
                Round::from_shapes(opponent_shape, player_shape),
                Round::from_outcome(opponent_shape, outcome),
            )
        })
        .unzip()
}
