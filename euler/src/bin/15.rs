fn factorial_range(start: u128, end: u128) -> u128 {
    (start..=end).fold(1, |acc, x| acc * x)
}

fn calculate_grid_paths(grid_size: u128) -> u128 {

    let n = grid_size;
    let double_n = 2 * n;
    
    let numerator = factorial_range(n + 1, double_n);
    let denominator = factorial_range(1, n);

    numerator / denominator
}

fn main() {
    let grid_size = 20;
    let paths = calculate_grid_paths(grid_size);
    println!("{}", paths);
}
