fn num_divisors(n: u64) -> u64 {
    let mut count = 1;
    let mut num = n;
    let mut divisor = 2;

    while divisor * divisor <= num {
        let mut current_count = 0;
        while num % divisor == 0 {
            num /= divisor;
            current_count += 1;
        }
        count *= current_count + 1;
        divisor += 1;
    }

    if num > 1 {
        count *= 2;
    }

    count
}

fn main() {
    let mut n = 1;

    loop {
        let triangle_number = n * (n + 1) / 2;
        if num_divisors(triangle_number) > 500 {
            println!("{}", triangle_number);
            break;
        }
        n += 1;
    }
}
