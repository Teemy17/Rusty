const PI: f32 = 3.1416;
fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        panic!("No radius specified");
    }

    let x_arg = &args[1];

    let x: f32 = x_arg.parse().unwrap();

    println!("Area: {}", cal_area(x));
}

fn cal_area(r: f32) -> f32 {
    return PI * r * r;
}