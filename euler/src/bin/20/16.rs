use num_bigint::BigInt;

fn large_exp(n: BigInt, j: u32)-> BigInt {
    let mut result = BigInt::from(2);
    for _ in 1..j {
        result *= BigInt::from(2);
    }
    println!("{}", result);
    result
}

fn main(){
    let num: BigInt = BigInt::from(1000);
    let j: u32 = 1000;
    let result = large_exp(num, j);
    let sum_of_digits = result.to_string().chars().map(|c| c.to_digit(10).unwrap()).sum::<u32>();
    println!("{}", sum_of_digits);
}

