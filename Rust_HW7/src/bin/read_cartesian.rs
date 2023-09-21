use Rust_HW7::to_polar;
use Rust_HW7::Point;
use csv::{ReaderBuilder, Writer, Trim};
use std::fs::File;

fn load_cartesian_points<R: std::io::Read>(rdr: R) -> Vec<Point> {
    let mut reader = ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(false)
        .trim(Trim::All)
        .from_reader(rdr);

    let mut cartesian_points = Vec::new();

    for result in reader.records() {
        let record = result.unwrap();
        let x: f64 = record[0].parse().unwrap();
        let y: f64 = record[1].parse().unwrap();
        cartesian_points.push(Point { x, y });
    }

    return cartesian_points
}

fn main() {
    let input_path = "input.csv";
    let output_path = "output.csv";
    let file = File::open(input_path).unwrap();
    let cartesian_points = load_cartesian_points(file);
    let mut wtr = Writer::from_path(output_path).unwrap();
    let polar_point = to_polar(cartesian_points);

    for point in &polar_point {
        let r_format = format!("{:.2}", point.r);
        let t_format = format!("{:.2}", point.t);
        wtr.write_record(&[r_format, t_format]).unwrap();
    }

    wtr.flush().unwrap();

    println!("Conversion complete. Output saved to '{}'", output_path);
}
