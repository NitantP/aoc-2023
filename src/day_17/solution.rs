use std::{collections::{BinaryHeap, HashSet}, cmp::Ordering};

// https://adventofcode.com/2023/day/17

type Position = (usize, usize);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Start,
}

impl Direction {
    fn delta(&self) -> (i8, i8) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
            _ => (0, 0),
        }
    }

    fn next(position: &Position, direction: &Self, row_max: usize, col_max: usize) -> Result<Position, ()> {
        let (dr, dc) = direction.delta();

        let next_row = position.0 as i32 + dr as i32;
        let next_col = position.1 as i32 + dc as i32;

        if next_row < 0 || next_row >= row_max as i32 || next_col < 0 || next_col >= col_max as i32 {
            Err(())
        } else {
            Ok((next_row as usize, next_col as usize))
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct State {
    position: Position,
    cost: u64,
    direction: Direction,
    straight_moves: u8, 
}

impl State {
    fn new(position: Position, cost: u64, direction: Direction, straight_moves: u8) -> Self {
        Self {
            position,
            cost,
            direction,
            straight_moves
        }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_grid(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|l| {
        l.chars().map(|n| n.to_digit(10).unwrap() as u8).collect()
    }).collect()
}

fn find_cheapest_path(grid: Vec<Vec<u8>>, start: Position, end: Position, min_consecutive: u8, max_consecutive: u8) -> u64 {
    let mut visited = HashSet::<(Position, Direction, u8)>::new();
    let mut heap = BinaryHeap::<State>::from([
        State::new(
            start,
            0,
            Direction::Start,
            0,
        )
    ]);

    while let Some(State { position, cost, direction, straight_moves }) = heap.pop() {
        if position == end {
            return cost;
        }
        
        let heading = (position, direction, straight_moves);
        if visited.contains(&heading) {
            continue;
        }
        visited.insert(heading);

        let next_directions = match direction {
            Direction::Start => Vec::from([Direction::Right, Direction::Up, Direction::Down]),
            _ if straight_moves < min_consecutive => Vec::from([direction]),
            Direction::Up | Direction::Down if straight_moves < max_consecutive => Vec::from([Direction::Left, Direction::Right, direction]),
            Direction::Left | Direction::Right if straight_moves < max_consecutive => Vec::from([Direction::Up, Direction::Down, direction]),
            Direction::Up | Direction::Down => Vec::from([Direction::Left, Direction::Right]),
            Direction::Left | Direction::Right => Vec::from([Direction::Up, Direction::Down]),
        };

        for d in next_directions {
            if let Ok(next) = Direction::next(&position, &d, grid.len(), grid[0].len()) {
                heap.push(
                    State::new(
                        next,
                        cost + grid[next.0][next.1] as u64,
                        d,
                        if d == direction { straight_moves + 1 } else { 1 },
                    )
                )
            }
        }

        // dbg!(&heap);
    }

    0
}

fn solve_part_1(input: &str) -> u64 {
    let grid = parse_grid(input);

    let start = (0, 0);
    let end = (grid.len() - 1, grid[0].len() - 1);

    find_cheapest_path(grid, start, end, 0, 3)
}

fn solve_part_2(input: &str) -> u64 {
    let grid = parse_grid(input);

    let start = (0, 0);
    let end = (grid.len() - 1, grid[0].len() - 1);

    find_cheapest_path(grid, start, end, 4, 10)
}

pub fn answers() {
    // let input = include_str!("input.txt").trim();
    let input = include_str!("test.txt").trim();

    println!("[P1 :: INFO] Answer: {}", solve_part_1(input));
    println!("[P2 :: INFO] Answer: {}", solve_part_2(input));
}
