use std::{collections::{HashSet, HashMap}, iter::{self, Peekable, Enumerate}, str::Chars};

// https://adventofcode.com/2023/day/3

fn solve_part_1(input: &str) -> u64 {
    fn find_symbol_locations(schematic: &str) -> HashSet<(usize, usize)> {
        let mut locations = HashSet::<(usize, usize)>::new();

        for (row, line) in schematic.lines().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if !ch.is_ascii_digit() && ch != '.' {
                    locations.insert((row, col));
                }
            }
        }

        locations
    }

    fn is_symbol_adjacent(row: usize, col_span: &(usize, usize), symbol_locations: &HashSet<(usize, usize)>) -> bool {
        let row_min: usize = if row == 0 { 0 } else { row - 1 };
        let col_min: usize = if col_span.0 == 0 { 0 } else { col_span.0 - 1 };
        let row_above = (col_min..=col_span.1 + 1).map(|col| (row_min, col));
        let row_below = (col_min..=col_span.1 + 1).map(|col| (row + 1, col));
        let row_beginning = (row, col_min);
        let row_end = (row, col_span.1 + 1);

        let mut adjacent_locations = iter::once(row_beginning)
            .chain(iter::once(row_end))
            .chain(row_above)
            .chain(row_below);

        adjacent_locations.any(|location| symbol_locations.contains(&location))
    }

    fn parse_number(chars: &mut Peekable<Enumerate<Chars>>) -> (u64, usize) {
        let mut num: u64 = 0;
        let mut end: usize = usize::MAX;

        while let Some(&(col, ch)) = chars.peek() {
            if !ch.is_ascii_digit() {
                break;
            }

            num = (num * 10) + (ch.to_digit(10).unwrap()) as u64;
            end = col;

            chars.next();
        }

        (num, end)
    }

    fn process_schematic_line(row: usize, line: &str, symbol_locations: &HashSet<(usize, usize)>) -> u64 {
        let mut line_sum: u64 = 0;
        
        let mut chars = line.chars().enumerate().peekable();
        while let Some(&(start, ch)) = chars.peek() {
            if ch.is_ascii_digit() {
                let (num, end) = parse_number(&mut chars);
                if is_symbol_adjacent(row, &(start, end), symbol_locations) {
                    line_sum += num;
                }
            } else {
                chars.next();
            }
        }

        line_sum
    }

    let symbol_locations = find_symbol_locations(input);
    input.lines().enumerate().map(|(row, line)| process_schematic_line(row, line, &symbol_locations)).sum()
}

fn solve_part_2(input: &str) -> u64 {
    fn find_gear_locations(schematic: &str) -> HashSet<(usize, usize)> {
        let mut locations = HashSet::<(usize, usize)>::new();

        for (row, line) in schematic.lines().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if ch == '*' {
                    locations.insert((row, col)); 
                }
            }
        }

        locations
    }

    fn find_adjacent_gears<'a>(row: usize, col_span: &'a(usize, usize), gear_locations: &HashSet<(usize, usize)>) -> Vec<(usize, usize)> {
        let row_min: usize = if row == 0 { 0 } else { row - 1 };
        let col_min: usize = if col_span.0 == 0 { 0 } else { col_span.0 - 1 };
        let row_above = (col_min..=col_span.1 + 1).map(|col| (row_min, col));
        let row_below = (col_min..=col_span.1 + 1).map(|col| (row + 1, col));
        let row_beginning = (row, col_min);
        let row_end = (row, col_span.1 + 1);

        let adjacent_locations = iter::once(row_beginning)
            .chain(iter::once(row_end))
            .chain(row_above)
            .chain(row_below);

        adjacent_locations.filter(|location| gear_locations.contains(&location)).collect()
    }

    fn parse_number(chars: &mut Peekable<Enumerate<Chars>>) -> (u64, usize) {
        let mut num: u64 = 0;
        let mut end: usize = usize::MAX;

        while let Some(&(col, ch)) = chars.peek() {
            if !ch.is_ascii_digit() {
                break;
            }

            num = (num * 10) + (ch.to_digit(10).unwrap()) as u64;
            end = col;

            chars.next();
        }

        (num, end)
    }

    fn process_schematic_line(row: usize, line: &str, gear_locations: &HashSet<(usize, usize)>, gear_adjacent_parts: &mut HashMap<(usize, usize), Vec<u64>>) {
        let mut chars = line.chars().enumerate().peekable();
        while let Some(&(start, ch)) = chars.peek() {
            if ch.is_ascii_digit() {
                let (num, end) = parse_number(&mut chars);
                for gear_location in find_adjacent_gears(row, &(start, end), gear_locations) {
                    gear_adjacent_parts.entry(gear_location).and_modify(|v| v.push(num)).or_insert(vec![num]);
                }
            } else {
                chars.next();
            }
        }
    }

    let gear_locations = find_gear_locations(input);
    let mut gear_adjacent_parts = HashMap::<(usize, usize), Vec::<u64>>::new();

    for (row, line) in input.lines().enumerate() {
        process_schematic_line(row, line, &gear_locations, &mut gear_adjacent_parts);
    }

    gear_adjacent_parts.values().filter(|adjacent_parts| adjacent_parts.len() == 2).fold(0, |sum, adjacent_parts| sum + (adjacent_parts[0] * adjacent_parts[1]))
}

pub fn answers() {
    let input = include_str!("input.txt").trim();

    println!("[P1 :: INFO] Answer: {}", solve_part_1(input));
    println!("[P2 :: INFO] Answer: {}", solve_part_2(input));
}
