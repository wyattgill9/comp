fn main() {
    const LIMIT: usize = 2_000_000;
    let mut is_prime = vec![true; LIMIT];
    is_prime[0] = false;
    is_prime[1] = false;

    for num in 2..((LIMIT as f64).sqrt() as usize + 1) {
        if is_prime[num] {
            for multiple in (num * num..LIMIT).step_by(num) {
                is_prime[multiple] = false;
            }
        }
    }

    let sum: usize = is_prime
        .iter()
        .enumerate()
        .filter(|&(_, &prime)| prime)
        .map(|(num, _)| num)
        .sum();

    println!("{}", sum);
}
