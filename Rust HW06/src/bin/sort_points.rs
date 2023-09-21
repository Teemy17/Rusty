fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let mut points: Vec<(f64, f64)> = Vec::new();

    if args.len() < 2 {
        println!("Please provide a valid arguments.");
        return;  
    }

    for i in 0..args.len() / 2 {
        let x = args[i * 2].parse::<f64>();
        let y = args[i * 2 + 1].parse::<f64>();

        match (x, y) {
            (Ok(x), Ok(y)) => {
                points.push((x, y));
            }
            _ => {
                eprintln!("Error parsing coordinates.");
                return;
            }
        }
    }

    points.sort_by(|a, b| {
        if a.0 == b.0 {
            a.1.partial_cmp(&b.1).unwrap()
        } else {
            a.0.partial_cmp(&b.0).unwrap()
        }
    });

    println!("Points sorted in ascending order: {:?}", points);
   

    points.sort_by(|a, b| {
        if a.0 == b.0 {
            b.1.partial_cmp(&a.1).unwrap()
        } else {
            b.0.partial_cmp(&a.0).unwrap()
        }
    });

    println!("Points sorted in descending order: {:?}", points);
    
}
