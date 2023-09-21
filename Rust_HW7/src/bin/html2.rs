use Rust_HW7::to_cartesian;
use Rust_HW7::PolarPoint;
use csv::{ReaderBuilder, Writer, Trim};
use std::fs::File;
use std::io::Write;

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
    let file = File::open(input_path).unwrap();
    let cartesian_points = load_polar_points(file);
    let polar_points = to_cartesian(cartesian_points);

    let mut html_table = String::from("<table>\n");
    html_table.push_str("<tr><th>r</th><th>t</th></tr>\n");

    for point in &polar_points {
        html_table.push_str("<tr>");
        html_table.push_str(&format!("<td>{:.2}</td>", point.x));
        html_table.push_str(&format!("<td>{:.2}</td>", point.y));
        html_table.push_str("</tr>\n");
    }

    html_table.push_str("</table>");

    let mut output_file = File::create("output.html").unwrap();
    output_file.write_all(html_table.as_bytes()).unwrap();

    println!("Conversion complete. Output saved as 'output.html'."); 
}