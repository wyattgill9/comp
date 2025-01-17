
fn find_amicable_numbers(limit: u32) -> Vec<u32> {
    let mut amicable_numbers = Vec::new();
    let mut divisor_sums = vec![0; limit as usize];

    for i in 1..limit {
        for j in (i * 2..limit).step_by(i.try_into().unwrap()) {
            divisor_sums[j as usize] += i;
        }
    }

    for i in 2..limit {
        let sum_div_i = divisor_sums[i as usize];
        if sum_div_i < limit && sum_div_i != i && divisor_sums[sum_div_i as usize] == i {
            amicable_numbers.push(i);
        }
    }

    amicable_numbers
}

fn main() {

    let limit = 10000;
    let amicable_numbers = find_amicable_numbers(limit);

    let sum_of_amicable_numbers: u32 = amicable_numbers.iter().sum();

    println!("{}" , sum_of_amicable_numbers);
}
