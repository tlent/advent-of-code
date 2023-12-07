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

#[derive(Debug, PartialEq, Eq)]
pub struct Hand {
    cards: Vec<Card>,
    bid: u32,
}

impl Hand {
    pub fn hand_type(&self) -> HandType {
        let mut joker_count = 0;
        let mut card_counts = [0; 13];
        for &card in &self.cards {
            if card == Card::Joker {
                joker_count += 1;
                continue;
            }
            let index = match card {
                Card::Ace => 0,
                Card::King => 1,
                Card::Queen => 2,
                Card::Jack => 3,
                Card::Ten => 4,
                Card::Nine => 5,
                Card::Eight => 6,
                Card::Seven => 7,
                Card::Six => 8,
                Card::Five => 9,
                Card::Four => 10,
                Card::Three => 11,
                Card::Two => 12,
                Card::Joker => unreachable!(),
            };
            card_counts[index] += 1;
        }
        let max_count = card_counts.iter_mut().max().unwrap();
        *max_count += joker_count;
        let max_count = *max_count;
        if max_count == 5 {
            return HandType::FiveOfAKind;
        }
        if max_count == 4 {
            return HandType::FourOfAKind;
        }
        let pair_count = card_counts.iter().filter(|&&count| count == 2).count();
        if max_count == 3 && pair_count == 1 {
            return HandType::FullHouse;
        }
        if max_count == 3 {
            return HandType::ThreeOfAKind;
        }
        if pair_count == 2 {
            return HandType::TwoPair;
        }
        if pair_count == 1 {
            return HandType::OnePair;
        }
        HandType::HighCard
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.hand_type(), &self.cards).cmp(&(other.hand_type(), &other.cards))
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
            let (left, right) = line.split_once(' ').unwrap();
            let cards = left
                .bytes()
                .map(|b| match b {
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
                    _ => panic!("invalid card"),
                })
                .collect();
            let bid = right.parse().unwrap();
            Hand { cards, bid }
        })
        .collect()
}

pub fn part_one(hands: &mut [Hand]) -> u32 {
    hands.sort_unstable();
    hands
        .iter()
        .zip(1..=hands.len())
        .map(|(hand, rank)| hand.bid * rank as u32)
        .sum()
}

pub fn part_two(hands: &mut [Hand]) -> u32 {
    for hand in hands.iter_mut() {
        for card in hand.cards.iter_mut() {
            if *card == Card::Jack {
                *card = Card::Joker;
            }
        }
    }
    hands.sort_unstable();
    hands
        .iter()
        .zip(1..=hands.len())
        .map(|(hand, rank)| hand.bid * rank as u32)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

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
}
