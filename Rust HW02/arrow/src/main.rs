fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        print!("")
    }
    else {
    
    let n: i32 = args[1].parse().unwrap();
    
    let m = n;

    for i in 0..n {
        for _j in 0..i + 1 {
            print!("*");
        }
        println!("");
    }

    for i in 0..n {
        for _j in 0..m - i - 1 {
            print!("*");
        }
        println!("");
    }

    }
}