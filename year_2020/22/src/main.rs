use std::collections::HashSet;
use std::collections::VecDeque;

static INPUT: &str = include_str!("../input");
const VERBOSE_PART_TWO: bool = false;

fn main() {
    let (a, b) = parse_input(INPUT);
    println!("part one: {}", combat(&a, &b));
    println!("part two: {}", recursive_combat(&a, &b));
}

fn parse_input(s: &str) -> (Vec<u32>, Vec<u32>) {
    let mut parts = s.split("\n\n");
    let a = parts
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|l| l.parse().unwrap())
        .collect();
    let b = parts
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|l| l.parse().unwrap())
        .collect();
    (a, b)
}

fn combat(a: &[u32], b: &[u32]) -> u32 {
    let mut a: VecDeque<_> = a.iter().cloned().collect();
    let mut b: VecDeque<_> = b.iter().cloned().collect();
    while !a.is_empty() && !b.is_empty() {
        let a_value = a.pop_front().unwrap();
        let b_value = b.pop_front().unwrap();
        if a_value > b_value {
            a.push_back(a_value);
            a.push_back(b_value);
        } else {
            b.push_back(b_value);
            b.push_back(a_value);
        };
    }
    let mut winning_deck = if a.is_empty() { b } else { a };
    deck_score(winning_deck.make_contiguous())
}

fn deck_score(deck: &[u32]) -> u32 {
    let len = deck.len();
    let mut score = 0;
    for (i, v) in deck.iter().enumerate() {
        score += (len - i) as u32 * v;
    }
    score
}

#[derive(Debug, PartialEq, Eq)]
enum Player {
    A,
    B,
}

fn recursive_combat(a: &[u32], b: &[u32]) -> u32 {
    let (winner, winning_deck) = game(a, b, 1);
    if VERBOSE_PART_TWO {
        println!("== Post-game results ==");
        println!(
            "Player 1's deck: {}",
            if winner == Player::A {
                winning_deck
                    .iter()
                    .map(u32::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            } else {
                String::default()
            }
        );
        println!(
            "Player 2's deck: {}",
            if winner == Player::B {
                winning_deck
                    .iter()
                    .map(u32::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            } else {
                String::default()
            }
        );
    }
    deck_score(&winning_deck)
}

fn game(a: &[u32], b: &[u32], game_number: u32) -> (Player, Vec<u32>) {
    if VERBOSE_PART_TWO {
        println!("=== Game {} ===\n", game_number);
    }
    let mut a: VecDeque<_> = a.iter().cloned().collect();
    let mut b: VecDeque<_> = b.iter().cloned().collect();
    let mut game_winner = None;
    let mut seen = HashSet::new();
    let mut round_number = 1;
    while !a.is_empty() && !b.is_empty() {
        if VERBOSE_PART_TWO {
            println!("-- Round {} (Game {}) --\n", round_number, game_number);
            println!(
                "Player 1's deck: {}",
                a.iter().map(u32::to_string).collect::<Vec<_>>().join(", ")
            );
            println!(
                "Player 2's deck: {}",
                b.iter().map(u32::to_string).collect::<Vec<_>>().join(", ")
            );
        }
        // optimization from https://www.reddit.com/r/adventofcode/comments/khyjgv/2020_day_22_solutions/ggpcsnd
        let a_max = *a.iter().max().unwrap();
        let b_max = *b.iter().max().unwrap();
        if a_max > b_max && a_max as usize > (a.len() + b.len()) {
            game_winner = Some(Player::A);
            break;
        }
        let state = (a.clone(), b.clone());
        if seen.contains(&state) {
            game_winner = Some(Player::A);
            break;
        }
        seen.insert(state);
        let a_value = a.pop_front().unwrap();
        let b_value = b.pop_front().unwrap();
        if VERBOSE_PART_TWO {
            println!("Player 1 plays: {}", a_value);
            println!("Player 2 plays: {}", b_value);
        }
        let round_winner = if a_value as usize <= a.len() && b_value as usize <= b.len() {
            if VERBOSE_PART_TWO {
                println!("Playing a sub-game to determine the winner...\n");
            }
            let (winner, _) = game(
                &a.make_contiguous()[..a_value as usize],
                &b.make_contiguous()[..b_value as usize],
                game_number + 1,
            );
            if VERBOSE_PART_TWO {
                println!("...anyway, back to game {}.", game_number);
            }
            winner
        } else if a_value > b_value {
            Player::A
        } else {
            Player::B
        };
        let player_string;
        match round_winner {
            Player::A => {
                player_string = "1";
                a.push_back(a_value);
                a.push_back(b_value);
            }
            Player::B => {
                player_string = "2";
                b.push_back(b_value);
                b.push_back(a_value);
            }
        }
        if VERBOSE_PART_TWO {
            println!(
                "Player {} wins round {} of game {}!\n",
                player_string, round_number, game_number
            );
        }
        round_number += 1;
    }
    let game_winner =
        game_winner.unwrap_or_else(|| if b.is_empty() { Player::A } else { Player::B });
    if VERBOSE_PART_TWO {
        println!(
            "The winner of game {} is player {}!\n",
            game_number,
            match game_winner {
                Player::A => "1",
                Player::B => "2",
            }
        );
    }
    let winning_deck = match game_winner {
        Player::A => a.into_iter().collect(),
        Player::B => b.into_iter().collect(),
    };
    (game_winner, winning_deck)
}
