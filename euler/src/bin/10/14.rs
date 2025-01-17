fn main() {
    let (mut max_length, mut max_index) = (0, 0);

    for i in 2..1_000_000 {
        let length = collatz(i);
        if length > max_length {
            max_length = length;
            max_index = i;
        }
    }

    println!("{:?}", max_index);
}

fn collatz(mut n: u64) -> u64 {
    let mut length = 1;

    while n != 1 {
        n = if n % 2 == 0 {
            n / 2
        } else {
            3 * n + 1
        };
        length += 1;
    }

    length
}
