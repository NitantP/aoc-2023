use std::collections::{HashMap, HashSet};

use regex::Regex;
use once_cell::sync::Lazy;

// https://adventofcode.com/2023/day/4

static CARD_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^Card\s+(?<id>\d+):\s+(?<winning_numbers>[0-9\s]+)\s\|\s+(?<card_numbers>[0-9\s]+)$").unwrap());

fn solve_part_1(input: &str) -> u64 {
    fn check_card(card: &str) -> u64 {
        if let Some(captures) = CARD_RE.captures(card) {
            let winning_numbers: &HashSet<&str> = &captures["winning_numbers"].split(' ').filter(|&s| s != "").collect();
            let card_numbers: &HashSet<&str> = &captures["card_numbers"].split(' ').filter(|&s| s != "").collect();

            match winning_numbers.intersection(card_numbers).count() {
                0 => 0,
                num_matches => 1 << (num_matches - 1)
            }
        } else {
            eprintln!("[P1 :: ERROR] Expected to find regex captures but found none for: {}", card);
            0
        }
    }

    input.lines().map(check_card).sum()
}

fn solve_part_2(input: &str) -> u64 {
    fn check_card(card: &str, checked_cards: &mut HashMap<u8, u64>) -> u64 {
        if let Some(captures) = CARD_RE.captures(card) {
            let winning_numbers: &HashSet<&str> = &captures["winning_numbers"].split(' ').filter(|&s| s != "").collect();
            let card_numbers: &HashSet<&str> = &captures["card_numbers"].split(' ').filter(|&s| s != "").collect();

            let num_matches: u8 = winning_numbers.intersection(card_numbers).count() as u8;
            let card_id: u8 = *(&captures["id"].parse().unwrap());

            let num_additional_cards: u64 = num_matches as u64 + (card_id + 1..=card_id + num_matches).map(|id| checked_cards.get(&id).or(Some(&0)).unwrap()).sum::<u64>();
            checked_cards.insert(card_id, num_additional_cards);

            1 + num_additional_cards
        } else {
            eprintln!("[P1 :: ERROR] Expected to find regex captures but found none for: {}", card);
            0
        }
    }

    let mut checked_cards: HashMap<u8, u64> = HashMap::new();
    input.lines().rev().map(|card| check_card(card, &mut checked_cards)).sum()
}

pub fn answers() {
    let input = include_str!("input.txt").trim();

    println!("[P1 :: INFO] Answer: {}", solve_part_1(input));
    println!("[P2 :: INFO] Answer: {}", solve_part_2(input));
}
