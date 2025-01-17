fn main(){
    let result = count_sundays_on_first(1901, 2000);
    println!("{}", result);

}

fn is_leap_year(year: u32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

fn count_sundays_on_first(year: u32, end_year: u32) -> u32 {

    let days_in_month = vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut current_day = 2;
    let mut sunday_count = 0;

    for year in year..=end_year {
        for i in 0..days_in_month.len() {
            if current_day == 0 {
                sunday_count += 1
            }

            let mut days = days_in_month[i];
            if i == 1 && is_leap_year(year) {
                days += 1;
            }

            current_day = (current_day + days) % 7;
        }
    }
    sunday_count
}