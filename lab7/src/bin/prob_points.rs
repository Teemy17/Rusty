use rand::Rng;


fn main() {
    let mut rng = rand::thread_rng();
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("No argument specified");
    }

    let n_args = &args[1];

    let n: i32 = n_args.parse().unwrap();
    
    let mut count = 0.0;

    for i in 0..n {
        let x: f64 = rng.gen_range(-1.0 ..=1.0);
        let y: f64 = rng.gen_range(-1.0 ..=1.0);
        if (x.powf(2.0) + y.powf(2.0)).sqrt() <= 1.0 {
            count += 1.
        }
    }
   

    let p = (count as f64 / n as f64) ;

    println!("Probability values: {}", p);
    println!("Probability P*4: {:.6}", p*4.0);

}