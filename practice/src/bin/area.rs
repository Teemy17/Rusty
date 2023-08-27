const PI:f32 = 3.14;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        panic!("Invalid input")
    }
    
    let x:f32 = args[1].parse().unwrap();

    println!("The area is {}", cal_area(x))

}
fn cal_area(r:f32) -> f32 {
    return PI * r * r
}