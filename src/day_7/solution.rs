// https://adventofcode.com/2023/day/7

use std::{collections::{BinaryHeap, HashMap}, cmp::Ordering};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum CardLabel {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

impl CardLabel {
    fn from(c: char) -> Result<Self, ()> {
        match c {
            '2' => Ok(Self::Two),
            '3' => Ok(Self::Three),
            '4' => Ok(Self::Four),
            '5' => Ok(Self::Five),
            '6' => Ok(Self::Six),
            '7' => Ok(Self::Seven),
            '8' => Ok(Self::Eight),
            '9' => Ok(Self::Nine),
            'T' => Ok(Self::T),
            'J' => Ok(Self::J),
            'Q' => Ok(Self::Q),
            'K' => Ok(Self::K),
            'A' => Ok(Self::A),
            _ => Err(()),
        }
    }

    fn from_with_jokers(c: char) -> Result<Self, ()> {
        match c {
            'J' => Ok(Self::Joker),
            '2' => Ok(Self::Two),
            '3' => Ok(Self::Three),
            '4' => Ok(Self::Four),
            '5' => Ok(Self::Five),
            '6' => Ok(Self::Six),
            '7' => Ok(Self::Seven),
            '8' => Ok(Self::Eight),
            '9' => Ok(Self::Nine),
            'T' => Ok(Self::T),
            'Q' => Ok(Self::Q),
            'K' => Ok(Self::K),
            'A' => Ok(Self::A),
            _ => Err(()),
        }
    }

}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Hand {
    ty: HandType,
    cards: Vec<CardLabel>,
    bid: u64,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        other.ty.cmp(&self.ty).then_with(|| other.compare_cards(self))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other)) 
    }
}

impl Hand {
    fn new(hand: &str, bid: u64) -> Self {
        let cards = Self::parse_hand(hand);
        let ty = Self::determine_hand_type(&cards);

        Self {
            ty,
            cards,
            bid,
        }
    }

    fn new_with_jokers(hand: &str, bid: u64) -> Self {
        let cards = Self::parse_hand_with_jokers(hand);
        let ty = Self::determine_hand_type(&cards);

        Self {
            ty,
            cards,
            bid,
        }
    }

    fn parse_hand(hand: &str) -> Vec<CardLabel> {
        hand.chars().map(|c| CardLabel::from(c).unwrap()).collect()
    }

    fn parse_hand_with_jokers(hand: &str) -> Vec<CardLabel> {
        hand.chars().map(|c| CardLabel::from_with_jokers(c).unwrap()).collect()
    }

    fn determine_hand_type(cards: &Vec<CardLabel>) -> HandType {
        let mut counts = HashMap::<CardLabel, u8>::new();

        for &card in cards {
            counts.entry(card).and_modify(|n| { *n += 1 }).or_insert(1);
        }

        match counts.get(&CardLabel::Joker) {
            Some(&5) | None => {},
            Some(&num_jokers) => {
                counts.remove_entry(&CardLabel::Joker);

                let (&card_label, _) = counts.iter().max_by(|a, b| a.1.cmp(&b.1).then_with(|| a.0.cmp(&b.0))).unwrap();
                counts.entry(card_label).and_modify(|n| { *n += num_jokers });
            }
        }

        match counts.keys().len() {
            1 => HandType::FiveKind,
            2 => {
                match counts.values().next().unwrap() {
                    1 | 4 => HandType::FourKind,
                    2 | 3 => HandType::FullHouse,
                    _ => panic!("Failed to determine hand with two labels {:?}", cards),
                }
            },
            3 => {
                for &count in counts.values() {
                    if count == 3 {
                        return HandType::ThreeKind;
                    }
                }

                HandType::TwoPair
            },
            4 => HandType::OnePair,
            5 => HandType::HighCard,
            _ => panic!("Failed to determine hand {:?}", cards),
        }
    }

    fn compare_cards(&self, other: &Hand) -> Ordering {
        for (this, that) in self.cards.iter().zip(other.cards.iter()) {
            match this.cmp(that) {
                Ordering::Equal => continue,
                ordering => return ordering,
            }
        }

        Ordering::Equal
    }
}

fn calculate_winnings(hands: Vec<Hand>) -> u64 {
    // into_sorted_vec undoes the min-ordering and returns high-to-low...
    // dbg!(BinaryHeap::from(hands).into_sorted_vec());

    // into_iter doesn't handle tie-breakers...
    // BinaryHeap::from(hands).into_iter().enumerate().fold(0, |acc, (i, hand)| acc + ((i + 1) as u64 * hand.bid))

    let mut heap = BinaryHeap::from(hands);
    let mut rank = 1;
    let mut total = 0;
    while let Some(Hand { ty, cards, bid }) = heap.pop() {
        println!("Hand: {:?} {:?}", ty, cards);
        total += rank * bid;
        rank += 1;
    }

    total
}


fn solve_part_1(input: &str) -> u64 {
    let hands: Vec<Hand> = input.lines().map(|hand| {
        let (cards, bid) = hand.split_once(' ').unwrap();
        Hand::new(cards, bid.parse::<u64>().unwrap())
    }).collect();

    // dbg!(&hands);

    calculate_winnings(hands)
}

fn solve_part_2(input: &str) -> u64 {
    let hands: Vec<Hand> = input.lines().map(|hand| {
        let (cards, bid) = hand.split_once(' ').unwrap();
        Hand::new_with_jokers(cards, bid.parse::<u64>().unwrap())
    }).collect();

    // dbg!(&hands);

    calculate_winnings(hands)
}

pub fn answers() {
    let input = include_str!("input.txt").trim();

    println!("[P1 :: INFO] Answer: {}", solve_part_1(input));
    println!("[P2 :: INFO] Answer: {}", solve_part_2(input));
}
