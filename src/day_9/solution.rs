// https://adventofcode.com/2023/day/9

enum ExtrapolationDirection {
    Previous,
    Next
}

fn calculate_differences(nums: Vec<i64>, values_to_keep: &ExtrapolationDirection) -> Vec<i64> {
    let mut differences = nums;
    let mut values = Vec::new();

    loop {
        let mut it = differences.iter();
        let mut current_value = *it.next().unwrap();
        let mut next_differences = Vec::new();

        if matches!(values_to_keep, ExtrapolationDirection::Previous) {
            values.push(current_value);
        }

        while let Some(&value) = it.next() {
            next_differences.push(value - current_value);
            current_value = value;
        }

        if matches!(values_to_keep, ExtrapolationDirection::Next) {
            values.push(current_value);
        }

        if next_differences.iter().all(|&x| x == 0) {
            break;
        }

        differences = next_differences;
    }

    values
}

fn extrapolate_value(line: &str, direction: ExtrapolationDirection) -> i64 {
    let nums = line.split(' ').map(|c| c.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    let last_values = calculate_differences(nums, &direction);

    match direction {
        ExtrapolationDirection::Next => last_values.iter().rev().fold(0, |acc, x| acc + x),
        ExtrapolationDirection::Previous => last_values.iter().rev().fold(0, |acc, x| x - acc),
    }
}

fn solve_part_1(input: &str) -> i64 {
    input.lines().map(|l| extrapolate_value(l, ExtrapolationDirection::Next)).sum()
}

fn solve_part_2(input: &str) -> i64 {
    input.lines().map(|l| extrapolate_value(l, ExtrapolationDirection::Previous)).sum()
}

pub fn answers() {
    let input = include_str!("input.txt").trim();
    // let input = include_str!("test.txt").trim();

    println!("[P1 :: INFO] Answer: {}", solve_part_1(input));
    println!("[P2 :: INFO] Answer: {}", solve_part_2(input));
}
