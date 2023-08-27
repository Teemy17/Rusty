fn cal_temp(f: f32) -> f32 {
    return (5. / 9.)*(f as f32 - 32.)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("No fahr provided")
    }

    let f: f32  = args[1].parse().unwrap();

    println!("Celcius: {}", cal_temp(f))
}

fn temp_loop(v: &[f32]) -> f32 {
    for n in 0..v.len() {
        v[n] = ((5.0 / 9.0) * (v[n] as f32 - 32.0)) as f32;
    }
    return v.to_vec()
}