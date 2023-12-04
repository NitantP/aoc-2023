// use std::collections::HashMap;

use regex::Regex;
use once_cell::sync::Lazy;

// https://adventofcode.com/2023/day/2

// const MAX_COUNTS: Lazy<HashMap<&str, u8>> = Lazy::new( || HashMap::from([
//     ("red", 12),
//     ("green", 13),
//     ("blue", 14),
// ]));

static GAME_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^Game (?<id>\d+):\s(?<rounds>.*)$").unwrap());
// static ROUND_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(?<amount>\d+)\s(?<color>\w+)$").unwrap());

static RED_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?<amount>\d+)\sred").unwrap());
static GREEN_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?<amount>\d+)\sgreen").unwrap());
static BLUE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?<amount>\d+)\sblue").unwrap());

macro_rules! get_color_count {
    ($re:ident, $hay:ident) => (
        (match $re.captures($hay).and_then(|captures| captures.name("amount")) {
            Some(value) => value.as_str().parse::<u64>().unwrap(),
            None => 0, 
        })
    )
}

fn solve_part_1(input: &str) -> u64 {
    fn check_rounds(rounds: &str) -> bool {
        let rounds = rounds.split(';');

        for round in rounds {
            // for count in round.split(',') {
            //     let captures = ROUND_RE.captures(count.trim()).unwrap();
            //     let color = &captures["color"];
            //     let amount = &captures["amount"].parse::<u8>().unwrap();
            //
            //     if MAX_COUNTS.get(color).unwrap() < amount{
            //         return false;
            //     }
            // }
            
            let is_red_valid = get_color_count!(RED_RE, round) <= 12;
            let is_green_valid = get_color_count!(GREEN_RE, round) <= 13;
            let is_blue_valid = get_color_count!(BLUE_RE, round) <= 14;

            if !is_red_valid || !is_blue_valid || !is_green_valid {
                return false;
            }
        }

        true
    }

    fn check_game(game: &str) -> u64 {
        match GAME_RE.captures(game) {
            Some(captures) if check_rounds(&captures["rounds"]) => *(&captures["id"].parse::<u64>().unwrap()),
            _ => 0
        }
    }

    input.lines().map(check_game).sum()
}

fn solve_part_2(input: &str) -> u64 {
    fn check_rounds(rounds: &str) -> (u64, u64, u64) {
        let rounds = rounds.split(';');

        let round_counts = rounds.map(|round| {
            let red_count = get_color_count!(RED_RE, round);
            let green_count = get_color_count!(GREEN_RE, round);
            let blue_count = get_color_count!(BLUE_RE, round);

            (red_count, green_count, blue_count)
        });

        round_counts.fold((1u64, 1u64, 1u64), |acc, x| (acc.0.max(x.0), acc.1.max(x.1), acc.2.max(x.2)))
    }

    fn check_game(game: &str) -> u64 {
        match GAME_RE.captures(game) {
            Some(captures) => {
                let counts = check_rounds(&captures["rounds"]);
                (counts.0 * counts.1 * counts.2) as u64
            }
            _ => 0
        }
    }

    input.lines().map(check_game).sum()
}

pub fn answers() {
    let input = include_str!("input.txt").trim();

    println!("[P1 :: INFO] Answer: {}", solve_part_1(input));
    println!("[P2 :: INFO] Answer: {}", solve_part_2(input));
}
