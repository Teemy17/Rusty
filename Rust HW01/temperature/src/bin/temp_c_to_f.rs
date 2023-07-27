fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        panic!("Invalid input");
    }

    let c_arg = &args[1];

    let c: f32 = c_arg.parse().unwrap();

    println!("{}°C is {}°F", c, cal_tempc(c));
}

fn cal_tempc(c: f32) -> f32 {
    return (9./5.)*c + 32.;
}