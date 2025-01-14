fn main() {
    let mut n: u64 = 600851475143;
    let mut factor: u64 = 2;

    while factor * factor <= n {
        if n % factor == 0 {
            n /= factor;
        } else {
            factor += 1;
        }
    }

    println!("Largest prime factor: {}", n);
}
