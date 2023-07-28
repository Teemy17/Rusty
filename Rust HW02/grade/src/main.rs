fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        panic!("No score specified");
    }

    let x_arg = &args[1];

    let x: i32 = x_arg.parse().unwrap();

    if x > 100  {
        print!("Invalid score")
    } 
    else if x >= 95 && x <= 100 {
        print!("Excellent with A+")
    }
    else if x >= 81 && x < 95 {
        print!("A")
    }
    else if x >= 71 && x < 81 {
        print!("B")
    }
    else if x >= 61 && x < 71 {
        print!("C")
    }
    else if x >= 50 && x < 61 {
        print!("D")
    }
    else if x >= 0 && x < 50 {
        print!("Failed with F")
    }
    else if x < 0 {
        print!("Invalid score")
    }


    
    
}
