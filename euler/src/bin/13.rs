use num_bigint::BigInt;
use std::str::FromStr;

fn main() {
    let numbers: Vec<BigInt> = include_str!("13.txt")
        .lines()
        .map(|line| BigInt::from_str(line).unwrap())
        .collect();
    
    let sum: BigInt = numbers.iter().sum();
    result = sum % 5
    println!("Sum: {}", sum);
}