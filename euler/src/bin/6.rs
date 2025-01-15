fn main() {
    let mut sum_of_squares = 0;
    let mut sum = 0;
    
    for i in 1..=100 {
        sum_of_squares += i * i;
        sum += i;
    }
    
    let square_of_sum = sum * sum;
    println!("{}", square_of_sum - sum_of_squares);
}