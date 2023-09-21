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

    for i in 0..points.len() {
        for j in i + 1..points.len() {
            if points[i].0 > points[j].0 || (points[i].0 == points[j].0 && points[i].1 > points[j].1) {
                points.swap(i, j);
            }
        }
    }

    println!("Points sorted in ascending order: {:?}", points);
 

    for i in 0..points.len() {
        for j in i + 1..points.len() {
            if points[i].0 < points[j].0 || (points[i].0 == points[j].0 && points[i].1 < points[j].1) {
                points.swap(i, j);
            }
        }
    }

    println!("Points sorted in descending order: {:?}", points);
    
}
