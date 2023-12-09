#![feature(test)]
extern crate test;

use std::env;

pub const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn from_cards(cards: &[Card; 5]) -> Self {
        let mut card_counts = [0; 14];
        let mut max_count = 0;
        let mut second_max_count = 0;
        for &card in cards {
            let count = &mut card_counts[card as usize];
            *count += 1;
            if card == Card::Joker {
                continue;
            }
            if *count > max_count {
                max_count = *count;
            } else if *count > second_max_count {
                second_max_count = *count;
            }
        }
        max_count += card_counts[Card::Joker as usize];
        match (max_count, second_max_count) {
            (5, _) => HandType::FiveOfAKind,
            (4, _) => HandType::FourOfAKind,
            (3, 2) => HandType::FullHouse,
            (3, _) => HandType::ThreeOfAKind,
            (2, 2) => HandType::TwoPair,
            (2, _) => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Hand {
    cards: [Card; 5],
    bid: u32,
    hand_type: HandType,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.hand_type, &self.cards).cmp(&(other.hand_type, &other.cards))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn parse_input(input: &str) -> Vec<Hand> {
    input
        .lines()
        .map(|line| {
            let mut bytes = line[..5].bytes();
            let mut cards = [Card::Joker; 5];
            cards.fill_with(|| match bytes.next().unwrap() {
                b'A' => Card::Ace,
                b'K' => Card::King,
                b'Q' => Card::Queen,
                b'J' => Card::Jack,
                b'T' => Card::Ten,
                b'9' => Card::Nine,
                b'8' => Card::Eight,
                b'7' => Card::Seven,
                b'6' => Card::Six,
                b'5' => Card::Five,
                b'4' => Card::Four,
                b'3' => Card::Three,
                b'2' => Card::Two,
                b => panic!("invalid byte {b}"),
            });
            let bid = line[6..].parse().unwrap();
            let hand_type = HandType::from_cards(&cards);
            Hand {
                cards,
                bid,
                hand_type,
            }
        })
        .collect()
}

pub fn part_one(hands: &mut [Hand]) -> u32 {
    hands.sort_unstable();
    hands
        .iter()
        .zip(1..)
        .map(|(hand, rank)| hand.bid * rank)
        .sum()
}

pub fn part_two(hands: &mut [Hand]) -> u32 {
    for hand in hands.iter_mut() {
        let mut changed_cards = false;
        for card in hand.cards.iter_mut() {
            if *card == Card::Jack {
                *card = Card::Joker;
                changed_cards = true;
            }
        }
        if changed_cards {
            hand.hand_type = HandType::from_cards(&hand.cards);
        }
    }
    hands.sort_unstable();
    hands
        .iter()
        .zip(1..)
        .map(|(hand, rank)| hand.bid * rank)
        .sum()
}

fn main() {
    let mut parse_result = parse_input(INPUT);
    match env::args().nth(1).as_deref() {
        Some("all") => {
            let part_one = part_one(&mut parse_result);
            println!("{part_one}");
            let part_two = part_two(&mut parse_result);
            println!("{part_two}");
        }
        Some("parse") => {}
        Some("one") => {
            let part_one = part_one(&mut parse_result);
            println!("{part_one}");
        }
        Some("two") => {
            let part_two = part_two(&mut parse_result);
            println!("{part_two}");
        }
        _ => println!("Invalid argument: must be one of all, parse, one, or two"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::hint::black_box;
    use test::Bencher;

    #[test]
    fn test_part_one() {
        let mut hands = parse_input(INPUT);
        assert_eq!(part_one(&mut hands), 253_866_470);
    }

    #[test]
    fn test_part_two() {
        let mut hands = parse_input(INPUT);
        assert_eq!(part_two(&mut hands), 254_494_947);
    }

    #[bench]
    fn bench_parse_input(b: &mut Bencher) {
        b.iter(|| parse_input(black_box(INPUT)));
    }

    #[bench]
    fn bench_part_one(b: &mut Bencher) {
        let mut hands = parse_input(INPUT);
        b.iter(|| part_one(black_box(&mut hands)));
    }

    #[bench]
    fn bench_part_two(b: &mut Bencher) {
        let mut hands = parse_input(INPUT);
        b.iter(|| part_two(black_box(&mut hands)));
    }
}
