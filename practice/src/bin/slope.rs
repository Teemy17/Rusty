fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 5 {
        panic!("Invalid input")
    }
    
    let x1: f32 = args[1].parse().unwrap();
    let x2: f32 = args[2].parse().unwrap();
    let y1: f32 = args[3].parse().unwrap();
    let y2: f32 = args[4].parse().unwrap();

    println!("The slope is {}", slope(x1, x2, y1, y2))

}

fn slope(x1: f32, x2: f32, y1: f32, y2: f32) -> f32 {
    return (y2 - y1) / (x2 - x1)
}