use std::{str::Lines, collections::HashMap};

use num::Integer;
use regex::Regex;
use once_cell::sync::Lazy;

// https://adventofcode.com/2023/day/8

static NODE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(?<node>[A-Z]+) = \((?<left>[A-Z]+), (?<right>[A-Z]+)\)$").unwrap());

fn solve_part_1(input: &str) -> u64 {
    fn construct_network(nodes: &mut Lines) -> HashMap<String, (String, String)> {
        let mut network = HashMap::<String, (String, String)>::new();

        while let Some(node) = nodes.next() {
            let captures = NODE_RE.captures(node).unwrap();
            network.insert(captures["node"].to_owned(), (captures["left"].to_owned(), captures["right"].to_owned()));
        }

        network
    }

    fn follow_directions(directions: &str, start: &str, network: &HashMap<String, (String, String)>) -> u64 {
        let mut steps = 0;
        let mut directions = directions.chars().cycle();
        let mut node = start;

        loop {
            steps += 1;

            let next = network.get(node).unwrap();
            match directions.next().unwrap() {
                'L' => {
                    node = &next.0;
                }
                'R' => {
                    node = &next.1;
                }
                otherwise => panic!("Unknown direction: {:?}", otherwise)
            }

            if node == "ZZZ" {
                break
            }
        }

        steps
    }

    let mut map = input.lines();

    let directions = map.next().unwrap();
    map.next(); // Consume empty line between directions and node network
    let network = construct_network(&mut map);

    follow_directions(directions, &"AAA", &network)
}

fn solve_part_2(input: &str) -> u64 {
    fn construct_network(nodes: &mut Lines) -> (Vec<String>, HashMap<String, (String, String)>) {
        let mut start_nodes = Vec::new();
        let mut network = HashMap::<String, (String, String)>::new();

        while let Some(node) = nodes.next() {
            let captures = NODE_RE.captures(node).unwrap();

            let node = captures["node"].to_owned();
            let left = captures["left"].to_owned();
            let right = captures["right"].to_owned();

            if node.ends_with("A") {
                start_nodes.push(node.clone());
            }

            network.insert(node, (left, right));
        }

        (start_nodes, network)
    }

    fn find_distances_to_z(directions: &str, start_nodes: &Vec<String>, network: &HashMap<String, (String, String)>) -> Vec<u64> {
        let mut distances = Vec::new();

        for node in start_nodes {
            let mut directions = directions.chars().cycle();
            let mut current_node = node;
            let mut steps = 0;
            
            while !current_node.ends_with("Z") {
                steps += 1;
                match directions.next().unwrap() {
                    'L' => current_node = &(network.get(current_node).unwrap().0),
                    'R' => current_node = &(network.get(current_node).unwrap().1),
                    otherwise => panic!("Unknown direction: {:?}", otherwise)
                }
            }

            distances.push(steps);
        }

        distances
    }

    let mut map = input.lines();

    let directions = map.next().unwrap();
    map.next(); // Consume empty line between directions and node network
    let (start_nodes, network) = construct_network(&mut map);

    /*
    * Problem statement is very poorly worded and does not state any crucial assumptions that must
    * be made in order for least common multiple to be a tractable approach.
    * ---
    * """
    * Nothing in the problem statement guarantees that any of this would be true, but all the provided inputs for this problem have special properties even if they were not explicitly spelled out:
    * - Each --A node only reaches one --Z node in it's loop
    * - they all reach their --Z at the same "step" in the directions every time, meaning the loops are all a consistent period instead of changing or branching
    * - Conveniently the period it takes to reach the first --Z from the starting --A node is the same period as it takes to re-reach the --Z node when you're already there.
    * """
    * See: https://www.reddit.com/r/adventofcode/comments/18dfpub/2023_day_8_part_2_why_is_spoiler_correct/
    */
    find_distances_to_z(directions, &start_nodes, &network).iter().fold(1, |acc, d| acc.lcm(d))
}

pub fn answers() {
    let input = include_str!("input.txt").trim();
    // let input = include_str!("test.txt").trim();

    println!("[P1 :: INFO] Answer: {}", solve_part_1(input));
    println!("[P2 :: INFO] Answer: {}", solve_part_2(input));
}
