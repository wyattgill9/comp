fn main() {
    println!("{}", num_prime(10001));
}

fn num_prime(n: u64) -> u64 {
    let mut count = 0;
    let mut i = 2;
    while count < n {
        if is_prime(i) {
            count += 1;
        }
        i += 1;
    }
    return i - 1;
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}