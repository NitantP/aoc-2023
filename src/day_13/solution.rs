use regex::Regex;
use once_cell::sync::Lazy;

// https://adventofcode.com/2023/day/13

static PATTERN_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?<pattern>(?:[\.|\#]+\n)+)\n").unwrap());

type DataLine = Vec<char>;

fn parse_pattern(pattern: &str) -> (Vec<DataLine>, Vec<DataLine>) {
    let mut rows = Vec::<DataLine>::new();
    let mut cols = Vec::<DataLine>::new();

    for line in pattern.lines() {
        rows.push(line.chars().collect());

        for (col, c) in line.char_indices() {
            if col >= cols.len() {
                cols.push(vec![c]);
            } else {
                cols[col].push(c);
            }
        }
    }

    (rows, cols)
}

fn is_symmetric_axis(start: usize, data: &Vec<DataLine>) -> Option<u64> {
    let mut left = start - 1;
    let mut right = start;

    while left >= 0 && right < data.len() && data[left] == data[right] {
        if left == 0 || right == data.len() - 1 {
            return Some(start as u64);
        }

        left -= 1;
        right += 1;
    }

    None
}

fn find_reflection_line(data: &Vec<DataLine>) -> Option<u64> {
    if data.len() == 1 {
        return None;
    }

    for axis in 1..data.len() {
        let check = is_symmetric_axis(axis, data);

        if check.is_some() {
            return check;
        }
    }

    None
}

fn can_fix(l1: &DataLine, l2: &DataLine) -> bool {
    l1.iter().zip(l2.iter()).filter(|(a, b)| a != b).count() == 1
}

fn is_symmetric_axis_with_smudge(start: usize, data: &Vec<DataLine>) -> Option<u64> {
    let mut left = start - 1;
    let mut right = start;
    let mut fixed_smudge: bool = false;

    while left >= 0 && right < data.len() {
        if data[left] != data[right] {
            if !fixed_smudge && can_fix(&data[left], &data[right]) {
                fixed_smudge = true;
            } else {
                break;
            }
        }

        if data[left] != data[right] {
            fixed_smudge = true;
        }

        if left == 0 || right == data.len() - 1 {
            return if fixed_smudge { Some(start as u64) } else { None };
        }

        left -= 1;
        right += 1;
    }

    None
}

fn find_reflection_line_with_smudge(data: &Vec<DataLine>) -> Option<u64> {
    if data.len() == 1 {
        return None;
    }

    for axis in 1..data.len() {
        let check = is_symmetric_axis_with_smudge(axis, data);

        if check.is_some() {
            return check;
        }
    }

    None
}

fn solve_part_1(input: &str) -> u64 {
    let mut total: u64 = 0;

    let captures = PATTERN_RE.captures_iter(input);
    for capture in captures {
        let pattern = &capture["pattern"];

        let (rows, cols) = parse_pattern(pattern);
        // dbg!(&rows);
        // dbg!(&cols);

        total += {
            if let Some(n) = find_reflection_line(&rows) {
                n * 100
            } else if let Some(n) = find_reflection_line(&cols) {
                n
            } else {
                eprintln!("[P1 :: ERROR] Line of reflection not found for pattern {}", pattern);
                0
            }
        };
    }

    total
}

fn solve_part_2(input: &str) -> u64 {
    let mut total: u64 = 0;

    let captures = PATTERN_RE.captures_iter(input);
    for capture in captures {
        let pattern = &capture["pattern"];

        let (rows, cols) = parse_pattern(pattern);
        // dbg!(&rows);
        // dbg!(&cols);

        total += {
            if let Some(n) = find_reflection_line_with_smudge(&rows) {
                n * 100
            } else if let Some(n) = find_reflection_line_with_smudge(&cols) {
                n
            } else {
                eprintln!("[P1 :: ERROR] Line of reflection not found for pattern {}", pattern);
                0
            }
        };
    }

    total
}

pub fn answers() {
    let mut input = String::from(include_str!("input.txt").trim());
    // let mut input = String::from(include_str!("test.txt").trim());
    input.push_str("\n\n");

    // println!("[P1 :: INFO] Answer: {}", solve_part_1(&input));
    println!("[P2 :: INFO] Answer: {}", solve_part_2(&input));
}
