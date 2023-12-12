use std::collections::HashSet;

// https://adventofcode.com/2023/day/11

fn parse_image(image: &str, galaxies: &mut Vec<(usize, usize)>, occupied_rows: &mut HashSet<usize>, occupied_cols: &mut HashSet<usize>) {
    for (row, line) in image.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                occupied_rows.insert(row);
                occupied_cols.insert(col);
                galaxies.push((row, col));
            }
        }
    }
}

fn calculate_directional_distance(start: usize, end: usize, occupied: &HashSet<usize>, expansion_factor: usize) -> usize {
    (start..end).fold(0, |acc, i| if occupied.contains(&i) { acc + 1 } else { acc + expansion_factor })
}

fn calculate_galaxy_distances(galaxies: &Vec<(usize, usize)>, occupied_rows: &HashSet<usize>, occupied_cols: &HashSet<usize>, expansion_factor: usize) -> usize {
    let mut distance: usize = 0;

    for (i, g1) in galaxies.iter().enumerate() {
        for g2 in galaxies.iter().skip(i + 1) {
            // row distance
            distance += calculate_directional_distance(g1.0.min(g2.0), g1.0.max(g2.0), occupied_rows, expansion_factor);

            // col distance
            distance += calculate_directional_distance(g1.1.min(g2.1), g1.1.max(g2.1), occupied_cols, expansion_factor);
        }
    }

    distance
}

fn solve_part_1(input: &str) -> usize {
    let mut galaxies = Vec::<(usize, usize)>::new();
    let mut occupied_rows = HashSet::<usize>::new();
    let mut occupied_cols = HashSet::<usize>::new();

    parse_image(input, &mut galaxies, &mut occupied_rows, &mut occupied_cols);

    calculate_galaxy_distances(&galaxies, &occupied_rows, &occupied_cols, 2)
}

fn solve_part_2(input: &str) -> usize {
    let mut galaxies = Vec::<(usize, usize)>::new();
    let mut occupied_rows = HashSet::<usize>::new();
    let mut occupied_cols = HashSet::<usize>::new();

    parse_image(input, &mut galaxies, &mut occupied_rows, &mut occupied_cols);

    calculate_galaxy_distances(&galaxies, &occupied_rows, &occupied_cols, 1000000)
}

pub fn answers() {
    let input = include_str!("input.txt").trim();
    // let input = include_str!("test.txt").trim();

    // println!("[P1 :: INFO] Answer: {}", solve_part_1(input));
    println!("[P2 :: INFO] Answer: {}", solve_part_2(input));
}
