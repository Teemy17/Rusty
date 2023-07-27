fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        panic!("No fahrenheit provided");
    }

    let f_arg = &args[1];

    let f: f32 = f_arg.parse().unwrap();

    println!("{}°F is {}°C", f, cal_tempf(f));
}

fn cal_tempf(f: f32) -> f32 {
    return (5./9.)*(f - 32.);
}