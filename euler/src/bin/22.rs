fn main() {
    let names = include_str!("22.txt") 
        .split(',')
        .filter_map(|s| {
            let s = s.trim_matches('"'); 
            if !s.is_empty() {
                Some(s.to_string()) 
            } else {
                None
            }
        })
        .collect::<Vec<String>>(); 

    let output = names.iter().sort();
    println!("{:?}", output); 
}
