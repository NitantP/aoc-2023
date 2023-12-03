use crate::utils::trie::Trie;

// https://adventofcode.com/2023/day/1

fn solve_part_1(input: &str) -> u64 {

    fn calculate_calibration_value(line: &str) -> u64 {
        let first_num = line.chars().find(|x| x.is_ascii_digit()).unwrap().to_digit(10).unwrap();
        let last_num = line.chars().rfind(|x| x.is_ascii_digit()).unwrap().to_digit(10).unwrap();

        ((first_num * 10) + last_num) as u64
    }

    input.lines().map(calculate_calibration_value).sum()
}

fn solve_part_2(input: &str) -> u64 {

    fn initialize_trie() -> Trie {
        let digits = Vec::from([
            ("one", '1'),
            ("two", '2'),
            ("three", '3'),
            ("four", '4'),
            ("five", '5'),
            ("six", '6'),
            ("seven", '7'),
            ("eight", '8'),
            ("nine", '9'),
            ("eno", '1'),
            ("owt", '2'),
            ("eerht", '3'),
            ("ruof", '4'),
            ("evif", '5'),
            ("xis", '6'),
            ("neves", '7'),
            ("thgie", '8'),
            ("enin", '9'),
        ]);

        Trie::from(digits)
    }

    fn calculate_calibration_value(line: &str) -> u64 {
        let trie = initialize_trie();

        let mut digit_string = String::new();
        let mut first_num = 0;
        let mut last_num = 0;

        for c in line.chars() {
            if c.is_ascii_digit() {
                first_num = c.to_digit(10).unwrap();
                break;
            }

            digit_string.push(c);
            match trie.contains(&digit_string) {
                Some('\0') => {},
                Some(digit_char) => {
                    first_num = digit_char.to_digit(10).unwrap();
                    break;
                }
                None => {
                    while !digit_string.is_empty() {
                        digit_string.remove(0);
                        match trie.contains(&digit_string) {
                            Some('\0') => break,
                            Some(digit_char) => {
                                first_num = digit_char.to_digit(10).unwrap();
                                break;
                            }
                            None => {},
                        }
                    }
                },
            }
        }

        digit_string.clear();

        for c in line.chars().rev() {
            if c.is_ascii_digit() {
                last_num = c.to_digit(10).unwrap();
                break;
            }

            digit_string.push(c);
            match trie.contains(&digit_string) {
                Some('\0') => {},
                Some(digit_char) => {
                    last_num = digit_char.to_digit(10).unwrap();
                    break;
                }
                None => {
                    while !digit_string.is_empty() {
                        digit_string.remove(0);
                        match trie.contains(&digit_string) {
                            Some('\0') => break,
                            Some(digit_char) => {
                                last_num = digit_char.to_digit(10).unwrap();
                                break;
                            }
                            None => {},
                        }
                    }
                }
            }
        }

        ((first_num * 10) + last_num) as u64
    }

    input.lines().map(calculate_calibration_value).sum()
}

pub fn answers() {
    let input = include_str!("input.txt").trim();

    println!("[P1 :: INFO] Answer: {}", solve_part_1(input));
    println!("[P2 :: INFO] Answer: {}", solve_part_2(input));
}
