fn main() {
    let m = 1000;
    for a in 1..m {
        for b in a..m {
            let c = m - (a + b);
            if a*a + b*b == c*c {
                print!("{}", a*b*c);
            }
        }
    }
}
