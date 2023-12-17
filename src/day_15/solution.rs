use std::collections::HashMap;

// https://adventofcode.com/2023/day/15

fn hash(s: &str) -> u64 {
    let mut hash: u64 = 0;

    for c in s.chars() {
        hash += c as u64;
        hash *= 17;
        hash %= 256;
    }

    hash
}

fn remove_lens(lens: &str, hashmap: &mut HashMap<u64, Vec<(String, u64)>>) {
    let key = hash(lens);
    hashmap.entry(key).and_modify(|v| { v.retain(|l| l.0 != lens) });
}

fn add_lens(lens: &str, focal_length: u64, hashmap: &mut HashMap<u64, Vec<(String, u64)>>) {
    let key = hash(lens);
    let mut lenses = hashmap.entry(key).or_insert(Vec::new());

    for l in lenses.iter_mut() {
        if l.0 == lens {
            l.1 = focal_length;
            return;
        } 
    }

    lenses.push((lens.to_owned(), focal_length));
}

fn configure_lens(step: &str, hashmap: &mut HashMap<u64, Vec<(String, u64)>>) {
    if step.contains("-") {
        remove_lens(&step[..step.len() - 1], hashmap)
    } else if step.contains("=") {
        let mut _step = step.split('=');
        let lens = _step.next().unwrap();
        let focal_length = _step.next().unwrap().parse::<u64>().unwrap();

        add_lens(lens, focal_length, hashmap);
    } else {
        panic!("Unrecognized step pattern: {}", step);
    }
}

fn solve_part_1(input: &str) -> u64 {
    input.split(',').filter(|&s| s != "").map(hash).sum()    
}

fn solve_part_2(input: &str) -> u64 {
    let mut hashmap = HashMap::<u64, Vec<(String, u64)>>::new();

    input.split(',').filter(|&s| s != "").for_each(|s| configure_lens(s, &mut hashmap));

    hashmap.iter().map(|(k, v)| v.iter().enumerate().fold(0, |acc, (i, lens)| acc + ((k + 1) * (i + 1) as u64 * lens.1))).sum()
}

pub fn answers() {
    let input = include_str!("input.txt").trim();
    // let input = include_str!("test.txt").trim();

    println!("[P1 :: INFO] Answer: {}", solve_part_1(input));
    println!("[P2 :: INFO] Answer: {}", solve_part_2(input));
}
