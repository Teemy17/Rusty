use Rust_HW7::to_cartesian;
use Rust_HW7::PolarPoint;
use csv::{ReaderBuilder, Writer, Trim};
use std::fs::File;

fn load_polar_points<R: std::io::Read>(rdr: R) -> Vec<PolarPoint> {
    let mut reader = ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(false)
        .trim(Trim::All)
        .from_reader(rdr);

    let mut polar_points = Vec::new();

    for result in reader.records() {
        let record = result.unwrap();
        let r: f64 = record[0].parse().unwrap();
        let t: f64 = record[1].parse().unwrap();
        polar_points.push(PolarPoint { r, t });
    }

    return polar_points
}

fn main() {
    let input_path = "input.csv";
    let output_path = "output.csv";
    let file = File::open(input_path).unwrap();
    let polar_points = load_polar_points(file);
    let mut wtr = Writer::from_path(output_path).unwrap();
    let cartesian_points = to_cartesian(polar_points);

    for point in &cartesian_points {
        let x_format = format!("{:.2}", point.x);
        let y_format = format!("{:.2}", point.y);
        wtr.write_record(&[x_format, y_format]).unwrap();
    }

    wtr.flush().unwrap();

    println!("Conversion complete. Output saved to '{}'", output_path);
}
