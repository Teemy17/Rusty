use std::str::FromStr;


fn main() {
    
    let args: Vec<String> = std::env::args().collect();

    let mut numbers: Vec<i32> = Vec::new();

    if args.len() < 2 {
        println!("Please provied a valid arguments.");
        return;
    }

    for arg in args[1..].iter() {
        if let Ok(num) = i32::from_str(arg) {
            numbers.push(num);
        }
    }

    if numbers.is_empty() {
        println!("Please input only numbers");
        return;
    }
    

    println!("Original numbers: {:?}", numbers);

    numbers.sort_by(|a, b| a.cmp(b)); 
    println!("Ascending order: {:?}", numbers);

    numbers.sort_by(|a, b| b.cmp(a)); 
    println!("Descending order: {:?}", numbers);
 

}
