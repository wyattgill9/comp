fn num_to_words(n: u32) -> String {
    let below_20 = [
        "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", 
        "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", 
        "seventeen", "eighteen", "nineteen",
    ];
    
    let tens = [
        "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];

    if n == 0 {
        "zero".to_string()
    } else if n < 20 {
        below_20[n as usize].to_string()
    } else if n < 100 {
        let tens_part = tens[(n / 10) as usize];
        let units_part = below_20[(n % 10) as usize];
        if n % 10 == 0 {
            tens_part.to_string()
        } else {
            format!("{}-{}", tens_part, units_part)
        }
    } else if n < 1000 {
        let hundreds_part = below_20[(n / 100) as usize];
        let remainder = n % 100;
        if remainder == 0 {
            format!("{} hundred", hundreds_part)
        } else {
            format!("{} hundred and {}", hundreds_part, num_to_words(remainder))
        }
    } else if n == 1000 {
        "one thousand".to_string()
    } else {
        unreachable!()
    }
}


fn main(){

    let mut sum: u128 = 0;

    for i in 1..=1000 {
        let yes: String = num_to_words(i)
        .replace(" ", "")
        .replace("-", "");
        println!("{}", yes);
        sum += yes.len() as u128;

    }


    println!("{}", sum);
}