use rand::Rng;

// fn gen_number<R: Rng>(rng: &mut R, x: f64) -> f64 {
//     rng.gen_range(-x ..= x)
// }

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
        let value = rng.gen_range(-10.0..=10.0);
        if value >= -1.0 && value <= 1.0 {
            count += 1.
        }
    }
    
    let p = (count as f64 / n as f64) ;

    println!("Generated {} values.", n);
    println!("Probability P: {:.6}", p);

}