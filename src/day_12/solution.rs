use regex::Regex;
use once_cell::sync::Lazy;

// https://adventofcode.com/2023/day/12

static SPRING_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(?<springs>(\?|\#|\.)+)\s(?<groups>[0-9,]+)$").unwrap());

fn parse_line(line: &str, fold_factor: usize) -> (String, Vec<usize>) {
    let captures = SPRING_RE.captures(line).unwrap();

    // add '.' at the end to avoid boundary errors in validation logic
    // i.e., always expect a group of damaged springs to end with '.'
    // if there are no damaged springs at the end of the string, it won't matter anyway
    let mut springs = String::from(&captures["springs"]);
    if fold_factor > 1 {
        springs.push('?');
        springs = springs.repeat(fold_factor);
        springs.pop();
    }
    springs.push('.');

    let mut groups: Vec<usize> = (&captures["groups"]).split(',').map(|n| n.parse::<usize>().unwrap()).rev().collect();
    groups = groups.repeat(fold_factor);
    groups.insert(0, 0);

    (springs, groups)
}

fn is_valid(springs: &str, num_damaged: usize) -> bool {
    if springs.is_empty() && num_damaged == 0 {
        return true;
    }

    if springs.is_empty() || springs.len() <= num_damaged {
        return false;
    }

    let end = springs.chars().nth(num_damaged).unwrap();

    let valid = (end == '.' || end == '?') &&
    (springs.chars().take(num_damaged).all(|c| c == '?' || c == '#'));

    valid
}

fn count_arrangements(line: &str, fold_factor: usize) -> u64 {
    let (springs, groups) = parse_line(line, fold_factor);
    let mut memo = vec![vec![0; springs.len() + 1]; groups.len()];

    // initialize base case where there are no damaged groups remaining and only '.'s remain
    // '?'s are assumed to be '.'s in this case since there are no damaged groups remaining
    let mut idx: usize = springs.len() - 1;
    while idx >= 0 && ['.', '?'].contains(&springs.chars().nth(idx).unwrap()) {
        memo[0][idx + 1] = 1;

        if idx == 0 {
            break;
        }

        idx -= 1;
    }

    for i in 1..groups.len() {
        // -2 because '.' appended to end while parsing for even-ness, so we skip it
        for j in (0..=(springs.len() - 2)).rev() {
            match springs.chars().nth(j).unwrap() {
                // possible arrangements depends on rest of string since '.' is a skip
                '.' => memo[i][j] = memo[i][j + 1], 

                // possible arrangements depends on if the group is valid for the remaining string
                '#' => memo[i][j] = if is_valid(&springs[j..], groups[i]) { memo[i - 1][j + groups[i] + 1] } else { 0 },

                // count arrangements in both cases where '?' is a '.' and a '#'
                '?' => memo[i][j] = memo[i][j + 1] + if is_valid(&springs[j..], groups[i]) { memo[i - 1][j + groups[i] + 1] } else { 0 },

                otherwise => panic!("Unrecognized spring character {}", otherwise),
            }
        }
    }

    // in the memo table, this is the full group list with the full string
    memo[groups.len() - 1][0]
}

fn solve_part_1(input: &str) -> u64 {
    input.lines().map(|line| count_arrangements(line, 1)).sum()
}

fn solve_part_2(input: &str) -> u64 {
    input.lines().map(|line| count_arrangements(line, 5)).sum()
}

pub fn answers() {
    let input = include_str!("input.txt").trim();
    // let input = include_str!("test.txt").trim();

    println!("[P1 :: INFO] Answer: {}", solve_part_1(input));
    println!("[P2 :: INFO] Answer: {}", solve_part_2(input));
}
