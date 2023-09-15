use std::str::FromStr;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        panic!("No arguments provided.");
        
    }

    let mut numbers: Vec<i32> = Vec::new();

    for arg in &args[1..] {
        if let Ok(num) = i32::from_str(arg) {
            numbers.push(num);
        } else {
            eprintln!("Invalid number: {}", arg);
            std::process::exit(1);
        }
    }

    let mut ascending = numbers.clone();
    sort(&mut ascending, true);

    let mut descending = numbers.clone();
    sort(&mut descending, false);

    println!("Ascending order: {:?}", ascending);
    println!("Descending order: {:?}", descending);
}

fn sort(numbers: &mut Vec<i32>, ascending: bool) {
    let n = numbers.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if (ascending && numbers[j] > numbers[j + 1]) || (!ascending && numbers[j] < numbers[j + 1]) {
                numbers.swap(j, j + 1);
            }
        }
    }
}    
    
    


