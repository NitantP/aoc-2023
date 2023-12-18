
use std::collections::{VecDeque, HashSet};

// https://adventofcode.com/2023/day/16

type Position = (usize, usize);

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn delta(&self) -> (i8, i8) {
        match self {
            Self::Up => (-1, 0),
            Self::Down => (1, 0),
            Self::Left => (0, -1),
            Self::Right => (0, 1),
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Beam {
    position: Position,
    direction: Direction,
}

impl Beam {
    fn new(position: Position, direction: Direction) -> Self {
        Self {
            position,
            direction,
        }
    }

    fn next(position: Position, direction: Direction) -> Self {
        let (delta_row, delta_col) = direction.delta();
        let row = ((position.0 as i8) + delta_row) as usize;
        let col = ((position.1 as i8) + delta_col) as usize;

        Self::new((row, col), direction)
    }
}

fn trace_light_beam(start: Beam, grid: &Vec<Vec<char>>) -> u64 {
    let mut visited = HashSet::<Beam>::new();

    let mut deq = VecDeque::from([start]);
    while !deq.is_empty() {
        let beam = deq.pop_front().unwrap();

        if visited.contains(&beam) {
            continue;
        }

        let tile = grid[beam.position.0][beam.position.1];
        match tile {
            'O' => {}

            '.' => {
                deq.push_back(Beam::next(beam.position, beam.direction));
            }

            '|' => {
                match beam.direction {
                    Direction::Up | Direction::Down => deq.push_back(Beam::next(beam.position, beam.direction)),
                    Direction:: Left | Direction::Right => deq.extend([Beam::next(beam.position, Direction::Up), Beam::next(beam.position, Direction::Down)]),
                }
            }

            '-' => {
                match beam.direction {
                    Direction::Up | Direction::Down => deq.extend([Beam::next(beam.position, Direction::Left), Beam::next(beam.position, Direction::Right)]),
                    Direction:: Left | Direction::Right => deq.push_back(Beam::next(beam.position, beam.direction)),
                }
            }

            '/' => {
                match beam.direction {
                    Direction::Up => deq.push_back(Beam::next(beam.position, Direction::Right)),
                    Direction::Down => deq.push_back(Beam::next(beam.position, Direction::Left)),
                    Direction::Left => deq.push_back(Beam::next(beam.position, Direction::Down)),
                    Direction::Right => deq.push_back(Beam::next(beam.position, Direction::Up)),
                }
            }

            '\\' => {
                match beam.direction {
                    Direction::Up => deq.push_back(Beam::next(beam.position, Direction::Left)),
                    Direction::Down => deq.push_back(Beam::next(beam.position, Direction::Right)),
                    Direction::Left => deq.push_back(Beam::next(beam.position, Direction::Up)),
                    Direction::Right => deq.push_back(Beam::next(beam.position, Direction::Down)),
                }
            }

            _ => panic!("Unrecognized tile {tile} under beam {beam:?}"),
        }

        if tile != 'O' {
            visited.insert(beam);
        }
    }

    visited.iter().map(|x| x.position).collect::<HashSet<Position>>().len() as u64
}

fn parse_grid_with_boundary(input: &str) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = input.lines().map(|s| {
        let mut v: Vec<char> = s.chars().collect();
        v.push('O');
        v.insert(0, 'O');

        v
    }).collect();

    grid.push(vec!['O'; grid[0].len()]);
    grid.insert(0, vec!['O'; grid[0].len()]);

    grid
}

fn solve_part_1(input: &str) -> u64 {
    let grid = parse_grid_with_boundary(input);

    let start = Beam::new((1, 1), Direction::Right);

    trace_light_beam(start, &grid)
}

fn solve_part_2(input: &str) -> u64 {
    let grid = parse_grid_with_boundary(input);

    let mut highest: u64 = 0;

    // left edge
    for r in 1..grid.len() - 1 {
        let start = Beam::new((r, 1), Direction::Right);
        highest = highest.max(trace_light_beam(start, &grid));
    }

    // top edge
    for c in 1..grid[0].len() - 1 {
        let start = Beam::new((1, c), Direction::Down);
        highest = highest.max(trace_light_beam(start, &grid));
    }

    // right edge
    for r in 1..grid.len() - 1 {
        let start = Beam::new((r, grid[0].len() - 2), Direction::Left);
        highest = highest.max(trace_light_beam(start, &grid));
    }

    // bottom edge
    for c in 1..grid[0].len() - 1 {
        let start = Beam::new((grid.len() - 2, c), Direction::Up);
        highest = highest.max(trace_light_beam(start, &grid));
    }

    highest
}

pub fn answers() {
    let input = include_str!("input.txt").trim();
    // let input = include_str!("test.txt").trim();

    // println!("[P1 :: INFO] Answer: {}", solve_part_1(input));
    println!("[P2 :: INFO] Answer: {}", solve_part_2(input));
}
