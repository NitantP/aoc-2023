fn print_grid<T: std::fmt::Display>(grid: &Vec<Vec<T>>) {
    for row in grid {
        for x in row {
            print!("{x}");
        }
        println!();
    }
    println!();
}
