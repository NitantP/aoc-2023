// https://adventofcode.com/2023/day/6

static INPUT1: [(f64, f64); 4] = [
    (40.0, 277.0),
    (82.0, 1338.0),
    (91.0, 1349.0),
    (66.0, 1063.0),
];

static INPUT2: (f64, f64) = (40829166.0, 277133813491063.0);

fn solve_quadratic_equation(a: f64, b: f64, c: f64) -> (f64, f64) {
    let discriminant = (b.powi(2) - (4.0 * a * c)).sqrt();

    ((-b + discriminant) / (2.0 * a), (-b - discriminant) / (2.0 * a))
}

fn solve_part_1() -> u64 {
    INPUT1.iter().map(|&race| {
        let (root1, root2) = solve_quadratic_equation(-1.0, race.0, -race.1);
        ((root2.ceil() - root1.floor()).abs() as u64) - 1
    }).product()
}

fn solve_part_2() -> u64 {
    let (root1, root2) = solve_quadratic_equation(-1.0, INPUT2.0, -INPUT2.1);
    ((root2.ceil() - root1.floor()).abs() as u64) - 1
}

pub fn answers() {
    println!("[P1 :: INFO] Answer: {}", solve_part_1());
    println!("[P2 :: INFO] Answer: {}", solve_part_2());
}
